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

    // Fetch tags for each shortcut
    let mut result = Vec::new();
    for shortcut in shortcuts {
        let tags = sqlx::query_scalar::<_, String>(
            "SELECT t.name FROM tags t JOIN shortcut_tags st ON t.id = st.tag_id WHERE st.shortcut_id = ?",
        )
        .bind(shortcut.id)
        .fetch_all(&state.db)
        .await
        .unwrap_or_default();

        result.push(ShortcutWithTags { shortcut, tags });
    }

    Ok(Json(result))
}
