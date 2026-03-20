use std::net::SocketAddr;
use std::sync::Arc;

use axum::Json;
use axum::extract::{ConnectInfo, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::Serialize;
use sha2::{Digest, Sha256};

use crate::routes::admin::alerts::check_like_spike;
use crate::state::AppState;

#[derive(Serialize)]
struct LikeStatus {
    liked: bool,
}

fn hash_ip(ip: &str, salt: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(format!("{ip}{salt}"));
    format!("{:x}", hasher.finalize())
}

// ---- Player likes ----

/// GET /api/v1/player/{uuid}/like
pub async fn get_player_like(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let ip_hash = hash_ip(&addr.ip().to_string(), &state.ip_salt);

    let count = sqlx::query_scalar::<_, i32>(
        "SELECT COUNT(*) FROM likes WHERE entity_type = 'player' AND entity_id = ? AND ip_hash = ?",
    )
    .bind(&uuid)
    .bind(&ip_hash)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LikeStatus { liked: count > 0 }))
}

/// POST /api/v1/player/{uuid}/like
pub async fn like_player(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let ip_hash = hash_ip(&addr.ip().to_string(), &state.ip_salt);

    // Check the player exists in our index
    let exists = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM players WHERE uuid = ?")
        .bind(&uuid)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if exists == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    // Insert like (ignore if already liked)
    let result = sqlx::query(
        "INSERT OR IGNORE INTO likes (entity_type, entity_id, ip_hash) VALUES ('player', ?, ?)",
    )
    .bind(&uuid)
    .bind(&ip_hash)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Only increment if a new row was inserted
    if result.rows_affected() > 0 {
        sqlx::query("UPDATE players SET likes = likes + 1 WHERE uuid = ?")
            .bind(&uuid)
            .execute(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        // Check for like spike alerts
        check_like_spike(&state.db, "player", &uuid).await;
    }

    Ok(StatusCode::CREATED)
}

/// DELETE /api/v1/player/{uuid}/like
pub async fn unlike_player(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(uuid): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let ip_hash = hash_ip(&addr.ip().to_string(), &state.ip_salt);

    let result = sqlx::query(
        "DELETE FROM likes WHERE entity_type = 'player' AND entity_id = ? AND ip_hash = ?",
    )
    .bind(&uuid)
    .bind(&ip_hash)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() > 0 {
        sqlx::query("UPDATE players SET likes = MAX(0, likes - 1) WHERE uuid = ?")
            .bind(&uuid)
            .execute(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}

// ---- Server likes ----

/// GET /api/v1/server/{address}/like
pub async fn get_server_like(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(address): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let ip_hash = hash_ip(&addr.ip().to_string(), &state.ip_salt);

    let count = sqlx::query_scalar::<_, i32>(
        "SELECT COUNT(*) FROM likes WHERE entity_type = 'server' AND entity_id = ? AND ip_hash = ?",
    )
    .bind(&address)
    .bind(&ip_hash)
    .fetch_one(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LikeStatus { liked: count > 0 }))
}

/// POST /api/v1/server/{address}/like
pub async fn like_server(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(address): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let ip_hash = hash_ip(&addr.ip().to_string(), &state.ip_salt);

    let exists = sqlx::query_scalar::<_, i32>("SELECT COUNT(*) FROM servers WHERE address = ?")
        .bind(&address)
        .fetch_one(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if exists == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    let result = sqlx::query(
        "INSERT OR IGNORE INTO likes (entity_type, entity_id, ip_hash) VALUES ('server', ?, ?)",
    )
    .bind(&address)
    .bind(&ip_hash)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() > 0 {
        sqlx::query("UPDATE servers SET likes = likes + 1 WHERE address = ?")
            .bind(&address)
            .execute(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

        check_like_spike(&state.db, "server", &address).await;
    }

    Ok(StatusCode::CREATED)
}

/// DELETE /api/v1/server/{address}/like
pub async fn unlike_server(
    State(state): State<Arc<AppState>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    Path(address): Path<String>,
) -> Result<impl IntoResponse, StatusCode> {
    let ip_hash = hash_ip(&addr.ip().to_string(), &state.ip_salt);

    let result = sqlx::query(
        "DELETE FROM likes WHERE entity_type = 'server' AND entity_id = ? AND ip_hash = ?",
    )
    .bind(&address)
    .bind(&ip_hash)
    .execute(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if result.rows_affected() > 0 {
        sqlx::query("UPDATE servers SET likes = MAX(0, likes - 1) WHERE address = ?")
            .bind(&address)
            .execute(&state.db)
            .await
            .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    }

    Ok(StatusCode::NO_CONTENT)
}
