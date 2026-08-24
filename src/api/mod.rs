pub mod middleware;
pub mod shortcuts;
pub mod auth;
pub mod users;
pub mod tags;
pub mod analytics;
pub mod auth_extractor;
pub mod settings;
pub mod api_keys;

use axum::{Router, routing::{get, post, put, delete}, Json};
use serde_json::{json, Value};
use sqlx::SqlitePool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::config::Config;

/// IP dedup cache: maps "shortcut_id:ip" -> last_view_timestamp
pub type DedupCache = Arc<RwLock<HashMap<String, i64>>>;

/// Cached shortcut data for fast redirects
#[derive(Clone)]
pub struct CachedShortcut {
    pub id: i64,
    pub link: String,
    pub creator_id: i64,
    pub title: String,
    pub description: String,
    pub og_title: String,
    pub og_description: String,
    pub og_image: String,
}

/// URL cache: maps shortcut name -> CachedShortcut
pub type UrlCache = Arc<RwLock<HashMap<String, CachedShortcut>>>;

#[derive(Clone)]
pub struct AppState {
    pub db: SqlitePool,
    pub config: Config,
    pub allow_registration: bool,
    pub view_dedup: DedupCache,
    pub url_cache: UrlCache,
}

impl AppState {
    pub fn new(db: SqlitePool, config: Config, allow_registration: bool) -> Self {
        Self {
            db,
            config,
            allow_registration,
            view_dedup: Arc::new(RwLock::new(HashMap::new())),
            url_cache: Arc::new(RwLock::new(HashMap::new())),
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
        .route("/api/v1/auth/change-password", put(auth::change_password))
        .route("/api/v1/auth/forgot-password", post(auth::forgot_password))
        .route("/api/v1/auth/reset-password", post(auth::reset_password))
        .route("/api/v1/auth/register-allowed", get(auth::register_allowed))
        // Shortcuts
        .route("/api/v1/shortcuts", get(shortcuts::list).post(shortcuts::create))
        .route("/api/v1/shortcuts/{id}", get(shortcuts::get).put(shortcuts::update).delete(shortcuts::delete))
        .route("/api/v1/shortcuts/by-name/{name}", get(shortcuts::get_by_name))
        .route("/api/v1/shortcuts/{id}/analytics", get(analytics::shortcut_analytics).delete(analytics::reset_analytics))
        // Tags
        .route("/api/v1/tags", get(tags::list).post(tags::create))
        .route("/api/v1/tags/{id}", put(tags::rename).delete(tags::delete))
        .route("/api/v1/tags/{name}/shortcuts", get(tags::shortcuts_by_tag))
        // Users (admin only)
        .route("/api/v1/users", get(users::list).post(users::create))
        .route("/api/v1/users/{id}", put(users::update).delete(users::delete))
        .route("/api/v1/users/{id}/password", put(users::reset_password))
        // Workspace settings (public read, admin write)
        .route("/api/v1/settings", get(settings::get_settings).put(settings::update_settings))
        // Logo upload
        .route("/api/v1/settings/logo", post(settings::upload_logo))
        // OG image upload
        .route("/api/v1/upload/og-image", post(shortcuts::upload_og_image))
        // API Keys
        .route("/api/v1/api-keys", get(api_keys::list_keys).post(api_keys::create_key))
        .route("/api/v1/api-keys/{id}", delete(api_keys::revoke_key).put(api_keys::toggle_key))
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
