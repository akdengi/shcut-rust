use axum::{
    extract::{Multipart, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::path::PathBuf;

use super::{AppState, auth_extractor::AuthClaims};

/// Get a boolean setting from the database (defaults to true if not set)
pub async fn get_bool_setting(db: &sqlx::SqlitePool, key: &str, default: bool) -> bool {
    let value = sqlx::query_scalar::<_, String>(
        "SELECT value FROM workspace_settings WHERE key = ?"
    )
    .bind(key)
    .fetch_optional(db)
    .await
    .ok()
    .flatten();

    match value.as_deref() {
        Some("true") => true,
        Some("false") => false,
        _ => default,
    }
}

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

pub async fn upload_logo(
    State(state): State<AppState>,
    _auth: AuthClaims,
    mut multipart: Multipart,
) -> Result<Json<Value>, StatusCode> {
    let upload_dir = PathBuf::from("/app/data/uploads");
    tokio::fs::create_dir_all(&upload_dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        if name != "file" {
            continue;
        }

        let file_name = field.file_name().unwrap_or("logo").to_string();
        let ext = file_name.rsplit('.').next().unwrap_or("png");
        
        // Validate extension
        let allowed = ["png", "jpg", "jpeg", "gif", "svg", "webp"];
        if !allowed.contains(&ext.to_lowercase().as_str()) {
            return Err(StatusCode::BAD_REQUEST);
        }

        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;
        
        // Limit file size to 2MB
        if data.len() > 2 * 1024 * 1024 {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }

        let saved_name = format!("logo.{}", ext);
        let file_path = upload_dir.join(&saved_name);
        
        tokio::fs::write(&file_path, &data)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let logo_url = format!("/uploads/{}", saved_name);

        // Save to database
        sqlx::query(
            "INSERT INTO workspace_settings (key, value) VALUES ('logo_url', ?) ON CONFLICT(key) DO UPDATE SET value = excluded.value"
        )
        .bind(&logo_url)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        return Ok(json!({ "logo_url": logo_url }).into());
    }

    Err(StatusCode::BAD_REQUEST)
}
