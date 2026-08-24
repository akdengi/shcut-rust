use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;
use rand::Rng;
use sha2::{Sha256, Digest};
use tracing::error;

use super::AppState;
use super::auth_extractor::AuthClaims;
use crate::db::models::{ApiKey, ApiKeyResponse, ApiKeyInfo, CreateApiKey};

/// Hash a raw API key using SHA-256
fn hash_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Generate a random API key: "shcut_" + 96 hex chars (48 random bytes)
fn generate_api_key() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..48).map(|_| rng.gen()).collect();
    let hex: String = bytes.iter().map(|b| format!("{:02x}", b)).collect();
    format!("shcut_{}", hex)
}

/// List API keys for the authenticated user (or all users for admin)
pub async fn list_keys(
    State(state): State<AppState>,
    auth: AuthClaims,
) -> Result<Json<Vec<ApiKeyInfo>>, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let keys = if role == "admin" {
        // Admin can see all keys
        sqlx::query_as::<_, ApiKey>(
            "SELECT * FROM api_keys ORDER BY created_ts DESC"
        )
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    } else {
        // Regular users can only see their own keys
        sqlx::query_as::<_, ApiKey>(
            "SELECT * FROM api_keys WHERE user_id = ? ORDER BY created_ts DESC"
        )
        .bind(user_id)
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    };

    let result: Vec<ApiKeyInfo> = keys
        .into_iter()
        .map(|k| ApiKeyInfo {
            id: k.id,
            name: k.name,
            key_prefix: k.key_prefix,
            created_ts: k.created_ts,
            last_used_ts: k.last_used_ts,
            expires_at: k.expires_at,
            is_active: k.is_active != 0,
        })
        .collect();

    Ok(Json(result))
}

/// Create a new API key for the authenticated user (admin can create for any user)
pub async fn create_key(
    State(state): State<AppState>,
    auth: AuthClaims,
    Json(input): Json<CreateApiKey>,
) -> Result<Json<ApiKeyResponse>, StatusCode> {
    let caller_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
        .bind(caller_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // view role cannot create API keys
    if role == "view" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Admin can create keys for other users; regular users only for themselves
    let target_user_id = if role == "admin" {
        input.user_id.unwrap_or(caller_id)
    } else {
        caller_id
    };

    let now = Utc::now().timestamp();
    let raw_key = generate_api_key();
    let key_hash = hash_key(&raw_key);
    // Store first 12 chars as prefix (shcut_ + 6 hex chars)
    let key_prefix = raw_key[..12].to_string();

    let expires_at = input.expires_in_days.map(|days| now + days * 86400);

    let result = sqlx::query(
        "INSERT INTO api_keys (user_id, name, key_hash, key_prefix, created_ts, expires_at) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(target_user_id)
    .bind(&input.name)
    .bind(&key_hash)
    .bind(&key_prefix)
    .bind(now)
    .bind(expires_at)
    .execute(&state.db)
    .await
    .map_err(|e| {
        error!("Failed to create API key: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let key_id = result.last_insert_rowid();

    Ok(Json(ApiKeyResponse {
        id: key_id,
        name: input.name,
        key: Some(raw_key), // Only returned on creation
        key_prefix,
        created_ts: now,
        expires_at,
    }))
}

/// Revoke (delete) an API key
pub async fn revoke_key(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth: AuthClaims,
) -> Result<StatusCode, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check ownership (unless admin)
    if role != "admin" {
        let owner = sqlx::query_scalar::<_, i64>("SELECT user_id FROM api_keys WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        match owner {
            Some(owner_id) if owner_id == user_id => {}
            _ => return Err(StatusCode::NOT_FOUND),
        }
    }

    let result = sqlx::query("DELETE FROM api_keys WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}

/// Toggle API key active/inactive status
pub async fn toggle_key(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth: AuthClaims,
) -> Result<Json<ApiKeyInfo>, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check ownership (unless admin)
    if role != "admin" {
        let owner = sqlx::query_scalar::<_, i64>("SELECT user_id FROM api_keys WHERE id = ?")
            .bind(id)
            .fetch_optional(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        match owner {
            Some(owner_id) if owner_id == user_id => {}
            _ => return Err(StatusCode::NOT_FOUND),
        }
    }

    // Toggle the is_active field
    let result = sqlx::query(
        "UPDATE api_keys SET is_active = CASE WHEN is_active = 1 THEN 0 ELSE 1 END WHERE id = ?"
    )
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Fetch updated key
    let key = sqlx::query_as::<_, ApiKey>("SELECT * FROM api_keys WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ApiKeyInfo {
        id: key.id,
        name: key.name,
        key_prefix: key.key_prefix,
        created_ts: key.created_ts,
        last_used_ts: key.last_used_ts,
        expires_at: key.expires_at,
        is_active: key.is_active != 0,
    }))
}
