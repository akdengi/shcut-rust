pub mod middleware;
pub mod shortcuts;
pub mod collections;
pub mod auth;
pub mod users;
pub mod tags;
pub mod analytics;
pub mod auth_extractor;
pub mod settings;

use axum::{Router, routing::{get, post, put}, Json};
use serde_json::{json, Value};
use sqlx::SqlitePool;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    pub allow_registration: bool,
}

impl AppState {
    pub fn new(db: SqlitePool, config: Config, allow_registration: bool) -> Self {
        Self {
            db,
            config,
            allow_registration,
        }
    }
}

pub fn routes() -> Router<AppState> {
    let api_routes = Router::new()
        // Health
        .route("/healthz", get(health_check))
        // Auth
        .route("/api/v1/auth/register", post(auth::register))
        .route("/api/v1/auth/login", post(auth::login))
        .route("/api/v1/auth/me", get(auth::me))
        .route("/api/v1/auth/register-allowed", get(auth::register_allowed))
        // Shortcuts
        .route("/api/v1/shortcuts", get(shortcuts::list).post(shortcuts::create))
        .route("/api/v1/shortcuts/{id}", get(shortcuts::get).put(shortcuts::update).delete(shortcuts::delete))
        .route("/api/v1/shortcuts/by-name/{name}", get(shortcuts::get_by_name))
        .route("/api/v1/shortcuts/{id}/analytics", get(analytics::shortcut_analytics))
        // Collections
        .route("/api/v1/collections", get(collections::list).post(collections::create))
        .route("/api/v1/collections/{id}", get(collections::get).put(collections::update).delete(collections::delete))
        // Tags
        .route("/api/v1/tags", get(tags::list))
        // Users (admin only)
        .route("/api/v1/users", get(users::list))
        .route("/api/v1/users/{id}", put(users::update))
        // Workspace settings (public read, admin write)
        .route("/api/v1/settings", get(settings::get_settings).put(settings::update_settings))
        // Redirect (public)
        .route("/s/{name}", get(shortcuts::redirect));

    api_routes
}

async fn health_check() -> Json<Value> {
    Json(json!({
        "status": "ok",
        "service": "shcut-rust"
    }))
}
