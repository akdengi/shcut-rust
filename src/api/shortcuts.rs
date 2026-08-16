use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::Deserialize;

use super::{AppState, auth_extractor::AuthClaims};
use crate::db::models::{CreateShortcut, PaginatedResponse, Shortcut, ShortcutWithTags, UpdateShortcut};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub tag: Option<String>,
    pub creator_id: Option<i64>,
    pub visibility: Option<String>,
    pub search: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> Result<Json<PaginatedResponse<ShortcutWithTags>>, StatusCode> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Build query
    let mut where_clauses = vec!["1=1".to_string()];
    let mut bind_values: Vec<String> = vec![];

    if let Some(ref tag) = params.tag {
        where_clauses.push("s.id IN (SELECT st.shortcut_id FROM shortcut_tags st JOIN tags t ON st.tag_id = t.id WHERE t.name = ?)".to_string());
        bind_values.push(tag.clone());
    }

    if let Some(creator_id) = params.creator_id {
        where_clauses.push("s.creator_id = ?".to_string());
        bind_values.push(creator_id.to_string());
    }

    if let Some(ref visibility) = params.visibility {
        where_clauses.push("s.visibility = ?".to_string());
        bind_values.push(visibility.clone());
    }

    if let Some(ref search) = params.search {
        where_clauses.push("(s.name LIKE ? OR s.title LIKE ? OR s.description LIKE ?)".to_string());
        let search_pattern = format!("%{}%", search);
        bind_values.push(search_pattern.clone());
        bind_values.push(search_pattern.clone());
        bind_values.push(search_pattern);
    }

    let where_str = where_clauses.join(" AND ");

    // Count total
    let count_query = format!("SELECT COUNT(*) FROM shortcuts s WHERE {}", where_str);
    let mut count_query_builder = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_query_builder = count_query_builder.bind(val);
    }
    let total = count_query_builder
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch shortcuts
    let query = format!(
        "SELECT s.* FROM shortcuts s WHERE {} ORDER BY s.created_ts DESC LIMIT ? OFFSET ?",
        where_str
    );
    let mut query_builder = sqlx::query_as::<_, Shortcut>(&query);
    for val in &bind_values {
        query_builder = query_builder.bind(val);
    }
    query_builder = query_builder.bind(per_page).bind(offset);

    let shortcuts = query_builder
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch tags for each shortcut
    let mut result = Vec::new();
    for shortcut in shortcuts {
        let tags = get_shortcut_tags(&state.db, shortcut.id)
            .await
            .unwrap_or_default();
        result.push(ShortcutWithTags { shortcut, tags });
    }

    let total_pages = (total as f64 / per_page as f64).ceil() as i64;

    Ok(Json(PaginatedResponse {
        items: result,
        total,
        page,
        per_page,
        total_pages,
    }))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ShortcutWithTags>, StatusCode> {
    let shortcut = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match shortcut {
        Some(s) => {
            let tags = get_shortcut_tags(&state.db, s.id)
                .await
                .unwrap_or_default();
            Ok(Json(ShortcutWithTags { shortcut: s, tags }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_by_name(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<ShortcutWithTags>, StatusCode> {
    let shortcut = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE name = ?")
        .bind(&name)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match shortcut {
        Some(s) => {
            let tags = get_shortcut_tags(&state.db, s.id)
                .await
                .unwrap_or_default();
            Ok(Json(ShortcutWithTags { shortcut: s, tags }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthClaims,
    Json(input): Json<CreateShortcut>,
) -> Result<Json<ShortcutWithTags>, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check if name is unique
    let existing = sqlx::query_scalar::<_, i64>("SELECT id FROM shortcuts WHERE name = ?")
        .bind(&input.name)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let now = Utc::now().timestamp();
    let visibility = input.visibility.unwrap_or_else(|| "workspace".to_string());

    // Insert shortcut
    let result = sqlx::query(
        "INSERT INTO shortcuts (creator_id, name, link, title, description, visibility, og_title, og_description, og_image, created_ts, updated_ts) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(&input.name)
    .bind(&input.link)
    .bind(input.title.as_deref().unwrap_or(""))
    .bind(input.description.as_deref().unwrap_or(""))
    .bind(&visibility)
    .bind(input.og_title.as_deref().unwrap_or(""))
    .bind(input.og_description.as_deref().unwrap_or(""))
    .bind(input.og_image.as_deref().unwrap_or(""))
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let shortcut_id = result.last_insert_rowid();

    // Add tags if provided
    if let Some(tags) = &input.tags {
        add_shortcut_tags(&state.db, shortcut_id, tags)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Fetch the created shortcut
    let shortcut = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE id = ?")
        .bind(shortcut_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tags = get_shortcut_tags(&state.db, shortcut_id)
        .await
        .unwrap_or_default();

    Ok(Json(ShortcutWithTags { shortcut, tags }))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth: AuthClaims,
    Json(input): Json<UpdateShortcut>,
) -> Result<Json<ShortcutWithTags>, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check ownership
    let existing = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let existing = match existing {
        Some(s) => s,
        None => return Err(StatusCode::NOT_FOUND),
    };

    if existing.creator_id != user_id {
        // Check if user is admin
        let is_admin = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if is_admin != "admin" {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    let now = Utc::now().timestamp();

    // Update shortcut
    sqlx::query(
        "UPDATE shortcuts SET name = ?, link = ?, title = ?, description = ?, visibility = ?, og_title = ?, og_description = ?, og_image = ?, updated_ts = ? WHERE id = ?",
    )
    .bind(input.name.as_deref().unwrap_or(&existing.name))
    .bind(input.link.as_deref().unwrap_or(&existing.link))
    .bind(input.title.as_deref().unwrap_or(&existing.title))
    .bind(input.description.as_deref().unwrap_or(&existing.description))
    .bind(input.visibility.as_deref().unwrap_or(&existing.visibility))
    .bind(input.og_title.as_deref().unwrap_or(&existing.og_title))
    .bind(input.og_description.as_deref().unwrap_or(&existing.og_description))
    .bind(input.og_image.as_deref().unwrap_or(&existing.og_image))
    .bind(now)
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update tags if provided
    if let Some(tags) = &input.tags {
        // Remove existing tags
        sqlx::query("DELETE FROM shortcut_tags WHERE shortcut_id = ?")
            .bind(id)
            .execute(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Add new tags
        add_shortcut_tags(&state.db, id, tags)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Fetch updated shortcut
    let shortcut = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tags = get_shortcut_tags(&state.db, id)
        .await
        .unwrap_or_default();

    Ok(Json(ShortcutWithTags { shortcut, tags }))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth: AuthClaims,
) -> Result<StatusCode, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check ownership
    let existing = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let existing = match existing {
        Some(s) => s,
        None => return Err(StatusCode::NOT_FOUND),
    };

    if existing.creator_id != user_id {
        // Check if user is admin
        let is_admin = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
            .bind(user_id)
            .fetch_one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if is_admin != "admin" {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    sqlx::query("DELETE FROM shortcuts WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn redirect(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<axum::response::Redirect, StatusCode> {
    let shortcut = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE name = ?")
        .bind(&name)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match shortcut {
        Some(s) => {
            // Increment view count
            sqlx::query("UPDATE shortcuts SET view_count = view_count + 1 WHERE id = ?")
                .bind(s.id)
                .execute(&state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

            Ok(axum::response::Redirect::temporary(&s.link))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

// Helper functions

async fn get_shortcut_tags(db: &sqlx::SqlitePool, shortcut_id: i64) -> Result<Vec<String>, sqlx::Error> {
    let tags = sqlx::query_scalar::<_, String>(
        "SELECT t.name FROM tags t JOIN shortcut_tags st ON t.id = st.tag_id WHERE st.shortcut_id = ?",
    )
    .bind(shortcut_id)
    .fetch_all(db)
    .await?;

    Ok(tags)
}

async fn add_shortcut_tags(db: &sqlx::SqlitePool, shortcut_id: i64, tags: &[String]) -> Result<(), sqlx::Error> {
    for tag_name in tags {
        // Insert or get tag
        sqlx::query("INSERT OR IGNORE INTO tags (name) VALUES (?)")
            .bind(tag_name)
            .execute(db)
            .await?;

        let tag_id = sqlx::query_scalar::<_, i64>("SELECT id FROM tags WHERE name = ?")
            .bind(tag_name)
            .fetch_one(db)
            .await?;

        // Link shortcut to tag
        sqlx::query("INSERT OR IGNORE INTO shortcut_tags (shortcut_id, tag_id) VALUES (?, ?)")
            .bind(shortcut_id)
            .bind(tag_id)
            .execute(db)
            .await?;
    }

    Ok(())
}
