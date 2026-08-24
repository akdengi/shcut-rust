use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use serde::Deserialize;
use tracing::error;

use super::AppState;
use super::auth_extractor::AuthClaims;
use crate::db::models::{Activity, ShortcutAnalytics, AnalyticsItem, ActivityEntry, ViewsByDate};

#[derive(Debug, Deserialize)]
pub struct AnalyticsQuery {
    pub from: Option<i64>,
    pub to: Option<i64>,
}

pub async fn shortcut_analytics(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Query(params): Query<AnalyticsQuery>,
    auth: AuthClaims,
) -> Result<Json<ShortcutAnalytics>, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Check role - view role cannot access analytics
    let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    if role == "view" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Get shortcut and check existence + ownership
    let shortcut = sqlx::query_as::<_, (i64, i64)>("SELECT view_count, creator_id FROM shortcuts WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let (view_count, creator_id) = match shortcut {
        Some(v) => v,
        None => return Err(StatusCode::NOT_FOUND),
    };

    // Non-admin users can only see their own shortcuts' analytics
    if role != "admin" && creator_id != user_id {
        return Err(StatusCode::FORBIDDEN);
    }

    // Build query with date range
    let mut query = "SELECT * FROM activities WHERE shortcut_id = ? AND type = 'shortcut.view'".to_string();
    if params.from.is_some() {
        query.push_str(" AND created_ts >= ?");
    }
    if params.to.is_some() {
        query.push_str(" AND created_ts <= ?");
    }
    query.push_str(" ORDER BY created_ts DESC LIMIT 1000");

    let mut query_builder = sqlx::query_as::<_, Activity>(&query).bind(id);
    if let Some(from) = params.from {
        query_builder = query_builder.bind(from);
    }
    if let Some(to) = params.to {
        query_builder = query_builder.bind(to);
    }

    let activities = query_builder
        .fetch_all(&state.db)
        .await
        .map_err(|e| {
            error!("Failed to fetch activities for shortcut {}: {}", id, e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Aggregate data
    let mut reference_map = std::collections::HashMap::new();
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
        let mut country = None;
        let mut city = None;
        let mut referer = None;
        let mut referer_domain = None;
        let mut utm_source = None;
        let mut utm_medium = None;
        let mut utm_campaign = None;

        if let Ok(payload) = serde_json::from_str::<serde_json::Value>(&activity.payload) {
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
            if let Some(i) = payload.get("ip").and_then(|v| v.as_str()) {
                ip = Some(i.to_string());
            }

            // Referrer
            if let Some(r) = payload.get("referer").and_then(|v| v.as_str()) {
                if r != "direct" {
                    *reference_map.entry(r.to_string()).or_insert(0) += 1;
                    referer = Some(r.to_string());
                }
            }
            if let Some(rd) = payload.get("referer_domain").and_then(|v| v.as_str()) {
                referer_domain = Some(rd.to_string());
            }

            // Country/City
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

            // UTM
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
        }

        // Group by date
        let date = chrono::DateTime::from_timestamp(activity.created_ts, 0)
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

pub async fn reset_analytics(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    auth: super::auth_extractor::AuthClaims,
) -> Result<StatusCode, StatusCode> {
    let user_id: i64 = auth.0.sub.parse().map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Only admin can reset analytics
    let role = sqlx::query_scalar::<_, String>("SELECT role FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if role != "admin" {
        return Err(StatusCode::FORBIDDEN);
    }

    // Check shortcut exists
    let exists = sqlx::query_scalar::<_, i64>("SELECT id FROM shortcuts WHERE id = ?")
        .bind(id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if exists.is_none() {
        return Err(StatusCode::NOT_FOUND);
    }

    // Delete activities and reset view count
    sqlx::query("DELETE FROM activities WHERE shortcut_id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("UPDATE shortcuts SET view_count = 0 WHERE id = ?")
        .bind(id)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(StatusCode::NO_CONTENT)
}

fn map_to_analytics(map: std::collections::HashMap<String, i64>) -> Vec<AnalyticsItem> {
    let mut items: Vec<AnalyticsItem> = map
        .into_iter()
        .map(|(name, count)| AnalyticsItem { name, count })
        .collect();

    items.sort_by(|a, b| b.count.cmp(&a.count));
    items
}
