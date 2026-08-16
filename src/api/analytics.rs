use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use super::AppState;
use crate::db::models::{Activity, ShortcutAnalytics, AnalyticsItem, ActivityEntry};

pub async fn shortcut_analytics(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<Json<ShortcutAnalytics>, StatusCode> {
    // Get shortcut and check existence in one query
    let shortcut = sqlx::query_scalar::<_, i64>("SELECT view_count FROM shortcuts WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let view_count = match shortcut {
        Some(v) => v,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Get analytics data (limit to last 1000 for performance)
    let activities = sqlx::query_as::<_, Activity>(
        "SELECT * FROM activities WHERE shortcut_id = ? AND type = 'shortcut.view' ORDER BY created_ts DESC LIMIT 1000",
    )
    .bind(id)
    .fetch_all(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Aggregate data and build activity log
    let mut reference_map = std::collections::HashMap::new();
    let mut device_map = std::collections::HashMap::new();
    let mut browser_map = std::collections::HashMap::new();
    let mut country_map = std::collections::HashMap::new();
    let mut utm_source_map = std::collections::HashMap::new();
    let mut utm_medium_map = std::collections::HashMap::new();
    let mut utm_campaign_map = std::collections::HashMap::new();
    let mut activity_log = Vec::new();

    for activity in &activities {
        let mut ip = None;
        let mut device = None;
        let mut browser = None;
        let mut referer = activity.referer.clone();

        if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&activity.payload) {
            if let Some(r) = payload.get("referer").and_then(|v| v.as_str()) {
                if r != "direct" {
                    *reference_map.entry(r.to_string()).or_insert(0) += 1;
                    referer = Some(r.to_string());
                }
            }
            if let Some(d) = payload.get("device").and_then(|v| v.as_str()) {
                *device_map.entry(d.to_string()).or_insert(0) += 1;
                device = Some(d.to_string());
            }
            if let Some(b) = payload.get("browser").and_then(|v| v.as_str()) {
                *browser_map.entry(b.to_string()).or_insert(0) += 1;
                browser = Some(b.to_string());
            }
            if let Some(i) = payload.get("ip").and_then(|v| v.as_str()) {
                ip = Some(i.to_string());
            }
        }

        if let Some(ref country) = activity.ip_country {
            if !country.is_empty() {
                *country_map.entry(country.clone()).or_insert(0) += 1;
            }
        }
        if let Some(ref source) = activity.utm_source {
            if !source.is_empty() {
                *utm_source_map.entry(source.clone()).or_insert(0) += 1;
            }
        }
        if let Some(ref medium) = activity.utm_medium {
            if !medium.is_empty() {
                *utm_medium_map.entry(medium.clone()).or_insert(0) += 1;
            }
        }
        if let Some(ref campaign) = activity.utm_campaign {
            if !campaign.is_empty() {
                *utm_campaign_map.entry(campaign.clone()).or_insert(0) += 1;
            }
        }

        activity_log.push(ActivityEntry {
            id: activity.id,
            created_ts: activity.created_ts,
            ip,
            device,
            browser,
            referer,
            user_agent: activity.user_agent.clone(),
        });
    }

    Ok(Json(ShortcutAnalytics {
        view_count,
        references: map_to_analytics(reference_map),
        devices: map_to_analytics(device_map),
        browsers: map_to_analytics(browser_map),
        countries: map_to_analytics(country_map),
        utm_sources: map_to_analytics(utm_source_map),
        utm_mediums: map_to_analytics(utm_medium_map),
        utm_campaigns: map_to_analytics(utm_campaign_map),
        activities: activity_log,
    }))
}

fn map_to_analytics(map: std::collections::HashMap<String, i64>) -> Vec<AnalyticsItem> {
    let mut items: Vec<AnalyticsItem> = map
        .into_iter()
        .map(|(name, count)| AnalyticsItem { name, count })
        .collect();

    items.sort_by(|a, b| b.count.cmp(&a.count));
    items
}
