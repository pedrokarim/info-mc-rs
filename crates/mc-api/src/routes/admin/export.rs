use std::sync::Arc;

use axum::extract::{Extension, State};
use axum::http::header;
use axum::response::{IntoResponse, Response};

use crate::error::ApiError;
use crate::middleware::admin_auth::AdminClaims;
use crate::state::AppState;

fn csv_escape(s: &str) -> String {
    if s.contains(',') || s.contains('"') || s.contains('\n') {
        format!("\"{}\"", s.replace('"', "\"\""))
    } else {
        s.to_string()
    }
}

fn opt(s: &Option<String>) -> String {
    s.as_deref().unwrap_or("").to_string()
}

// -- Players export --

#[derive(sqlx::FromRow)]
struct PlayerRow {
    uuid: String,
    username: String,
    skin_url: Option<String>,
    skin_model: Option<String>,
    status: String,
    views: i64,
    likes: i64,
    first_seen_at: String,
    last_seen_at: String,
}

/// GET /api/v1/admin/export/players (protected)
pub async fn export_players(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
) -> Result<Response, ApiError> {
    let rows: Vec<PlayerRow> = sqlx::query_as(
        "SELECT uuid, username, skin_url, skin_model, status, views, likes, first_seen_at, last_seen_at
         FROM players ORDER BY views DESC",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let mut csv = String::from(
        "uuid,username,skin_url,skin_model,status,views,likes,first_seen_at,last_seen_at\n",
    );
    for r in &rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{}\n",
            csv_escape(&r.uuid),
            csv_escape(&r.username),
            csv_escape(&opt(&r.skin_url)),
            csv_escape(&opt(&r.skin_model)),
            csv_escape(&r.status),
            r.views,
            r.likes,
            csv_escape(&r.first_seen_at),
            csv_escape(&r.last_seen_at),
        ));
    }

    // Audit
    sqlx::query(
        "INSERT INTO admin_audit_log (discord_id, action, detail) VALUES (?, 'export_players', ?)",
    )
    .bind(&claims.sub)
    .bind(format!("{} rows", rows.len()))
    .execute(&state.db)
    .await
    .ok();

    Ok((
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"players.csv\"",
            ),
        ],
        csv,
    )
        .into_response())
}

// -- Servers export --

#[derive(sqlx::FromRow)]
struct ServerRow {
    address: String,
    hostname: String,
    ip: String,
    port: i64,
    edition: String,
    version_name: Option<String>,
    motd_clean: Option<String>,
    max_players: Option<i64>,
    status: String,
    views: i64,
    likes: i64,
    first_seen_at: String,
    last_seen_at: String,
    last_online_at: Option<String>,
}

/// GET /api/v1/admin/export/servers (protected)
pub async fn export_servers(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
) -> Result<Response, ApiError> {
    let rows: Vec<ServerRow> = sqlx::query_as(
        "SELECT address, hostname, ip, port, edition, version_name, motd_clean, max_players, status, views, likes, first_seen_at, last_seen_at, last_online_at
         FROM servers ORDER BY views DESC",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let mut csv = String::from(
        "address,hostname,ip,port,edition,version_name,motd_clean,max_players,status,views,likes,first_seen_at,last_seen_at,last_online_at\n",
    );
    for r in &rows {
        csv.push_str(&format!(
            "{},{},{},{},{},{},{},{},{},{},{},{},{},{}\n",
            csv_escape(&r.address),
            csv_escape(&r.hostname),
            csv_escape(&r.ip),
            r.port,
            csv_escape(&r.edition),
            csv_escape(&opt(&r.version_name)),
            csv_escape(&opt(&r.motd_clean)),
            r.max_players.map(|v| v.to_string()).unwrap_or_default(),
            csv_escape(&r.status),
            r.views,
            r.likes,
            csv_escape(&r.first_seen_at),
            csv_escape(&r.last_seen_at),
            csv_escape(&opt(&r.last_online_at)),
        ));
    }

    sqlx::query(
        "INSERT INTO admin_audit_log (discord_id, action, detail) VALUES (?, 'export_servers', ?)",
    )
    .bind(&claims.sub)
    .bind(format!("{} rows", rows.len()))
    .execute(&state.db)
    .await
    .ok();

    Ok((
        [
            (header::CONTENT_TYPE, "text/csv; charset=utf-8"),
            (
                header::CONTENT_DISPOSITION,
                "attachment; filename=\"servers.csv\"",
            ),
        ],
        csv,
    )
        .into_response())
}
