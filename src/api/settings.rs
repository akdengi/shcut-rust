use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};

use super::{AppState, auth_extractor::AuthClaims};

#[derive(Debug, Deserialize)]
pub struct UpdateSettings {
    pub company_name: Option<String>,
    pub logo_url: Option<String>,
}

pub async fn get_settings(
    State(state): State<AppState>,
) -> Result<Json<Value>, StatusCode> {
    let rows = sqlx::query_as::<_, (String, String)>("SELECT key, value FROM workspace_settings")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut settings = serde_json::Map::new();
    for (key, value) in rows {
        settings.insert(key, json!(value));
    }

    Ok(Json(json!(settings)))
}

pub async fn update_settings(
    State(state): State<AppState>,
    _auth: AuthClaims,
    Json(input): Json<UpdateSettings>,
) -> Result<Json<Value>, StatusCode> {
    if let Some(name) = &input.company_name {
        sqlx::query(
            "INSERT INTO workspace_settings (key, value) VALUES ('company_name', ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value"
        )
        .bind(name)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    if let Some(url) = &input.logo_url {
        sqlx::query(
            "INSERT INTO workspace_settings (key, value) VALUES ('logo_url', ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value"
        )
        .bind(url)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    get_settings(State(state)).await
}
