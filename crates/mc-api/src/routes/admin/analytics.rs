use std::sync::Arc;

use axum::Json;
use axum::extract::{Extension, Query, State};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::middleware::admin_auth::AdminClaims;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct AnalyticsQuery {
    /// Number of days to look back (default 30, max 365)
    #[serde(default = "default_days")]
    pub days: i64,
}

fn default_days() -> i64 {
    30
}

// -- Growth: new entries per day --

#[derive(Serialize, sqlx::FromRow)]
pub struct DailyCount {
    pub date: String,
    pub count: i64,
}

#[derive(Serialize)]
pub struct GrowthResponse {
    pub players: Vec<DailyCount>,
    pub servers: Vec<DailyCount>,
    pub likes: Vec<DailyCount>,
}

/// GET /api/v1/admin/analytics/growth?days=30 (protected)
pub async fn growth(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<AdminClaims>,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<GrowthResponse>, ApiError> {
    let days = query.days.clamp(1, 365);
    let days_str = format!("-{days} days");

    let players: Vec<DailyCount> = sqlx::query_as(
        "SELECT DATE(first_seen_at) as date, COUNT(*) as count
         FROM players WHERE first_seen_at > datetime('now', ?)
         GROUP BY DATE(first_seen_at) ORDER BY date ASC",
    )
    .bind(&days_str)
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let servers: Vec<DailyCount> = sqlx::query_as(
        "SELECT DATE(first_seen_at) as date, COUNT(*) as count
         FROM servers WHERE first_seen_at > datetime('now', ?)
         GROUP BY DATE(first_seen_at) ORDER BY date ASC",
    )
    .bind(&days_str)
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let likes: Vec<DailyCount> = sqlx::query_as(
        "SELECT DATE(created_at) as date, COUNT(*) as count
         FROM likes WHERE created_at > datetime('now', ?)
         GROUP BY DATE(created_at) ORDER BY date ASC",
    )
    .bind(&days_str)
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(GrowthResponse {
        players,
        servers,
        likes,
    }))
}

// -- Activity: searches per day (based on last_seen_at updates via audit-like approach) --
// Since we only track last_seen_at (not each search), we use admin_audit_log + views growth

#[derive(Serialize)]
pub struct ActivityResponse {
    pub logins: Vec<DailyCount>,
    pub actions: Vec<DailyCount>,
}

/// GET /api/v1/admin/analytics/activity?days=30 (protected)
pub async fn activity(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<AdminClaims>,
    Query(query): Query<AnalyticsQuery>,
) -> Result<Json<ActivityResponse>, ApiError> {
    let days = query.days.clamp(1, 365);
    let days_str = format!("-{days} days");

    let logins: Vec<DailyCount> = sqlx::query_as(
        "SELECT DATE(created_at) as date, COUNT(*) as count
         FROM admin_audit_log WHERE action = 'login' AND created_at > datetime('now', ?)
         GROUP BY DATE(created_at) ORDER BY date ASC",
    )
    .bind(&days_str)
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let actions: Vec<DailyCount> = sqlx::query_as(
        "SELECT DATE(created_at) as date, COUNT(*) as count
         FROM admin_audit_log WHERE created_at > datetime('now', ?)
         GROUP BY DATE(created_at) ORDER BY date ASC",
    )
    .bind(&days_str)
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(ActivityResponse { logins, actions }))
}

// -- Top entities --

#[derive(Serialize, sqlx::FromRow)]
pub struct TopPlayer {
    pub uuid: String,
    pub username: String,
    pub views: i64,
    pub likes: i64,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct TopServer {
    pub address: String,
    pub hostname: String,
    pub views: i64,
    pub likes: i64,
}

#[derive(Serialize)]
pub struct TopResponse {
    pub players_by_views: Vec<TopPlayer>,
    pub players_by_likes: Vec<TopPlayer>,
    pub servers_by_views: Vec<TopServer>,
    pub servers_by_likes: Vec<TopServer>,
}

/// GET /api/v1/admin/analytics/top (protected)
pub async fn top(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<AdminClaims>,
) -> Result<Json<TopResponse>, ApiError> {
    let players_by_views: Vec<TopPlayer> = sqlx::query_as(
        "SELECT uuid, username, views, likes FROM players ORDER BY views DESC LIMIT 10",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let players_by_likes: Vec<TopPlayer> = sqlx::query_as(
        "SELECT uuid, username, views, likes FROM players ORDER BY likes DESC LIMIT 10",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let servers_by_views: Vec<TopServer> = sqlx::query_as(
        "SELECT address, hostname, views, likes FROM servers ORDER BY views DESC LIMIT 10",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let servers_by_likes: Vec<TopServer> = sqlx::query_as(
        "SELECT address, hostname, views, likes FROM servers ORDER BY likes DESC LIMIT 10",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(TopResponse {
        players_by_views,
        players_by_likes,
        servers_by_views,
        servers_by_likes,
    }))
}
