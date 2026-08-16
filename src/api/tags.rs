use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use tracing::error;

use super::AppState;
use super::auth_extractor::AuthClaims;
use crate::db::models::Tag;

#[derive(Debug, Deserialize)]
pub struct CreateTag {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTag {
    pub name: String,
}

#[derive(Debug, Serialize)]
pub struct TagWithCount {
    pub id: i64,
    pub name: String,
    pub shortcut_count: i64,
}

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<TagWithCount>>, StatusCode> {
    let tags = sqlx::query_as::<_, (i64, String, i64)>(
        "SELECT t.id, t.name, COUNT(st.shortcut_id) as shortcut_count 
         FROM tags t 
         LEFT JOIN shortcut_tags st ON t.id = st.tag_id 
         GROUP BY t.id, t.name 
         ORDER BY t.name"
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to fetch tags: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let result: Vec<TagWithCount> = tags
        .into_iter()
        .map(|(id, name, shortcut_count)| TagWithCount { id, name, shortcut_count })
        .collect();

    Ok(Json(result))
}

pub async fn create(
    State(state): State<AppState>,
    _auth: AuthClaims,
    Json(input): Json<CreateTag>,
) -> Result<Json<TagWithCount>, StatusCode> {
    let name = input.name.trim().to_lowercase();
    if name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if tag already exists
    let existing = sqlx::query_scalar::<_, i64>("SELECT id FROM tags WHERE name = ?")
        .bind(&name)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    sqlx::query("INSERT INTO tags (name) VALUES (?)")
        .bind(&name)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tag = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE name = ?")
        .bind(&name)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(TagWithCount {
        id: tag.id,
        name: tag.name,
        shortcut_count: 0,
    }))
}

pub async fn rename(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    _auth: AuthClaims,
    Json(input): Json<UpdateTag>,
) -> Result<Json<TagWithCount>, StatusCode> {
    let new_name = input.name.trim().to_lowercase();
    if new_name.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if new name already exists
    let existing = sqlx::query_scalar::<_, i64>("SELECT id FROM tags WHERE name = ? AND id != ?")
        .bind(&new_name)
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    sqlx::query("UPDATE tags SET name = ? WHERE id = ?")
        .bind(&new_name)
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let tag = sqlx::query_as::<_, Tag>("SELECT * FROM tags WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Get shortcut count
    let shortcut_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM shortcut_tags WHERE tag_id = ?"
    )
    .bind(id)
    .fetch_one(&state.db)
    .await
    .unwrap_or(0);

    Ok(Json(TagWithCount {
        id: tag.id,
        name: tag.name,
        shortcut_count,
    }))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    _auth: AuthClaims,
) -> Result<StatusCode, StatusCode> {
    // Check if tag exists
    let existing = sqlx::query_scalar::<_, i64>("SELECT id FROM tags WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Delete tag (cascade will remove shortcut_tags entries)
    sqlx::query("DELETE FROM tags WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn shortcuts_by_tag(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Vec<crate::db::models::ShortcutWithTags>>, StatusCode> {
    // Get all shortcuts that have this tag
    let shortcuts = sqlx::query_as::<_, crate::db::models::Shortcut>(
        "SELECT s.* FROM shortcuts s
         JOIN shortcut_tags st ON s.id = st.shortcut_id
         JOIN tags t ON st.tag_id = t.id
         WHERE t.name = ?
         ORDER BY s.created_ts DESC",
    )
    .bind(&name)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Fetch ALL tags for these shortcuts in ONE query
    let shortcut_ids: Vec<i64> = shortcuts.iter().map(|s| s.id).collect();
    let tags_map = if shortcut_ids.is_empty() {
        std::collections::HashMap::new()
    } else {
        let placeholders: String = shortcut_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let tags_query = format!(
            "SELECT st.shortcut_id, t.name FROM tags t JOIN shortcut_tags st ON t.id = st.tag_id WHERE st.shortcut_id IN ({})",
            placeholders
        );
        let mut tags_builder = sqlx::query_as::<_, (i64, String)>(&tags_query);
        for id in &shortcut_ids {
            tags_builder = tags_builder.bind(id);
        }
        let rows = tags_builder.fetch_all(&state.db).await.unwrap_or_default();
        let mut map: std::collections::HashMap<i64, Vec<String>> = std::collections::HashMap::new();
        for (shortcut_id, tag_name) in rows {
            map.entry(shortcut_id).or_default().push(tag_name);
        }
        map
    };

    // Build result
    let result: Vec<crate::db::models::ShortcutWithTags> = shortcuts
        .into_iter()
        .map(|shortcut| {
            let tags = tags_map.get(&shortcut.id).cloned().unwrap_or_default();
            crate::db::models::ShortcutWithTags { shortcut, tags }
        })
        .collect();

    Ok(Json(result))
}
