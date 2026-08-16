use axum::{
    extract::State,
    http::StatusCode,
    Json,
};

use super::AppState;
use crate::db::models::Tag;

pub async fn list(
    State(state): State<AppState>,
) -> Result<Json<Vec<Tag>>, StatusCode> {
    let tags = sqlx::query_as::<_, Tag>("SELECT * FROM tags ORDER BY name")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(tags))
}
