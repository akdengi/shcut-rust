use axum::{
    extract::{Request, State},
    http::{header::AUTHORIZATION, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation, Algorithm};

use super::AppState;
use crate::db::models::Claims;

/// JWT token expiration in seconds (7 days)
const TOKEN_EXPIRATION: usize = 7 * 24 * 60 * 60;

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
    if is_public_route(&path) {
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

    // Decode and validate token
    let claims = match decode_token(token, &state.config.jwt_secret) {
        Ok(claims) => claims,
        Err(_) => return Err(StatusCode::UNAUTHORIZED),
    };

    // Store user info in request extensions
    request
        .extensions_mut()
        .insert(claims);

    Ok(next.run(request).await)
}

/// Check if a route is public (doesn't require authentication)
fn is_public_route(path: &str) -> bool {
    // Health check
    if path == "/healthz" {
        return true;
    }

    // Auth routes (login, register, register-allowed are public; me is NOT)
    if path == "/api/v1/auth/login"
        || path == "/api/v1/auth/register"
        || path == "/api/v1/auth/register-allowed"
    {
        return true;
    }

    // Workspace settings (public read)
    if path == "/api/v1/settings" {
        return true;
    }

    // Public shortcut access
    if path.starts_with("/s/") {
        return true;
    }

    // Public shortcut/collection get (by name)
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

    false
}
