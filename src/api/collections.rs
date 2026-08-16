use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::Deserialize;

use super::{AppState, auth_extractor::AuthClaims};
use crate::db::models::{Collection, CreateCollection, UpdateCollection};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub creator_id: Option<i64>,
    pub visibility: Option<String>,
    pub search: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct CollectionWithShortcuts {
    #[serde(flatten)]
    pub collection: Collection,
    pub shortcut_ids: Vec<i64>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> Result<Json<Vec<CollectionWithShortcuts>>, StatusCode> {
    let _page = params.page.unwrap_or(1).max(1);
    let _per_page = params.per_page.unwrap_or(20).min(100);

    let mut where_clauses = vec!["1=1".to_string()];
    let mut bind_values: Vec<String> = vec![];

    if let Some(creator_id) = params.creator_id {
        where_clauses.push("c.creator_id = ?".to_string());
        bind_values.push(creator_id.to_string());
    }

    if let Some(ref visibility) = params.visibility {
        where_clauses.push("c.visibility = ?".to_string());
        bind_values.push(visibility.clone());
    }

    if let Some(ref search) = params.search {
        where_clauses.push("(c.name LIKE ? OR c.title LIKE ? OR c.description LIKE ?)".to_string());
        let search_pattern = format!("%{}%", search);
        bind_values.push(search_pattern.clone());
        bind_values.push(search_pattern.clone());
        bind_values.push(search_pattern);
    }

    let where_str = where_clauses.join(" AND ");

    let query = format!(
        "SELECT c.* FROM collections c WHERE {} ORDER BY c.created_ts DESC",
        where_str
    );
    let mut query_builder = sqlx::query_as::<_, Collection>(&query);
    for val in &bind_values {
        query_builder = query_builder.bind(val);
    }

    let collections = query_builder
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut result = Vec::new();
    for collection in collections {
        let shortcut_ids = get_collection_shortcuts(&state.db, collection.id)
            .await
            .unwrap_or_default();
        result.push(CollectionWithShortcuts { collection, shortcut_ids });
    }

    Ok(Json(result))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<CollectionWithShortcuts>, StatusCode> {
    let collection = sqlx::query_as::<_, Collection>("SELECT * FROM collections WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match collection {
        Some(c) => {
            let shortcut_ids = get_collection_shortcuts(&state.db, c.id)
                .await
                .unwrap_or_default();
            Ok(Json(CollectionWithShortcuts { collection: c, shortcut_ids }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthClaims,
    Json(input): Json<CreateCollection>,
) -> Result<Json<CollectionWithShortcuts>, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check if name is unique
    let existing = sqlx::query_scalar::<_, i64>("SELECT id FROM collections WHERE name = ?")
        .bind(&input.name)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let now = Utc::now().timestamp();
    let visibility = input.visibility.unwrap_or_else(|| "workspace".to_string());

    let result = sqlx::query(
        "INSERT INTO collections (creator_id, name, title, description, visibility, created_ts, updated_ts) VALUES (?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(&input.name)
    .bind(input.title.as_deref().unwrap_or(""))
    .bind(input.description.as_deref().unwrap_or(""))
    .bind(&visibility)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let collection_id = result.last_insert_rowid();

    // Add shortcuts if provided
    if let Some(shortcut_ids) = &input.shortcut_ids {
        add_collection_shortcuts(&state.db, collection_id, shortcut_ids)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    let collection = sqlx::query_as::<_, Collection>("SELECT * FROM collections WHERE id = ?")
        .bind(collection_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let shortcut_ids = get_collection_shortcuts(&state.db, collection_id)
        .await
        .unwrap_or_default();

    Ok(Json(CollectionWithShortcuts { collection, shortcut_ids }))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth: AuthClaims,
    Json(input): Json<UpdateCollection>,
) -> Result<Json<CollectionWithShortcuts>, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check ownership
    let existing = sqlx::query_as::<_, Collection>("SELECT * FROM collections WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let existing = match existing {
        Some(c) => c,
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

    sqlx::query(
        "UPDATE collections SET name = ?, title = ?, description = ?, visibility = ?, updated_ts = ? WHERE id = ?",
    )
    .bind(input.name.as_deref().unwrap_or(&existing.name))
    .bind(input.title.as_deref().unwrap_or(&existing.title))
    .bind(input.description.as_deref().unwrap_or(&existing.description))
    .bind(input.visibility.as_deref().unwrap_or(&existing.visibility))
    .bind(now)
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update shortcuts if provided
    if let Some(shortcut_ids) = &input.shortcut_ids {
        sqlx::query("DELETE FROM collection_shortcuts WHERE collection_id = ?")
            .bind(id)
            .execute(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        add_collection_shortcuts(&state.db, id, shortcut_ids)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    let collection = sqlx::query_as::<_, Collection>("SELECT * FROM collections WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let shortcut_ids = get_collection_shortcuts(&state.db, id)
        .await
        .unwrap_or_default();

    Ok(Json(CollectionWithShortcuts { collection, shortcut_ids }))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth: AuthClaims,
) -> Result<StatusCode, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check ownership
    let existing = sqlx::query_as::<_, Collection>("SELECT * FROM collections WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let existing = match existing {
        Some(c) => c,
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

    sqlx::query("DELETE FROM collections WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

// Helper functions

async fn get_collection_shortcuts(db: &sqlx::SqlitePool, collection_id: i64) -> Result<Vec<i64>, sqlx::Error> {
    let ids = sqlx::query_scalar::<_, i64>(
        "SELECT shortcut_id FROM collection_shortcuts WHERE collection_id = ? ORDER BY position",
    )
    .bind(collection_id)
    .fetch_all(db)
    .await?;

    Ok(ids)
}

async fn add_collection_shortcuts(db: &sqlx::SqlitePool, collection_id: i64, shortcut_ids: &[i64]) -> Result<(), sqlx::Error> {
    for (position, shortcut_id) in shortcut_ids.iter().enumerate() {
        sqlx::query(
            "INSERT OR IGNORE INTO collection_shortcuts (collection_id, shortcut_id, position) VALUES (?, ?, ?)",
        )
        .bind(collection_id)
        .bind(shortcut_id)
        .bind(position as i64)
        .execute(db)
        .await?;
    }

    Ok(())
}
