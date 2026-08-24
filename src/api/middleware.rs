use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, Method, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};
use sha2::{Sha256, Digest};

use super::AppState;
use crate::db::models::Claims;

/// JWT token expiration in seconds (7 days)
const TOKEN_EXPIRATION: usize = 7 * 24 * 60 * 60;

/// Hash an API key using SHA-256
fn hash_api_key(key: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(key.as_bytes());
    format!("{:x}", hasher.finalize())
}

/// Create a new JWT token for a user
pub fn create_token(user_id: i64, email: &str, secret: &str) -> String {
    let now = Utc::now().timestamp() as usize;
    let claims = Claims {
        sub: user_id.to_string(),
        name: email.to_string(),
        exp: now + TOKEN_EXPIRATION,
        iat: now,
        iss: "shcut".to_string(),
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .expect("Failed to encode JWT")
}

/// Decode and validate a JWT token
pub fn decode_token(token: &str, secret: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::new(Algorithm::HS256),
    )
    .map(|data| data.claims)
}

/// Auth middleware that extracts and validates JWT from Authorization header
pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    // Skip auth for public routes
    let path = request.uri().path().to_string();
    let method = request.method().clone();
    if is_public_route(&path, &method) {
        return Ok(next.run(request).await);
    }

    // Extract token from Authorization header
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .and_then(|value| value.to_str().ok());

    let token = match auth_header {
        Some(header) if header.starts_with("Bearer ") => &header[7..],
        _ => return Err(StatusCode::UNAUTHORIZED),
    };

    // Decode and validate token — API key (shcut_*) or JWT
    let claims = if token.starts_with("shcut_") {
        // API key authentication
        let key_hash = hash_api_key(token);

        let api_key = sqlx::query_as::<_, crate::db::models::ApiKey>(
            "SELECT * FROM api_keys WHERE key_hash = ? AND is_active = 1"
        )
        .bind(&key_hash)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        match api_key {
            Some(key) => {
                if let Some(expires_at) = key.expires_at {
                    if Utc::now().timestamp() > expires_at {
                        return Err(StatusCode::UNAUTHORIZED);
                    }
                }

                let now = Utc::now().timestamp();
                let _ = sqlx::query("UPDATE api_keys SET last_used_ts = ? WHERE id = ?")
                    .bind(now)
                    .bind(key.id)
                    .execute(&state.db)
                    .await;

                let user = sqlx::query_as::<_, crate::db::models::User>(
                    "SELECT * FROM users WHERE id = ?"
                )
                .bind(key.user_id)
                .fetch_optional(&state.db)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                match user {
                    Some(user) => Claims {
                        sub: user.id.to_string(),
                        name: user.email,
                        exp: (Utc::now().timestamp() + 3600) as usize,
                        iat: Utc::now().timestamp() as usize,
                        iss: "shcut-apikey".to_string(),
                    },
                    None => return Err(StatusCode::UNAUTHORIZED),
                }
            }
            None => return Err(StatusCode::UNAUTHORIZED),
        }
    } else {
        // JWT authentication
        match decode_token(token, &state.config.jwt_secret) {
            Ok(claims) => claims,
            Err(_) => return Err(StatusCode::UNAUTHORIZED),
        }
    };

    // Store user info in request extensions
    request
        .extensions_mut()
        .insert(claims);

    Ok(next.run(request).await)
}

/// Check if a route is public (doesn't require authentication)
fn is_public_route(path: &str, method: &Method) -> bool {
    // Health check
    if path == "/healthz" {
        return true;
    }

    // Auth routes (login, register, register-allowed are public; me is NOT)
    if path == "/api/v1/auth/login"
        || path == "/api/v1/auth/register"
        || path == "/api/v1/auth/register-allowed"
        || path == "/api/v1/auth/forgot-password"
        || path == "/api/v1/auth/reset-password"
    {
        return true;
    }

    // Workspace settings (public read only)
    if path == "/api/v1/settings" && method == Method::GET {
        return true;
    }

    // Tags (GET is public, POST/PUT/DELETE require auth)
    if path == "/api/v1/tags" && method == Method::GET {
        return true;
    }
    if path.starts_with("/api/v1/tags/") && path.ends_with("/shortcuts") && method == Method::GET {
        return true;
    }

    // Public shortcut access
    if path.starts_with("/s/") {
        return true;
    }

    // Uploaded files (logos, etc.)
    if path.starts_with("/uploads/") {
        return true;
    }

    // Public shortcut get (by name)
    if path.contains("/by-name/") {
        return true;
    }

    // Static files (frontend assets)
    if path.starts_with("/_nuxt/") {
        return true;
    }
    if path.starts_with("/favicon") {
        return true;
    }
    if path.starts_with("/assets/") {
        return true;
    }

    // Root path (serves index.html)
    if path == "/" {
        return true;
    }

    // Static file extensions (CSS, JS, images, fonts, etc.)
    if path.ends_with(".html")
        || path.ends_with(".css")
        || path.ends_with(".js")
        || path.ends_with(".mjs")
        || path.ends_with(".json")
        || path.ends_with(".png")
        || path.ends_with(".jpg")
        || path.ends_with(".jpeg")
        || path.ends_with(".gif")
        || path.ends_with(".svg")
        || path.ends_with(".ico")
        || path.ends_with(".woff")
        || path.ends_with(".woff2")
        || path.ends_with(".ttf")
        || path.ends_with(".eot")
        || path.ends_with(".map")
    {
        return true;
    }

    // SPA routes (not API) — serve index.html
    if !path.starts_with("/api/") {
        return true;
    }

    false
}
