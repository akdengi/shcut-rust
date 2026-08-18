use axum::{
    extract::{ConnectInfo, Path, Query, State},
    http::{HeaderMap, StatusCode},
    Json,
};
use chrono::Utc;
use serde::Deserialize;
use std::net::SocketAddr;
use tracing::error;

use super::{AppState, auth_extractor::AuthClaims};
use crate::db::models::{CreateShortcut, PaginatedResponse, Shortcut, ShortcutWithTags, UpdateShortcut};

#[derive(Debug, Deserialize)]
pub struct ListQuery {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
    pub tag: Option<String>,
    pub creator_id: Option<i64>,
    pub visibility: Option<String>,
    pub search: Option<String>,
}

pub async fn list(
    State(state): State<AppState>,
    Query(params): Query<ListQuery>,
) -> Result<Json<PaginatedResponse<ShortcutWithTags>>, StatusCode> {
    let page = params.page.unwrap_or(1).max(1);
    let per_page = params.per_page.unwrap_or(20).min(100);
    let offset = (page - 1) * per_page;

    // Build query
    let mut where_clauses = vec!["1=1".to_string()];
    let mut bind_values: Vec<String> = vec![];

    if let Some(ref tag) = params.tag {
        where_clauses.push("s.id IN (SELECT st.shortcut_id FROM shortcut_tags st JOIN tags t ON st.tag_id = t.id WHERE t.name = ?)".to_string());
        bind_values.push(tag.clone());
    }

    if let Some(creator_id) = params.creator_id {
        where_clauses.push("s.creator_id = ?".to_string());
        bind_values.push(creator_id.to_string());
    }

    if let Some(ref visibility) = params.visibility {
        where_clauses.push("s.visibility = ?".to_string());
        bind_values.push(visibility.clone());
    }

    if let Some(ref search) = params.search {
        where_clauses.push("(s.name LIKE ? OR s.title LIKE ? OR s.description LIKE ?)".to_string());
        let search_pattern = format!("%{}%", search);
        bind_values.push(search_pattern.clone());
        bind_values.push(search_pattern.clone());
        bind_values.push(search_pattern);
    }

    let where_str = where_clauses.join(" AND ");

    // Count total
    let count_query = format!("SELECT COUNT(*) FROM shortcuts s WHERE {}", where_str);
    let mut count_query_builder = sqlx::query_scalar::<_, i64>(&count_query);
    for val in &bind_values {
        count_query_builder = count_query_builder.bind(val);
    }
    let total = count_query_builder
        .fetch_one(&state.db)
        .await
        .map_err(|e| {
            error!("Failed to count shortcuts: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Fetch shortcuts
    let query = format!(
        "SELECT s.* FROM shortcuts s WHERE {} ORDER BY s.created_ts DESC LIMIT ? OFFSET ?",
        where_str
    );
    let mut query_builder = sqlx::query_as::<_, Shortcut>(&query);
    for val in &bind_values {
        query_builder = query_builder.bind(val);
    }
    query_builder = query_builder.bind(per_page).bind(offset);

    let shortcuts = query_builder
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            error!("Failed to fetch shortcuts: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Fetch ALL tags for these shortcuts in ONE query
    let shortcut_ids: Vec<i64> = shortcuts.iter().map(|s| s.id).collect();
    let tags_map = if shortcut_ids.is_empty() {
        std::collections::HashMap::new()
    } else {
        let placeholders: String = shortcut_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let tags_query = format!(
            "SELECT st.shortcut_id, t.name FROM tags t JOIN shortcut_tags st ON t.id = st.tag_id WHERE st.shortcut_id IN ({})",
            placeholders
        );
        let mut tags_builder = sqlx::query_as::<_, (i64, String)>(&tags_query);
        for id in &shortcut_ids {
            tags_builder = tags_builder.bind(id);
        }
        let rows = tags_builder.fetch_all(&state.db).await.unwrap_or_default();
        let mut map: std::collections::HashMap<i64, Vec<String>> = std::collections::HashMap::new();
        for (shortcut_id, tag_name) in rows {
            map.entry(shortcut_id).or_default().push(tag_name);
        }
        map
    };

    // Build result
    let result: Vec<ShortcutWithTags> = shortcuts
        .into_iter()
        .map(|shortcut| {
            let tags = tags_map.get(&shortcut.id).cloned().unwrap_or_default();
            ShortcutWithTags { shortcut, tags }
        })
        .collect();

    let total_pages = (total as f64 / per_page as f64).ceil() as i64;

    Ok(Json(PaginatedResponse {
        items: result,
        total,
        page,
        per_page,
        total_pages,
    }))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ShortcutWithTags>, StatusCode> {
    let shortcut = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match shortcut {
        Some(s) => {
            let tags = get_shortcut_tags(&state.db, s.id)
                .await
                .unwrap_or_default();
            Ok(Json(ShortcutWithTags { shortcut: s, tags }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn get_by_name(
    State(state): State<AppState>,
    Path(name): Path<String>,
) -> Result<Json<ShortcutWithTags>, StatusCode> {
    let shortcut = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE name = ?")
        .bind(&name)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match shortcut {
        Some(s) => {
            let tags = get_shortcut_tags(&state.db, s.id)
                .await
                .unwrap_or_default();
            Ok(Json(ShortcutWithTags { shortcut: s, tags }))
        }
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create(
    State(state): State<AppState>,
    auth: AuthClaims,
    Json(input): Json<CreateShortcut>,
) -> Result<Json<ShortcutWithTags>, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check role - only admin and user can create
    let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if role == "view" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Check if name is unique
    let existing = sqlx::query_scalar::<_, i64>("SELECT id FROM shortcuts WHERE name = ?")
        .bind(&input.name)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_some() {
        return Err(StatusCode::CONFLICT);
    }

    let now = Utc::now().timestamp();
    let visibility = input.visibility.unwrap_or_else(|| "workspace".to_string());

    // Insert shortcut
    let result = sqlx::query(
        "INSERT INTO shortcuts (creator_id, name, link, title, description, visibility, og_title, og_description, og_image, created_ts, updated_ts) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(user_id)
    .bind(&input.name)
    .bind(&input.link)
    .bind(input.title.as_deref().unwrap_or(""))
    .bind(input.description.as_deref().unwrap_or(""))
    .bind(&visibility)
    .bind(input.og_title.as_deref().unwrap_or(""))
    .bind(input.og_description.as_deref().unwrap_or(""))
    .bind(input.og_image.as_deref().unwrap_or(""))
    .bind(now)
    .bind(now)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let shortcut_id = result.last_insert_rowid();

    // Add tags if provided
    if let Some(tags) = &input.tags {
        add_shortcut_tags(&state.db, shortcut_id, tags)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    // Fetch the created shortcut
    let shortcut = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE id = ?")
        .bind(shortcut_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Add to URL cache
    {
        let mut cache = state.url_cache.write().await;
        cache.insert(shortcut.name.clone(), (shortcut_id, shortcut.link.clone(), user_id));
    }

    let tags = get_shortcut_tags(&state.db, shortcut_id)
        .await
        .unwrap_or_default();

    Ok(Json(ShortcutWithTags { shortcut, tags }))
}

pub async fn update(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth: AuthClaims,
    Json(input): Json<UpdateShortcut>,
) -> Result<Json<ShortcutWithTags>, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check role
    let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if role == "view" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Check ownership
    let existing = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let existing = match existing {
        Some(s) => s,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // user role can only edit their own shortcuts
    if role == "user" && existing.creator_id != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    let now = Utc::now().timestamp();

    // Update shortcut
    sqlx::query(
        "UPDATE shortcuts SET name = ?, link = ?, title = ?, description = ?, visibility = ?, og_title = ?, og_description = ?, og_image = ?, updated_ts = ? WHERE id = ?",
    )
    .bind(input.name.as_deref().unwrap_or(&existing.name))
    .bind(input.link.as_deref().unwrap_or(&existing.link))
    .bind(input.title.as_deref().unwrap_or(&existing.title))
    .bind(input.description.as_deref().unwrap_or(&existing.description))
    .bind(input.visibility.as_deref().unwrap_or(&existing.visibility))
    .bind(input.og_title.as_deref().unwrap_or(&existing.og_title))
    .bind(input.og_description.as_deref().unwrap_or(&existing.og_description))
    .bind(input.og_image.as_deref().unwrap_or(&existing.og_image))
    .bind(now)
    .bind(id)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Update tags (always, even if empty - to allow removing all tags)
    if let Some(tags) = &input.tags {
        // Remove existing tags
        sqlx::query("DELETE FROM shortcut_tags WHERE shortcut_id = ?")
            .bind(id)
            .execute(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Add new tags (if any)
        if !tags.is_empty() {
            add_shortcut_tags(&state.db, id, tags)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
        }
    }

    // Fetch updated shortcut
    let shortcut = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE id = ?")
        .bind(id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Invalidate URL cache for old and new name
    {
        let mut cache = state.url_cache.write().await;
        if existing.name != shortcut.name {
            cache.remove(&existing.name);
        }
        cache.insert(shortcut.name.clone(), (id, shortcut.link.clone(), shortcut.creator_id));
    }

    let tags = get_shortcut_tags(&state.db, id)
        .await
        .unwrap_or_default();

    Ok(Json(ShortcutWithTags { shortcut, tags }))
}

pub async fn delete(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth: AuthClaims,
) -> Result<StatusCode, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Only admin can delete shortcuts
    let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Check if shortcut exists
    let existing = sqlx::query_scalar::<_, i64>("SELECT id FROM shortcuts WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if existing.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    sqlx::query("DELETE FROM shortcuts WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Invalidate URL cache
    {
        let mut cache = state.url_cache.write().await;
        cache.retain(|_, (cached_id, _, _)| *cached_id != id);
    }

    Ok(StatusCode::NO_CONTENT)
}

pub async fn upload_og_image(
    State(state): State<AppState>,
    _auth: AuthClaims,
    mut multipart: axum::extract::Multipart,
) -> Result<Json<serde_json::Value>, StatusCode> {
    let upload_dir = std::path::PathBuf::from("/app/data/uploads");
    tokio::fs::create_dir_all(&upload_dir)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    while let Some(field) = multipart.next_field().await.map_err(|_| StatusCode::BAD_REQUEST)? {
        let name = field.name().unwrap_or("").to_string();
        if name != "file" {
            continue;
        }

        let file_name = field.file_name().unwrap_or("og_image").to_string();
        let ext = file_name.rsplit('.').next().unwrap_or("png");

        // Validate extension
        let allowed = ["png", "jpg", "jpeg", "gif", "webp"];
        if !allowed.contains(&ext.to_lowercase().as_str()) {
            return Err(StatusCode::BAD_REQUEST);
        }

        let data = field.bytes().await.map_err(|_| StatusCode::BAD_REQUEST)?;

        // Limit file size to 2MB
        if data.len() > 2 * 1024 * 1024 {
            return Err(StatusCode::PAYLOAD_TOO_LARGE);
        }

        let saved_name = format!("og_{}.{}", chrono::Utc::now().timestamp_millis(), ext);
        let file_path = upload_dir.join(&saved_name);

        tokio::fs::write(&file_path, &data)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        let url = format!("/uploads/{}", saved_name);
        return Ok(serde_json::json!({ "url": url }).into());
    }

    Err(StatusCode::BAD_REQUEST)
}

pub async fn redirect(
    State(state): State<AppState>,
    Path(name): Path<String>,
    axum::extract::OriginalUri(uri): axum::extract::OriginalUri,
    headers: HeaderMap,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> Result<axum::response::Redirect, StatusCode> {
    // Try URL cache first for instant redirect
    let cached = {
        let cache = state.url_cache.read().await;
        cache.get(&name).cloned()
    };

    let (shortcut_id, target, creator_id) = if let Some((id, url, cid)) = cached {
        (id, url, cid)
    } else {
        // Cache miss: query DB and cache the result
        let shortcut = sqlx::query_as::<_, Shortcut>("SELECT * FROM shortcuts WHERE name = ?")
            .bind(&name)
            .fetch_optional(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        match shortcut {
            Some(s) => {
                let target = s.link.clone();
                // Cache for next time
                {
                    let mut cache = state.url_cache.write().await;
                    cache.insert(name.clone(), (s.id, target.clone(), s.creator_id));
                }
                (s.id, target, s.creator_id)
            }
            None => return Err(StatusCode::NOT_FOUND),
        }
    };

    // Redirect IMMEDIATELY — no DB, no dedup, just the cached URL
    let redirect_resp = Ok(axum::response::Redirect::temporary(&target));

    // ALL analytics work in background
    let user_agent = headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("unknown")
        .to_string();
    let referer = headers
        .get("referer")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("direct")
        .to_string();
    let ip = addr.ip().to_string();
    let query = uri.query().unwrap_or("").to_string();

    let db = state.db.clone();
    let dedup_cache = state.view_dedup.clone();
    tokio::spawn(async move {
        // Dedup: skip if same IP viewed this shortcut within last 60s
        let dedup_key = format!("{}:{}", shortcut_id, ip);
        let now = Utc::now().timestamp();
        {
            let mut dedup = dedup_cache.write().await;
            if let Some(last_seen) = dedup.get(&dedup_key) {
                if now - last_seen < 60 {
                    return; // Recent view, skip
                }
            }
            dedup.insert(dedup_key, now);
            if dedup.len() > 10000 {
                let cutoff = now - 120;
                dedup.retain(|_, ts| *ts > cutoff);
            }
        }

        // Increment view count
        let _ = sqlx::query("UPDATE shortcuts SET view_count = view_count + 1 WHERE id = ?")
            .bind(shortcut_id)
            .execute(&db)
            .await;

        let (device, browser, os) = parse_user_agent(&user_agent);
        let referer_domain = extract_domain(&referer);
        let utm_source = extract_utm_param(&query, "utm_source");
        let utm_medium = extract_utm_param(&query, "utm_medium");
        let utm_campaign = extract_utm_param(&query, "utm_campaign");
        let (country, city) = get_geo_from_ip(&ip).await;

        let payload = serde_json::json!({
            "referer": referer,
            "referer_domain": referer_domain,
            "user_agent": user_agent,
            "device": device,
            "browser": browser,
            "os": os,
            "ip": ip,
            "country": country,
            "city": city,
            "utm_source": utm_source,
            "utm_medium": utm_medium,
            "utm_campaign": utm_campaign,
        });

        let _ = sqlx::query(
            "INSERT INTO activities (creator_id, created_ts, type, level, payload, shortcut_id, referer, user_agent, ip_country, ip_city, utm_source, utm_medium, utm_campaign) VALUES (?, ?, 'shortcut.view', 'info', ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(creator_id)
        .bind(now)
        .bind(payload.to_string())
        .bind(shortcut_id)
        .bind(&referer)
        .bind(&user_agent)
        .bind(&country)
        .bind(&city)
        .bind(&utm_source)
        .bind(&utm_medium)
        .bind(&utm_campaign)
        .execute(&db)
        .await;
    });

    redirect_resp
}

fn parse_user_agent(ua: &str) -> (String, String, String) {
    let ua_lower = ua.to_lowercase();

    // Detect device type
    let device = if ua_lower.contains("mobile") || ua_lower.contains("android") || ua_lower.contains("iphone") {
        "Mobile"
    } else if ua_lower.contains("tablet") || ua_lower.contains("ipad") {
        "Tablet"
    } else {
        "Desktop"
    };

    // Detect browser
    let browser = if ua_lower.contains("firefox/") {
        "Firefox"
    } else if ua_lower.contains("edg/") || ua_lower.contains("edge/") {
        "Edge"
    } else if ua_lower.contains("chrome/") && !ua_lower.contains("chromium") {
        "Chrome"
    } else if ua_lower.contains("safari/") && !ua_lower.contains("chrome") {
        "Safari"
    } else if ua_lower.contains("opera/") || ua_lower.contains("opr/") {
        "Opera"
    } else {
        "Other"
    };

    // Detect OS
    let os = if ua_lower.contains("windows") {
        "Windows"
    } else if ua_lower.contains("mac os") || ua_lower.contains("macos") {
        "macOS"
    } else if ua_lower.contains("linux") && !ua_lower.contains("android") {
        "Linux"
    } else if ua_lower.contains("android") {
        "Android"
    } else if ua_lower.contains("iphone") || ua_lower.contains("ipad") || ua_lower.contains("ios") {
        "iOS"
    } else {
        "Other"
    };

    (device.to_string(), browser.to_string(), os.to_string())
}

fn extract_domain(url: &str) -> String {
    if url == "direct" || url.is_empty() {
        return "direct".to_string();
    }
    let without_protocol = url.strip_prefix("https://").or_else(|| url.strip_prefix("http://")).unwrap_or(url);
    without_protocol.split('/').next().unwrap_or("unknown").to_string()
}

fn extract_utm_param(query: &str, param: &str) -> Option<String> {
    for pair in query.split('&') {
        let mut parts = pair.splitn(2, '=');
        if let (Some(key), Some(value)) = (parts.next(), parts.next()) {
            if key == param {
                return Some(value.to_string());
            }
        }
    }
    None
}

async fn get_geo_from_ip(ip: &str) -> (Option<String>, Option<String>) {
    // Skip local/private IPs
    if ip.starts_with("127.") || ip.starts_with("10.") || ip.starts_with("192.168.") || ip.starts_with("172.") || ip == "::1" {
        return (None, None);
    }

    // Call ip-api.com with 2s timeout to avoid blocking
    let url = format!("http://ip-api.com/json/{}?fields=country,city", ip);
    let result = tokio::time::timeout(std::time::Duration::from_secs(2), async {
        reqwest::get(&url).await.ok()?.json::<serde_json::Value>().await.ok()
    }).await;

    match result {
        Ok(Some(data)) => {
            let country = data.get("country").and_then(|v| v.as_str()).map(|s| s.to_string());
            let city = data.get("city").and_then(|v| v.as_str()).map(|s| s.to_string());
            (country, city)
        }
        _ => (None, None),
    }
}

// Helper functions

async fn get_shortcut_tags(db: &sqlx::SqlitePool, shortcut_id: i64) -> Result<Vec<String>, sqlx::Error> {
    let tags = sqlx::query_scalar::<_, String>(
        "SELECT t.name FROM tags t JOIN shortcut_tags st ON t.id = st.tag_id WHERE st.shortcut_id = ?",
    )
    .bind(shortcut_id)
    .fetch_all(db)
    .await?;

    Ok(tags)
}

async fn add_shortcut_tags(db: &sqlx::SqlitePool, shortcut_id: i64, tags: &[String]) -> Result<(), sqlx::Error> {
    for tag_name in tags {
        // Insert or get tag
        sqlx::query("INSERT OR IGNORE INTO tags (name) VALUES (?)")
            .bind(tag_name)
            .execute(db)
            .await?;

        let tag_id = sqlx::query_scalar::<_, i64>("SELECT id FROM tags WHERE name = ?")
            .bind(tag_name)
            .fetch_one(db)
            .await?;

        // Link shortcut to tag
        sqlx::query("INSERT OR IGNORE INTO shortcut_tags (shortcut_id, tag_id) VALUES (?, ?)")
            .bind(shortcut_id)
            .bind(tag_id)
            .execute(db)
            .await?;
    }

    Ok(())
}
