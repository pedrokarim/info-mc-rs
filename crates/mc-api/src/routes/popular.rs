use axum::Json;
use axum::extract::{Query, State};
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::state::SharedState;

/// Popularity stats attached to player/server responses.
#[derive(Debug, Clone, Serialize)]
pub struct Popularity {
    pub views: i64,
    pub likes: i64,
    pub first_seen_at: String,
    pub last_seen_at: String,
}

// -- Popular/Recent player entries --

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PopularPlayerEntry {
    pub uuid: String,
    pub username: String,
    pub skin_url: Option<String>,
    pub skin_model: Option<String>,
    pub views: i64,
    pub likes: i64,
    pub first_seen_at: String,
    pub last_seen_at: String,
}

// -- Popular/Recent server entries --

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct PopularServerEntry {
    pub address: String,
    pub hostname: String,
    pub port: i64,
    pub edition: String,
    pub version_name: Option<String>,
    pub motd_clean: Option<String>,
    pub motd_html: Option<String>,
    pub favicon: Option<String>,
    pub max_players: Option<i64>,
    pub views: i64,
    pub likes: i64,
    pub first_seen_at: String,
    pub last_seen_at: String,
    pub last_online_at: Option<String>,
}

// -- Query params --

#[derive(Deserialize)]
pub struct PopularQuery {
    #[serde(default = "default_sort")]
    pub sort: String,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_sort() -> String {
    "views".to_string()
}

fn default_limit() -> i64 {
    20
}

#[derive(Deserialize)]
pub struct RecentQuery {
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn clamp_limit(limit: i64) -> i64 {
    limit.clamp(1, 100)
}

fn clamp_offset(offset: i64) -> i64 {
    offset.max(0)
}

// -- Handlers --

/// GET /api/v1/popular/players?sort=views|likes&limit=20&offset=0
pub async fn popular_players(
    State(state): State<SharedState>,
    Query(query): Query<PopularQuery>,
) -> Result<Json<Vec<PopularPlayerEntry>>, ApiError> {
    let order_col = match query.sort.as_str() {
        "likes" => "likes",
        _ => "views",
    };
    let limit = clamp_limit(query.limit);
    let offset = clamp_offset(query.offset);

    // sqlx doesn't support dynamic ORDER BY, so we use two branches
    let rows: Vec<PopularPlayerEntry> = if order_col == "likes" {
        sqlx::query_as(
            "SELECT uuid, username, skin_url, skin_model, views, likes, first_seen_at, last_seen_at
             FROM players WHERE status = 'active'
             ORDER BY likes DESC, views DESC
             LIMIT ? OFFSET ?",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?
    } else {
        sqlx::query_as(
            "SELECT uuid, username, skin_url, skin_model, views, likes, first_seen_at, last_seen_at
             FROM players WHERE status = 'active'
             ORDER BY views DESC, likes DESC
             LIMIT ? OFFSET ?",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?
    };

    Ok(Json(rows))
}

/// GET /api/v1/popular/servers?sort=views|likes&limit=20&offset=0
pub async fn popular_servers(
    State(state): State<SharedState>,
    Query(query): Query<PopularQuery>,
) -> Result<Json<Vec<PopularServerEntry>>, ApiError> {
    let order_col = match query.sort.as_str() {
        "likes" => "likes",
        _ => "views",
    };
    let limit = clamp_limit(query.limit);
    let offset = clamp_offset(query.offset);

    let rows: Vec<PopularServerEntry> = if order_col == "likes" {
        sqlx::query_as(
            "SELECT address, hostname, port, edition, version_name, motd_clean, motd_html, favicon, max_players,
                    views, likes, first_seen_at, last_seen_at, last_online_at
             FROM servers WHERE status = 'active'
             ORDER BY likes DESC, views DESC
             LIMIT ? OFFSET ?",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?
    } else {
        sqlx::query_as(
            "SELECT address, hostname, port, edition, version_name, motd_clean, motd_html, favicon, max_players,
                    views, likes, first_seen_at, last_seen_at, last_online_at
             FROM servers WHERE status = 'active'
             ORDER BY views DESC, likes DESC
             LIMIT ? OFFSET ?",
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?
    };

    Ok(Json(rows))
}

/// GET /api/v1/recent/players?limit=20&offset=0
pub async fn recent_players(
    State(state): State<SharedState>,
    Query(query): Query<RecentQuery>,
) -> Result<Json<Vec<PopularPlayerEntry>>, ApiError> {
    let limit = clamp_limit(query.limit);
    let offset = clamp_offset(query.offset);

    let rows: Vec<PopularPlayerEntry> = sqlx::query_as(
        "SELECT uuid, username, skin_url, skin_model, views, likes, first_seen_at, last_seen_at
         FROM players WHERE status = 'active'
         ORDER BY last_seen_at DESC
         LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(rows))
}

/// GET /api/v1/recent/servers?limit=20&offset=0
pub async fn recent_servers(
    State(state): State<SharedState>,
    Query(query): Query<RecentQuery>,
) -> Result<Json<Vec<PopularServerEntry>>, ApiError> {
    let limit = clamp_limit(query.limit);
    let offset = clamp_offset(query.offset);

    let rows: Vec<PopularServerEntry> = sqlx::query_as(
        "SELECT address, hostname, port, edition, version_name, motd_clean, favicon, max_players,
                views, likes, first_seen_at, last_seen_at, last_online_at
         FROM servers WHERE status = 'active'
         ORDER BY last_seen_at DESC
         LIMIT ? OFFSET ?",
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(rows))
}
