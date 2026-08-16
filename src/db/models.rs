use serde::{Deserialize, Serialize};

// ===== User =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub created_ts: i64,
    pub updated_ts: i64,
    pub email: String,
    pub nickname: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: String, // "admin" | "user"
}

#[derive(Debug, Deserialize)]
pub struct CreateUser {
    pub email: String,
    pub nickname: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUser {
    pub nickname: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

// ===== Shortcut =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Shortcut {
    pub id: i64,
    pub creator_id: i64,
    pub created_ts: i64,
    pub updated_ts: i64,
    pub name: String,
    pub link: String,
    pub title: String,
    pub description: String,
    pub visibility: String, // "workspace" | "public"
    pub view_count: i64,
    pub og_title: String,
    pub og_description: String,
    pub og_image: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShortcutWithTags {
    #[serde(flatten)]
    pub shortcut: Shortcut,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreateShortcut {
    pub name: String,
    pub link: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub tags: Option<Vec<String>>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateShortcut {
    pub name: Option<String>,
    pub link: Option<String>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub visibility: Option<String>,
    pub tags: Option<Vec<String>>,
    pub og_title: Option<String>,
    pub og_description: Option<String>,
    pub og_image: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T: Serialize> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub per_page: i64,
    pub total_pages: i64,
}

// ===== Tag =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
}

// ===== Activity =====

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::FromRow)]
pub struct Activity {
    pub id: i64,
    pub creator_id: i64,
    pub created_ts: i64,
    pub r#type: String,
    pub level: String,
    pub payload: String,
    pub shortcut_id: Option<i64>,
    pub referer: Option<String>,
    pub user_agent: Option<String>,
    pub ip_country: Option<String>,
    pub ip_city: Option<String>,
    pub utm_source: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_campaign: Option<String>,
    pub duration_ms: Option<i64>,
}

// ===== Auth =====

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,   // user_id
    pub name: String,  // email
    pub exp: usize,    // expiration
    pub iat: usize,    // issued at
    pub iss: String,   // issuer
}

// ===== Analytics =====

#[derive(Debug, Serialize)]
pub struct ShortcutAnalytics {
    pub view_count: i64,
    pub references: Vec<AnalyticsItem>,
    pub devices: Vec<AnalyticsItem>,
    pub browsers: Vec<AnalyticsItem>,
    pub os: Vec<AnalyticsItem>,
    pub countries: Vec<AnalyticsItem>,
    pub cities: Vec<AnalyticsItem>,
    pub utm_sources: Vec<AnalyticsItem>,
    pub utm_mediums: Vec<AnalyticsItem>,
    pub utm_campaigns: Vec<AnalyticsItem>,
    pub activities: Vec<ActivityEntry>,
    pub views_by_date: Vec<ViewsByDate>,
}

#[derive(Debug, Serialize)]
pub struct AnalyticsItem {
    pub name: String,
    pub count: i64,
}

#[derive(Debug, Serialize)]
pub struct ActivityEntry {
    pub id: i64,
    pub created_ts: i64,
    pub ip: Option<String>,
    pub device: Option<String>,
    pub browser: Option<String>,
    pub os: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub referer: Option<String>,
    pub referer_domain: Option<String>,
    pub utm_source: Option<String>,
    pub utm_medium: Option<String>,
    pub utm_campaign: Option<String>,
    pub user_agent: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct ViewsByDate {
    pub date: String,
    pub count: i64,
}
