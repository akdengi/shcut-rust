use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde_json::{json, Value};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

use super::{middleware::create_token, AppState, auth_extractor::AuthClaims};
use crate::db::models::{CreateUser, LoginRequest, User};

pub async fn register_allowed(
    State(state): State<AppState>,
) -> Json<Value> {
    Json(json!({
        "allowed": state.allow_registration
    }))
}

/// Hash password using Argon2id (OWASP recommended)
fn hash_password(password: &str) -> Result<String, StatusCode> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .to_string();
    Ok(password_hash)
}

/// Verify password against Argon2 hash
fn verify_password(password: &str, hash: &str) -> Result<bool, StatusCode> {
    let parsed_hash = PasswordHash::new(hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let argon2 = Argon2::default();
    Ok(argon2
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

pub async fn register(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<Json<Value>, StatusCode> {
    // Check if registration is allowed
    if !state.allow_registration {
        return Err(StatusCode::FORBIDDEN);
    }

    // Validate input
    if input.email.is_empty() || input.password.is_empty() {
        return Err(StatusCode::BAD_REQUEST);
    }

    if input.password.len() < 6 {
        return Err(StatusCode::BAD_REQUEST);
    }

    // Check if user already exists
    let existing = sqlx::query_scalar::<_, i64>("SELECT id FROM users WHERE email = ?")
        .bind(&input.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    // Hash password with Argon2
    let password_hash = hash_password(&input.password)?;

    let now = Utc::now().timestamp();
    let role = if sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)? == 0
    {
        "admin" // First user becomes admin
    } else {
        "view" // New users default to view-only
    };

    // Insert user
    let result = sqlx::query(
        "INSERT INTO users (email, nickname, password_hash, role, created_ts, updated_ts) VALUES (?, ?, ?, ?, ?, ?)",
    )
    .bind(&input.email)
    .bind(&input.nickname)
    .bind(&password_hash)
    .bind(role)
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = result.last_insert_rowid();

    // Fetch the created user
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Generate JWT
    let token = create_token(user.id, &user.email, &state.config.jwt_secret);

    Ok(Json(json!({
        "token": token,
        "user": user
    })))
}

pub async fn login(
    State(state): State<AppState>,
    Json(input): Json<LoginRequest>,
) -> Result<Json<Value>, StatusCode> {
    // Find user by email
    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&input.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = match user {
        Some(u) => u,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    // Verify password with Argon2
    let valid = verify_password(&input.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Generate JWT
    let token = create_token(user.id, &user.email, &state.config.jwt_secret);

    Ok(Json(json!({
        "token": token,
        "user": user
    })))
}

pub async fn me(
    State(state): State<AppState>,
    auth: AuthClaims,
) -> Result<Json<Value>, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match user {
        Some(u) => Ok(Json(json!(u))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
