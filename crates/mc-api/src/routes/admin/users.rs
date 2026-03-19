use std::sync::Arc;

use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::middleware::admin_auth::AdminClaims;
use crate::state::AppState;

fn require_super_admin(claims: &AdminClaims) -> Result<(), ApiError> {
    if claims.role != "super_admin" {
        return Err(ApiError::Forbidden("super_admin role required".into()));
    }
    Ok(())
}

#[derive(Serialize, sqlx::FromRow)]
pub struct AdminUserEntry {
    pub discord_id: String,
    pub discord_username: String,
    pub discord_avatar: Option<String>,
    pub role: String,
    pub created_at: String,
    pub last_login_at: Option<String>,
}

#[derive(Deserialize)]
pub struct AddAdminBody {
    pub discord_id: String,
    pub discord_username: String,
    #[serde(default = "default_role")]
    pub role: String,
}

fn default_role() -> String {
    "admin".to_string()
}

#[derive(Deserialize)]
pub struct UpdateAdminBody {
    pub role: Option<String>,
}

/// GET /api/v1/admin/users (protected, super_admin only)
pub async fn list_admins(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
) -> Result<Json<Vec<AdminUserEntry>>, ApiError> {
    require_super_admin(&claims)?;

    let rows: Vec<AdminUserEntry> = sqlx::query_as(
        "SELECT discord_id, discord_username, discord_avatar, role, created_at, last_login_at
         FROM admin_users ORDER BY created_at ASC",
    )
    .fetch_all(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(rows))
}

/// POST /api/v1/admin/users (protected, super_admin only)
pub async fn add_admin(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
    Json(body): Json<AddAdminBody>,
) -> Result<impl IntoResponse, ApiError> {
    require_super_admin(&claims)?;

    // Validate role
    match body.role.as_str() {
        "admin" | "super_admin" => {}
        _ => return Err(ApiError::InvalidAddress("role must be 'admin' or 'super_admin'".into())),
    }

    // Validate discord_id (numeric string)
    if body.discord_id.is_empty() || !body.discord_id.chars().all(|c| c.is_ascii_digit()) {
        return Err(ApiError::InvalidAddress("invalid discord_id".into()));
    }

    let result = sqlx::query(
        "INSERT OR IGNORE INTO admin_users (discord_id, discord_username, role) VALUES (?, ?, ?)",
    )
    .bind(&body.discord_id)
    .bind(&body.discord_username)
    .bind(&body.role)
    .execute(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::InvalidAddress("admin already exists".into()));
    }

    // Audit log
    sqlx::query(
        "INSERT INTO admin_audit_log (discord_id, action, detail) VALUES (?, 'add_admin', ?)",
    )
    .bind(&claims.sub)
    .bind(format!("added={} role={}", body.discord_id, body.role))
    .execute(&state.db)
    .await
    .ok();

    Ok(StatusCode::CREATED)
}

/// PATCH /api/v1/admin/users/{discord_id} (protected, super_admin only)
pub async fn update_admin(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
    Path(discord_id): Path<String>,
    Json(body): Json<UpdateAdminBody>,
) -> Result<impl IntoResponse, ApiError> {
    require_super_admin(&claims)?;

    if let Some(ref role) = body.role {
        match role.as_str() {
            "admin" | "super_admin" => {}
            _ => {
                return Err(ApiError::InvalidAddress(
                    "role must be 'admin' or 'super_admin'".into(),
                ))
            }
        }

        let result = sqlx::query("UPDATE admin_users SET role = ? WHERE discord_id = ?")
            .bind(role)
            .bind(&discord_id)
            .execute(&state.db)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?;

        if result.rows_affected() == 0 {
            return Err(ApiError::NotFound(format!("admin {discord_id} not found")));
        }

        sqlx::query(
            "INSERT INTO admin_audit_log (discord_id, action, detail) VALUES (?, 'update_admin', ?)",
        )
        .bind(&claims.sub)
        .bind(format!("target={discord_id} role={role}"))
        .execute(&state.db)
        .await
        .ok();
    }

    Ok(StatusCode::OK)
}

/// DELETE /api/v1/admin/users/{discord_id} (protected, super_admin only)
pub async fn delete_admin(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
    Path(discord_id): Path<String>,
) -> Result<impl IntoResponse, ApiError> {
    require_super_admin(&claims)?;

    // Prevent self-deletion
    if discord_id == claims.sub {
        return Err(ApiError::Forbidden("cannot delete yourself".into()));
    }

    let result = sqlx::query("DELETE FROM admin_users WHERE discord_id = ?")
        .bind(&discord_id)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    if result.rows_affected() == 0 {
        return Err(ApiError::NotFound(format!("admin {discord_id} not found")));
    }

    // Also revoke all their sessions
    sqlx::query("DELETE FROM admin_sessions WHERE discord_id = ?")
        .bind(&discord_id)
        .execute(&state.db)
        .await
        .ok();

    sqlx::query(
        "INSERT INTO admin_audit_log (discord_id, action, detail) VALUES (?, 'delete_admin', ?)",
    )
    .bind(&claims.sub)
    .bind(format!("deleted={discord_id}"))
    .execute(&state.db)
    .await
    .ok();

    Ok(StatusCode::NO_CONTENT)
}
