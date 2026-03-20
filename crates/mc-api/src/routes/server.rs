use axum::Json;
use axum::extract::{Path, Query, State};
use serde::{Deserialize, Serialize};

use mc_protocol::{SlpConfig, SlpResponse};

use crate::error::ApiError;
use crate::routes::popular::Popularity;
use crate::state::SharedState;

#[derive(Deserialize)]
pub struct ServerQuery {
    /// "java", "bedrock", or "auto" (default)
    #[serde(default = "default_type")]
    pub r#type: String,
}

fn default_type() -> String {
    "auto".to_string()
}

#[derive(Debug, Clone, Serialize)]
pub struct ServerResponse {
    pub online: bool,
    pub address: AddressInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<VersionInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub players: Option<PlayersInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub motd: Option<mc_motd::MotdRendered>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub favicon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latency_ms: Option<u64>,
    pub edition: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    pub retrieved_at: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub popularity: Option<Popularity>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AddressInfo {
    pub hostname: String,
    pub ip: String,
    pub port: u16,
    pub srv_record: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct VersionInfo {
    pub name: String,
    pub protocol: i32,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayersInfo {
    pub online: i32,
    pub max: i32,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub sample: Vec<PlayerSampleInfo>,
}

#[derive(Debug, Clone, Serialize)]
pub struct PlayerSampleInfo {
    pub name: String,
    pub uuid: String,
}

pub async fn get_server(
    State(state): State<SharedState>,
    Path(address): Path<String>,
    Query(query): Query<ServerQuery>,
) -> Result<Json<ServerResponse>, ApiError> {
    // Validate address (basic check)
    let address = address.trim().to_string();
    if address.is_empty() || address.len() > 253 {
        return Err(ApiError::InvalidAddress(
            "address must be 1-253 characters".into(),
        ));
    }

    // Cache key includes the query type
    let cache_key = format!("{}:{}", address, query.r#type);

    // Check cache
    if let Some(cached) = state.server_cache.get(&cache_key).await {
        return Ok(Json((*cached).clone()));
    }

    // Resolve DNS
    let resolved = mc_protocol::resolve_address(&address).await?;

    let now = chrono::Utc::now().to_rfc3339();
    let address_info = AddressInfo {
        hostname: resolved.hostname.clone(),
        ip: resolved.ip.clone(),
        port: resolved.port,
        srv_record: resolved.srv_found,
    };

    let response = match query.r#type.as_str() {
        "bedrock" => ping_bedrock(&resolved, address_info, &now).await,
        "java" => ping_java(&resolved, address_info, &now).await,
        _ => {
            // Auto-detect: try Java first, then Bedrock
            let java_result = ping_java(&resolved, address_info.clone(), &now).await;
            if java_result.online {
                java_result
            } else {
                // Try Bedrock on default port
                let bedrock = ping_bedrock(&resolved, address_info, &now).await;
                if bedrock.online {
                    bedrock
                } else {
                    java_result // Return Java result (with offline status)
                }
            }
        }
    };

    // Persist online servers to the index and get popularity stats
    let response = if response.online {
        let popularity = persist_server(&state.db, &response).await;
        ServerResponse {
            popularity,
            ..response
        }
    } else {
        response
    };

    // Cache the response
    state.server_cache.insert(cache_key, response.clone()).await;

    Ok(Json(response))
}

async fn ping_java(
    resolved: &mc_protocol::ResolvedAddress,
    address_info: AddressInfo,
    now: &str,
) -> ServerResponse {
    match mc_protocol::ping_java(
        &resolved.ip,
        resolved.port,
        &resolved.hostname,
        &SlpConfig::default(),
    )
    .await
    {
        Ok(slp) => build_java_response(slp, address_info, now),
        Err(e) => ServerResponse {
            online: false,
            address: address_info,
            version: None,
            players: None,
            motd: None,
            favicon: None,
            latency_ms: None,
            edition: "java".to_string(),
            error: Some(format_error(&e)),
            retrieved_at: now.to_string(),
            popularity: None,
        },
    }
}

async fn ping_bedrock(
    resolved: &mc_protocol::ResolvedAddress,
    address_info: AddressInfo,
    now: &str,
) -> ServerResponse {
    let port = if resolved.port == 25565 {
        19132
    } else {
        resolved.port
    };

    match mc_protocol::ping_bedrock(&resolved.ip, port, &mc_protocol::BedrockConfig::default())
        .await
    {
        Ok(bedrock) => ServerResponse {
            online: true,
            address: AddressInfo {
                port,
                ..address_info
            },
            version: Some(VersionInfo {
                name: bedrock.version.clone(),
                protocol: bedrock.protocol_version as i32,
            }),
            players: Some(PlayersInfo {
                online: bedrock.players_online as i32,
                max: bedrock.players_max as i32,
                sample: Vec::new(),
            }),
            motd: Some(mc_motd::MotdRendered {
                raw: bedrock.motd_line1.clone(),
                clean: format!(
                    "{}{}",
                    bedrock.motd_line1,
                    bedrock
                        .motd_line2
                        .as_deref()
                        .map(|l| format!("\n{l}"))
                        .unwrap_or_default()
                ),
                html: format!(
                    "{}{}",
                    bedrock.motd_line1,
                    bedrock
                        .motd_line2
                        .as_deref()
                        .map(|l| format!("<br>{l}"))
                        .unwrap_or_default()
                ),
            }),
            favicon: None,
            latency_ms: Some(bedrock.latency_ms),
            edition: "bedrock".to_string(),
            error: None,
            retrieved_at: now.to_string(),
            popularity: None,
        },
        Err(e) => ServerResponse {
            online: false,
            address: AddressInfo {
                port,
                ..address_info
            },
            version: None,
            players: None,
            motd: None,
            favicon: None,
            latency_ms: None,
            edition: "bedrock".to_string(),
            error: Some(format_error(&e)),
            retrieved_at: now.to_string(),
            popularity: None,
        },
    }
}

fn build_java_response(slp: SlpResponse, address_info: AddressInfo, now: &str) -> ServerResponse {
    // Render MOTD using mc-motd
    let motd_value = match &slp.description {
        mc_protocol::MotdComponent::Plain(s) => serde_json::Value::String(s.clone()),
        mc_protocol::MotdComponent::Component(v) => v.clone(),
    };
    let motd = mc_motd::render_motd(&motd_value);

    ServerResponse {
        online: true,
        address: address_info,
        version: Some(VersionInfo {
            name: slp.version.name,
            protocol: slp.version.protocol,
        }),
        players: Some(PlayersInfo {
            online: slp.players.online,
            max: slp.players.max,
            sample: slp
                .players
                .sample
                .into_iter()
                .map(|p| PlayerSampleInfo {
                    name: p.name,
                    uuid: p.id,
                })
                .collect(),
        }),
        motd: Some(motd),
        favicon: slp.favicon,
        latency_ms: Some(slp.latency_ms),
        edition: "java".to_string(),
        error: None,
        retrieved_at: now.to_string(),
        popularity: None,
    }
}

fn format_error(e: &mc_protocol::McProtocolError) -> String {
    match e {
        mc_protocol::McProtocolError::ConnectionRefused(_) => "connection_refused".to_string(),
        mc_protocol::McProtocolError::Timeout(_) => "timeout".to_string(),
        mc_protocol::McProtocolError::DnsFailure(_) => "dns_resolution_failed".to_string(),
        _ => e.to_string(),
    }
}

/// Normalize server address for use as DB key: "hostname:port"
fn normalize_address(resp: &ServerResponse) -> String {
    let default_port = if resp.edition == "bedrock" {
        19132
    } else {
        25565
    };
    if resp.address.port == default_port {
        resp.address.hostname.to_lowercase()
    } else {
        format!(
            "{}:{}",
            resp.address.hostname.to_lowercase(),
            resp.address.port
        )
    }
}

/// Upsert server into the persistent index and return popularity stats.
async fn persist_server(db: &sqlx::SqlitePool, resp: &ServerResponse) -> Option<Popularity> {
    let address = normalize_address(resp);
    let motd_clean = resp.motd.as_ref().map(|m| m.clean.as_str());
    let motd_html = resp.motd.as_ref().map(|m| m.html.as_str());
    let version_name = resp.version.as_ref().map(|v| v.name.as_str());
    let max_players = resp.players.as_ref().map(|p| p.max as i64);

    let upsert_result = sqlx::query(
        "INSERT INTO servers (address, hostname, ip, port, edition, version_name, motd_clean, motd_html, favicon, max_players, last_online_at)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, datetime('now'))
         ON CONFLICT(address) DO UPDATE SET
            hostname = excluded.hostname,
            ip = excluded.ip,
            port = excluded.port,
            edition = excluded.edition,
            version_name = excluded.version_name,
            motd_clean = excluded.motd_clean,
            motd_html = excluded.motd_html,
            favicon = excluded.favicon,
            max_players = excluded.max_players,
            last_seen_at = datetime('now'),
            last_online_at = datetime('now'),
            views = views + 1",
    )
    .bind(&address)
    .bind(&resp.address.hostname)
    .bind(&resp.address.ip)
    .bind(resp.address.port as i64)
    .bind(&resp.edition)
    .bind(version_name)
    .bind(motd_clean)
    .bind(motd_html)
    .bind(&resp.favicon)
    .bind(max_players)
    .execute(db)
    .await;

    if upsert_result.is_err() {
        tracing::warn!(
            "failed to persist server {address}: {:?}",
            upsert_result.err()
        );
        return None;
    }

    let row = sqlx::query_as::<_, (i64, i64, String, String)>(
        "SELECT views, likes, first_seen_at, last_seen_at FROM servers WHERE address = ?",
    )
    .bind(&address)
    .fetch_optional(db)
    .await
    .ok()
    .flatten()?;

    Some(Popularity {
        views: row.0,
        likes: row.1,
        first_seen_at: row.2,
        last_seen_at: row.3,
    })
}
