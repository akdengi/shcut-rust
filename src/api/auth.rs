use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString,
    },
    Argon2,
};

use super::{middleware::create_token, AppState, auth_extractor::AuthClaims};
use crate::db::models::{ChangePassword, CreateUser, LoginRequest, User};

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

pub async fn change_password(
    State(state): State<AppState>,
    auth: AuthClaims,
    Json(input): Json<ChangePassword>,
) -> Result<Json<Value>, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    if input.new_password.len() < 6 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = match user {
        Some(u) => u,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let valid = verify_password(&input.current_password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !valid {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let password_hash = hash_password(&input.new_password)?;

    sqlx::query("UPDATE users SET password_hash = ?, updated_ts = ? WHERE id = ?")
        .bind(&password_hash)
        .bind(Utc::now().timestamp())
        .bind(user_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({ "message": "Password changed" })))
}

#[derive(Deserialize)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Deserialize)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub new_password: String,
}

pub async fn forgot_password(
    State(state): State<AppState>,
    Json(input): Json<ForgotPasswordRequest>,
) -> Result<Json<Value>, StatusCode> {
    if !state.allow_registration {
        return Err(StatusCode::FORBIDDEN);
    }

    if !state.config.smtp_configured() {
        return Err(StatusCode::NOT_IMPLEMENTED);
    }

    let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
        .bind(&input.email)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Always return success to prevent email enumeration
    let _ = user.as_ref();

    if let Some(user) = user {
        let token = generate_reset_token();
        let expires_at = Utc::now().timestamp() + 3600; // 1 hour

        sqlx::query(
            "INSERT INTO password_resets (user_id, token, expires_at, created_ts) VALUES (?, ?, ?, ?)",
        )
        .bind(user.id)
        .bind(&token)
        .bind(expires_at)
        .bind(Utc::now().timestamp())
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let _ = send_reset_email(&state, &user.email, &token).await;
    }

    Ok(Json(json!({ "message": "If an account exists, a reset link has been sent" })))
}

pub async fn reset_password(
    State(state): State<AppState>,
    Json(input): Json<ResetPasswordRequest>,
) -> Result<Json<Value>, StatusCode> {
    if input.new_password.len() < 6 {
        return Err(StatusCode::BAD_REQUEST);
    }

    let reset = sqlx::query_as::<_, PasswordReset>(
        "SELECT * FROM password_resets WHERE token = ? AND used = 0 AND expires_at > ?",
    )
    .bind(&input.token)
    .bind(Utc::now().timestamp())
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let reset = match reset {
        Some(r) => r,
        None => return Err(StatusCode::BAD_REQUEST),
    };

    let password_hash = hash_password(&input.new_password)?;

    sqlx::query("UPDATE users SET password_hash = ?, updated_ts = ? WHERE id = ?")
        .bind(&password_hash)
        .bind(Utc::now().timestamp())
        .bind(reset.user_id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("UPDATE password_resets SET used = 1 WHERE id = ?")
        .bind(reset.id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(json!({ "message": "Password reset successfully" })))
}

#[derive(Debug, sqlx::FromRow)]
#[allow(dead_code)]
struct PasswordReset {
    id: i64,
    user_id: i64,
    token: String,
    expires_at: i64,
    used: i64,
}

fn generate_reset_token() -> String {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    (0..32)
        .map(|_| {
            let idx = rng.gen_range(0..62);
            match idx {
                0..10 => (b'0' + idx) as char,
                10..36 => (b'a' + idx - 10) as char,
                36..62 => (b'A' + idx - 36) as char,
                _ => unreachable!(),
            }
        })
        .collect()
}

async fn send_reset_email(
    state: &AppState,
    to_email: &str,
    token: &str,
) -> Result<(), StatusCode> {
    use lettre::message::header::ContentType;
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{Message, SmtpTransport, Transport};

    let app_url = state
        .config
        .app_url
        .as_deref()
        .unwrap_or("http://localhost:5231");

    let reset_url = format!("{}/auth/reset-password?token={}", app_url, token);

    let from = state
        .config
        .smtp_from
        .as_deref()
        .unwrap_or("noreply@localhost");

    let email = Message::builder()
        .from(format!("ShCut <{}>", from).parse().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
        .to(to_email.parse().map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?)
        .subject("Password Reset")
        .header(ContentType::TEXT_PLAIN)
        .body(format!(
            "To reset your password, visit:\n\n{}\n\nThis link expires in 1 hour.",
            reset_url
        ))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let creds = Credentials::new(
        state.config.smtp_user.clone().unwrap_or_default(),
        state.config.smtp_password.clone().unwrap_or_default(),
    );

    let mailer = SmtpTransport::relay(state.config.smtp_host.as_deref().unwrap_or("localhost"))
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .credentials(creds)
        .port(state.config.smtp_port)
        .build();

    mailer.send(&email).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(())
}
