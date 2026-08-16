use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use super::AppState;
use crate::db::models::{Tag, Shortcut, ShortcutWithTags};

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<Tag>>, StatusCode> {
    let tags = sqlx::query_as::<_, Tag>("SELECT * FROM tags ORDER BY name")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tags))
}

pub async fn shortcuts_by_tag(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<Vec<ShortcutWithTags>>, StatusCode> {
    // Get all shortcuts that have this tag
    let shortcuts = sqlx::query_as::<_, Shortcut>(
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
    let result: Vec<ShortcutWithTags> = shortcuts
        .into_iter()
        .map(|shortcut| {
            let tags = tags_map.get(&shortcut.id).cloned().unwrap_or_default();
            ShortcutWithTags { shortcut, tags }
        })
        .collect();

    Ok(Json(result))
}
