use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use super::AppState;
use crate::db::models::{Activity, ShortcutAnalytics, AnalyticsItem, ActivityEntry, ViewsByDate};

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

    // Aggregate data
    let mut reference_map = std::collections::HashMap::new();
    let mut referer_domain_map = std::collections::HashMap::new();
    let mut device_map = std::collections::HashMap::new();
    let mut browser_map = std::collections::HashMap::new();
    let mut os_map = std::collections::HashMap::new();
    let mut country_map = std::collections::HashMap::new();
    let mut city_map = std::collections::HashMap::new();
    let mut utm_source_map = std::collections::HashMap::new();
    let mut utm_medium_map = std::collections::HashMap::new();
    let mut utm_campaign_map = std::collections::HashMap::new();
    let mut date_map: std::collections::HashMap<String, i64> = std::collections::HashMap::new();
    let mut activity_log = Vec::new();

    for activity in &activities {
        let mut ip = None;
        let mut device = None;
        let mut browser = None;
        let mut os = None;
        let mut country = activity.ip_country.clone();
        let mut city = activity.ip_city.clone();
        let mut referer = activity.referer.clone();
        let mut referer_domain = None;
        let mut utm_source = activity.utm_source.clone();
        let mut utm_medium = activity.utm_medium.clone();
        let mut utm_campaign = activity.utm_campaign.clone();

        if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&activity.payload) {
            if let Some(r) = payload.get("referer").and_then(|v| v.as_str()) {
                if r != "direct" {
                    *reference_map.entry(r.to_string()).or_insert(0) += 1;
                    referer = Some(r.to_string());
                }
            }
            if let Some(rd) = payload.get("referer_domain").and_then(|v| v.as_str()) {
                *referer_domain_map.entry(rd.to_string()).or_insert(0) += 1;
                referer_domain = Some(rd.to_string());
            }
            if let Some(d) = payload.get("device").and_then(|v| v.as_str()) {
                *device_map.entry(d.to_string()).or_insert(0) += 1;
                device = Some(d.to_string());
            }
            if let Some(b) = payload.get("browser").and_then(|v| v.as_str()) {
                *browser_map.entry(b.to_string()).or_insert(0) += 1;
                browser = Some(b.to_string());
            }
            if let Some(o) = payload.get("os").and_then(|v| v.as_str()) {
                *os_map.entry(o.to_string()).or_insert(0) += 1;
                os = Some(o.to_string());
            }
            if let Some(c) = payload.get("country").and_then(|v| v.as_str()) {
                if !c.is_empty() {
                    *country_map.entry(c.to_string()).or_insert(0) += 1;
                    country = Some(c.to_string());
                }
            }
            if let Some(c) = payload.get("city").and_then(|v| v.as_str()) {
                if !c.is_empty() {
                    *city_map.entry(c.to_string()).or_insert(0) += 1;
                    city = Some(c.to_string());
                }
            }
            if let Some(us) = payload.get("utm_source").and_then(|v| v.as_str()) {
                if !us.is_empty() {
                    *utm_source_map.entry(us.to_string()).or_insert(0) += 1;
                    utm_source = Some(us.to_string());
                }
            }
            if let Some(um) = payload.get("utm_medium").and_then(|v| v.as_str()) {
                if !um.is_empty() {
                    *utm_medium_map.entry(um.to_string()).or_insert(0) += 1;
                    utm_medium = Some(um.to_string());
                }
            }
            if let Some(uc) = payload.get("utm_campaign").and_then(|v| v.as_str()) {
                if !uc.is_empty() {
                    *utm_campaign_map.entry(uc.to_string()).or_insert(0) += 1;
                    utm_campaign = Some(uc.to_string());
                }
            }
            if let Some(i) = payload.get("ip").and_then(|v| v.as_str()) {
                ip = Some(i.to_string());
            }
        }

        // Also check structured fields
        if let Some(ref c) = activity.ip_country {
            if !c.is_empty() && country.is_none() {
                *country_map.entry(c.clone()).or_insert(0) += 1;
                country = Some(c.clone());
            }
        }
        if let Some(ref s) = activity.utm_source {
            if !s.is_empty() && utm_source.is_none() {
                *utm_source_map.entry(s.clone()).or_insert(0) += 1;
                utm_source = Some(s.clone());
            }
        }
        if let Some(ref m) = activity.utm_medium {
            if !m.is_empty() && utm_medium.is_none() {
                *utm_medium_map.entry(m.clone()).or_insert(0) += 1;
                utm_medium = Some(m.clone());
            }
        }
        if let Some(ref c) = activity.utm_campaign {
            if !c.is_empty() && utm_campaign.is_none() {
                *utm_campaign_map.entry(c.clone()).or_insert(0) += 1;
                utm_campaign = Some(c.clone());
            }
        }

        // Group by date
        let date = chrono::NaiveDateTime::from_timestamp_opt(activity.created_ts, 0)
            .map(|dt| dt.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "unknown".to_string());
        *date_map.entry(date).or_insert(0) += 1;

        activity_log.push(ActivityEntry {
            id: activity.id,
            created_ts: activity.created_ts,
            ip,
            device,
            browser,
            os,
            country,
            city,
            referer,
            referer_domain,
            utm_source,
            utm_medium,
            utm_campaign,
            user_agent: activity.user_agent.clone(),
        });
    }

    // Convert date map to sorted vec
    let mut views_by_date: Vec<ViewsByDate> = date_map
        .into_iter()
        .map(|(date, count)| ViewsByDate { date, count })
        .collect();
    views_by_date.sort_by(|a, b| a.date.cmp(&b.date));

    Ok(Json(ShortcutAnalytics {
        view_count,
        references: map_to_analytics(reference_map),
        devices: map_to_analytics(device_map),
        browsers: map_to_analytics(browser_map),
        os: map_to_analytics(os_map),
        countries: map_to_analytics(country_map),
        cities: map_to_analytics(city_map),
        utm_sources: map_to_analytics(utm_source_map),
        utm_mediums: map_to_analytics(utm_medium_map),
        utm_campaigns: map_to_analytics(utm_campaign_map),
        activities: activity_log,
        views_by_date,
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
