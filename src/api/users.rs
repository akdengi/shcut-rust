use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use chrono::Utc;

use super::{AppState, auth_extractor::AuthClaims};
use crate::db::models::{User, UpdateUser};

pub async fn list(
    State(state): State<AppState>,
    auth: AuthClaims,
) -> Result<Json<Vec<User>>, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check if user is admin
    let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    let users = sqlx::query_as::<_, User>("SELECT * FROM users ORDER BY created_ts DESC")
        .fetch_all(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(users))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth: AuthClaims,
    Json(input): Json<UpdateUser>,
) -> Result<Json<User>, StatusCode> {
    let current_user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check ownership or admin
    if current_user_id != id {
        let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
            .bind(current_user_id)
            .fetch_one(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        if role != "admin" {
            return Err(StatusCode::FORBIDDEN);
        }
    }

    // Check if user exists
    let existing = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let existing = match existing {
        Some(u) => u,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let now = Utc::now().timestamp();

    sqlx::query(
        "UPDATE users SET nickname = ?, email = ?, updated_ts = ? WHERE id = ?",
    )
    .bind(input.nickname.as_deref().unwrap_or(&existing.nickname))
    .bind(input.email.as_deref().unwrap_or(&existing.email))
    .bind(now)
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(user))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth: AuthClaims,
) -> Result<StatusCode, StatusCode> {
    let current_user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Only admin can delete users
    let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
        .bind(current_user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Cannot delete yourself
    if current_user_id == id {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if user exists
    let existing = sqlx::query_scalar::<_, i64>("SELECT id FROM users WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    sqlx::query("DELETE FROM users WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}
