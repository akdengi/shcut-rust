use axum::{Router, middleware as axum_middleware};
use sqlx::sqlite::SqlitePoolOptions;
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use tower_http::services::{ServeDir, ServeFile};
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod config;
mod db;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "shcut_rust=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let cfg = config::Config::from_env();

    // === DATABASE SETUP ===
    let raw_path = std::env::var("DATABASE_URL").unwrap_or_else(|_| "/app/data/shcut.db".to_string());

    // Strip sqlite: prefix if present, we only need the file path
    let file_path = if raw_path.starts_with("sqlite:") {
        raw_path.trim_start_matches("sqlite:").trim_start_matches('/')
    } else {
        &raw_path
    };

    // Resolve to absolute path
    let abs_path = if std::path::Path::new(file_path).is_absolute() {
        std::path::PathBuf::from(file_path)
    } else {
        std::env::current_dir()
            .expect("Cannot get current dir")
            .join(file_path)
    };

    // Ensure parent directory exists
    let parent = abs_path.parent().unwrap_or(std::path::Path::new("."));
    std::fs::create_dir_all(parent).expect("Cannot create database directory");

    // Ensure uploads directory exists
    let uploads_dir = parent.join("uploads");
    std::fs::create_dir_all(&uploads_dir).expect("Cannot create uploads directory");
    tracing::info!("Uploads directory: {}", uploads_dir.display());

    // Debug: check what we can do
    tracing::info!("DB file path: {}", abs_path.display());
    tracing::info!("DB parent dir: {}", parent.display());
    tracing::info!("CWD: {:?}", std::env::current_dir());

    // Check directory exists and list contents
    match std::fs::read_dir(parent) {
        Ok(entries) => {
            for entry in entries.flatten() {
                tracing::info!("  dir entry: {} (is_file={})", entry.path().display(), entry.file_type().map(|ft| ft.is_file()).unwrap_or(false));
            }
        }
        Err(e) => tracing::error!("Cannot read parent dir: {}", e),
    }

    // Try to create a test file to verify write access
    let test_path = parent.join(".shcut_write_test");
    match std::fs::write(&test_path, b"test") {
        Ok(_) => {
            tracing::info!("Write test: OK");
            let _ = std::fs::remove_file(&test_path);
        }
        Err(e) => {
            tracing::error!("Write test FAILED: {}", e);
            tracing::error!("Cannot write to database directory! Check volume permissions.");
        }
    }

    // Build connection string — sqlx wants sqlite:///absolute/path
    let db_url = format!("sqlite://{}", abs_path.display());
    tracing::info!("Connection URL: {}", db_url);

    // Pre-create the database file (some SQLite builds can't create files)
    if !abs_path.exists() {
        match std::fs::File::create(&abs_path) {
            Ok(f) => {
                drop(f);
                tracing::info!("Pre-created database file: {}", abs_path.display());
            }
            Err(e) => tracing::error!("Cannot pre-create database file: {}", e),
        }
    }

    // Try multiple connection string formats
    let formats = [
        format!("sqlite://{}", abs_path.display()),
        format!("sqlite:{}", abs_path.display()),
        format!("sqlite:{}", abs_path.to_string_lossy()),
    ];

    let mut pool = None;
    for url in &formats {
        tracing::info!("Trying URL: {}", url);
        match SqlitePoolOptions::new()
            .max_connections(1)
            .connect(url)
            .await
        {
            Ok(p) => {
                tracing::info!("Connected with URL: {}", url);
                pool = Some(p);
                break;
            }
            Err(e) => tracing::error!("  Failed: {}", e),
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;
    }

    let pool = match pool {
        Some(p) => p,
        None => {
            tracing::error!("All connection attempts failed. Trying in-memory database...");
            SqlitePoolOptions::new()
                .max_connections(1)
                .connect("sqlite::memory:")
                .await
                .expect("Even in-memory failed!")
        }
    };

    // Run migrations
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    tracing::info!("Database ready at {}", abs_path.display());

    // Seed admin user from env vars if provided
    if let (Some(email), Some(password)) = (&cfg.admin_email, &cfg.admin_password) {
        let existing = sqlx::query_scalar::<_, i64>("SELECT id FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&pool)
            .await
            .ok()
            .flatten();

        if existing.is_none() {
            use argon2::{
                password_hash::{rand_core::OsRng, SaltString},
                PasswordHasher,
            };
            let salt = SaltString::generate(&mut OsRng);
            let password_hash = argon2::Argon2::default()
                .hash_password(password.as_bytes(), &salt)
                .expect("Failed to hash admin password")
                .to_string();

            let nickname = cfg.admin_nickname.as_deref().unwrap_or("admin");
            let now = chrono::Utc::now().timestamp();

            sqlx::query(
                "INSERT INTO users (email, nickname, password_hash, role, created_ts, updated_ts) VALUES (?, ?, ?, 'admin', ?, ?)"
            )
            .bind(email)
            .bind(nickname)
            .bind(&password_hash)
            .bind(now)
            .bind(now)
            .execute(&pool)
            .await
            .expect("Failed to seed admin user");

            tracing::info!("Admin user seeded: {}", email);
        }
    }

    // Build application state
    let state = api::AppState::new(pool.clone(), cfg.clone(), cfg.allow_registration);

    // Pre-warm URL cache
    {
        let shortcuts = sqlx::query_as::<_, crate::db::models::Shortcut>("SELECT * FROM shortcuts")
            .fetch_all(&pool)
            .await
            .unwrap_or_default();
        let mut cache = state.url_cache.write().await;
        for s in &shortcuts {
            cache.insert(s.name.clone(), api::CachedShortcut {
                id: s.id,
                link: s.link.clone(),
                creator_id: s.creator_id,
                og_title: s.og_title.clone(),
                og_description: s.og_description.clone(),
                og_image: s.og_image.clone(),
            });
        }
        tracing::info!("URL cache warmed with {} shortcuts", cache.len());
    }

    // CORS layer
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    // Static files directory (frontend build output)
    let static_dir = std::env::var("STATIC_DIR")
        .unwrap_or_else(|_| "/app/static".to_string());

    // Build router
    let app = Router::new()
        .merge(api::routes())
        // Serve static files (frontend) — falls back to index.html for SPA
        .fallback_service(
            ServeDir::new(&static_dir)
                .append_index_html_on_directories(true)
                .not_found_service(ServeFile::new(format!("{}/index.html", static_dir)))
        )
        .layer(axum_middleware::from_fn_with_state(
            state.clone(),
            api::middleware::auth_middleware,
        ))
        .layer(cors)
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = SocketAddr::new(cfg.host.parse().unwrap(), cfg.port);
    tracing::info!("Listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
