use std::sync::Arc;

use axum::Json;
use axum::extract::{Extension, State};
use serde::Serialize;

use crate::error::ApiError;
use crate::middleware::admin_auth::AdminClaims;
use crate::state::AppState;

#[derive(Serialize, sqlx::FromRow)]
pub struct DashboardMetrics {
    pub total_players: i64,
    pub total_servers: i64,
    pub total_views_players: i64,
    pub total_views_servers: i64,
    pub total_likes: i64,
    pub total_favorites: i64,
    pub players_last_24h: i64,
    pub servers_last_24h: i64,
    pub admin_count: i64,
}

/// GET /api/v1/admin/dashboard (protected)
pub async fn dashboard(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<AdminClaims>,
) -> Result<Json<DashboardMetrics>, ApiError> {
    let metrics: DashboardMetrics = sqlx::query_as(
        "SELECT
            (SELECT COUNT(*) FROM players) as total_players,
            (SELECT COUNT(*) FROM servers) as total_servers,
            (SELECT COALESCE(SUM(views), 0) FROM players) as total_views_players,
            (SELECT COALESCE(SUM(views), 0) FROM servers) as total_views_servers,
            (SELECT COUNT(*) FROM likes) as total_likes,
            (SELECT COUNT(*) FROM favorites) as total_favorites,
            (SELECT COUNT(*) FROM players WHERE last_seen_at > datetime('now', '-1 day')) as players_last_24h,
            (SELECT COUNT(*) FROM servers WHERE last_seen_at > datetime('now', '-1 day')) as servers_last_24h,
            (SELECT COUNT(*) FROM admin_users) as admin_count",
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(metrics))
}
