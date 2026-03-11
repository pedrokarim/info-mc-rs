use axum::Json;
use axum::extract::{Path, Query, State};
use serde::{Deserialize, Serialize};

use mc_protocol::{SlpConfig, SlpResponse};

use crate::error::ApiError;
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
