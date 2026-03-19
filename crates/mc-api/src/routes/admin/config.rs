use std::collections::HashMap;
use std::sync::Arc;
use std::sync::atomic::Ordering;

use axum::extract::{Extension, State};
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::middleware::admin_auth::AdminClaims;
use crate::state::AppState;

#[derive(Serialize, sqlx::FromRow)]
pub struct ConfigEntry {
    pub key: String,
    pub value: String,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct UpdateConfigBody {
    /// Key-value pairs to update
    pub values: HashMap<String, String>,
}

/// Allowed config keys (prevents arbitrary data injection)
const ALLOWED_KEYS: &[&str] = &[
    "maintenance_mode",
    "maintenance_message",
    "like_alert_threshold",
    "admin_ip_whitelist",
];

/// GET /api/v1/admin/config (protected)
pub async fn get_config(
    State(state): State<Arc<AppState>>,
    Extension(_claims): Extension<AdminClaims>,
) -> Result<Json<Vec<ConfigEntry>>, ApiError> {
    let rows: Vec<ConfigEntry> = sqlx::query_as(
        "SELECT key, value, updated_at FROM admin_config ORDER BY key ASC",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(rows))
}

/// PATCH /api/v1/admin/config (protected)
pub async fn update_config(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
    Json(body): Json<UpdateConfigBody>,
) -> Result<Json<Vec<ConfigEntry>>, ApiError> {
    if body.values.is_empty() {
        return Err(ApiError::InvalidAddress("no values provided".into()));
    }

    let mut updated_keys: Vec<String> = Vec::new();

    for (key, value) in &body.values {
        if !ALLOWED_KEYS.contains(&key.as_str()) {
            return Err(ApiError::InvalidAddress(format!("unknown config key: {key}")));
        }

        // Validate specific keys
        match key.as_str() {
            "maintenance_mode" => {
                if value != "true" && value != "false" {
                    return Err(ApiError::InvalidAddress(
                        "maintenance_mode must be 'true' or 'false'".into(),
                    ));
                }
            }
            "like_alert_threshold" => {
                if value.parse::<i64>().is_err() {
                    return Err(ApiError::InvalidAddress(
                        "like_alert_threshold must be a number".into(),
                    ));
                }
            }
            _ => {}
        }

        sqlx::query(
            "INSERT INTO admin_config (key, value, updated_at) VALUES (?, ?, datetime('now'))
             ON CONFLICT(key) DO UPDATE SET value = excluded.value, updated_at = datetime('now')",
        )
        .bind(key)
        .bind(value)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

        updated_keys.push(format!("{key}={value}"));

        // Sync in-memory flags
        if key == "maintenance_mode" {
            state
                .maintenance_mode
                .store(value == "true", Ordering::Relaxed);
        }
    }

    // Audit log
    sqlx::query(
        "INSERT INTO admin_audit_log (discord_id, action, detail) VALUES (?, 'update_config', ?)",
    )
    .bind(&claims.sub)
    .bind(updated_keys.join(", "))
    .execute(&state.db)
    .await
    .ok();

    // Return full config after update
    get_config(State(state), Extension(claims)).await
}
