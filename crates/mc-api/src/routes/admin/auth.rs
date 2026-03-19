use std::sync::Arc;

use axum::extract::{Extension, Query, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use jsonwebtoken::{EncodingKey, Header};
use rand::Rng;
use serde::{Deserialize, Serialize};

use crate::error::ApiError;
use crate::middleware::admin_auth::AdminClaims;
use crate::state::AppState;

// -- Discord API types --

#[derive(Deserialize)]
struct DiscordTokenResponse {
    access_token: String,
}

#[derive(Deserialize)]
struct DiscordUser {
    id: String,
    username: String,
    avatar: Option<String>,
}

// -- Request/Response types --

#[derive(Deserialize)]
pub struct CallbackParams {
    pub code: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    url: String,
    state: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    expires_at: String,
    requires_2fa: bool,
    user: AdminUserInfo,
}

#[derive(Serialize, sqlx::FromRow)]
pub struct AdminUserInfo {
    discord_id: String,
    discord_username: String,
    discord_avatar: Option<String>,
    role: String,
}

fn generate_random_hex(len: usize) -> String {
    let mut rng = rand::thread_rng();
    (0..len)
        .map(|_| format!("{:02x}", rng.r#gen::<u8>()))
        .collect()
}

/// GET /api/v1/admin/auth/login — returns Discord OAuth URL
pub async fn login(
    State(state): State<Arc<AppState>>,
) -> Result<Json<LoginResponse>, ApiError> {
    if state.discord_client_id.is_empty() {
        return Err(ApiError::InternalError(
            "Discord OAuth not configured".into(),
        ));
    }

    let state_token = generate_random_hex(16);
    let redirect_uri =
        urlencoding::encode(&state.discord_redirect_uri).into_owned();

    let url = format!(
        "https://discord.com/api/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify&state={}",
        state.discord_client_id, redirect_uri, state_token
    );

    Ok(Json(LoginResponse {
        url,
        state: state_token,
    }))
}

/// GET /api/v1/admin/auth/callback?code=...
pub async fn callback(
    State(state): State<Arc<AppState>>,
    Query(params): Query<CallbackParams>,
) -> Result<Json<AuthResponse>, ApiError> {
    if state.discord_client_id.is_empty() {
        return Err(ApiError::InternalError(
            "Discord OAuth not configured".into(),
        ));
    }

    // 1. Exchange code for access_token
    let token_resp = state
        .admin_http
        .post("https://discord.com/api/oauth2/token")
        .form(&[
            ("client_id", state.discord_client_id.as_str()),
            ("client_secret", state.discord_client_secret.as_str()),
            ("grant_type", "authorization_code"),
            ("code", params.code.as_str()),
            ("redirect_uri", state.discord_redirect_uri.as_str()),
        ])
        .send()
        .await
        .map_err(|e| ApiError::InternalError(format!("Discord token exchange failed: {e}")))?;

    if !token_resp.status().is_success() {
        let status = token_resp.status();
        let body = token_resp.text().await.unwrap_or_default();
        return Err(ApiError::InternalError(format!(
            "Discord token exchange returned {status}: {body}"
        )));
    }

    let token_data: DiscordTokenResponse = token_resp
        .json()
        .await
        .map_err(|e| ApiError::InternalError(format!("Failed to parse Discord token: {e}")))?;

    // 2. Fetch Discord user info
    let user_resp = state
        .admin_http
        .get("https://discord.com/api/users/@me")
        .header("Authorization", format!("Bearer {}", token_data.access_token))
        .send()
        .await
        .map_err(|e| ApiError::InternalError(format!("Discord user fetch failed: {e}")))?;

    if !user_resp.status().is_success() {
        return Err(ApiError::InternalError(
            "Failed to fetch Discord user info".into(),
        ));
    }

    let discord_user: DiscordUser = user_resp
        .json()
        .await
        .map_err(|e| ApiError::InternalError(format!("Failed to parse Discord user: {e}")))?;

    // 3. Check if user is an authorized admin
    let admin: Option<AdminUserInfo> = sqlx::query_as(
        "SELECT discord_id, discord_username, discord_avatar, role FROM admin_users WHERE discord_id = ?",
    )
    .bind(&discord_user.id)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let admin = admin.ok_or_else(|| {
        ApiError::Forbidden(format!(
            "Discord user {} is not an authorized admin",
            discord_user.id
        ))
    })?;

    // 4. Update admin_users with latest Discord info
    sqlx::query(
        "UPDATE admin_users SET discord_username = ?, discord_avatar = ?, last_login_at = datetime('now') WHERE discord_id = ?",
    )
    .bind(&discord_user.username)
    .bind(&discord_user.avatar)
    .bind(&discord_user.id)
    .execute(&state.db)
    .await
    .ok();

    // 5. Check if 2FA is enabled (confirmed secret without "pending:" prefix)
    let totp_secret: Option<String> = sqlx::query_scalar(
        "SELECT totp_secret FROM admin_users WHERE discord_id = ?",
    )
    .bind(&discord_user.id)
    .fetch_optional(&state.db)
    .await
    .ok()
    .flatten()
    .flatten();

    let has_2fa = totp_secret
        .as_ref()
        .is_some_and(|s| !s.is_empty() && !s.starts_with("pending:"));

    if has_2fa {
        // Return a temporary token that requires 2FA verification
        let now = chrono::Utc::now();
        let temp_expires = now + chrono::Duration::minutes(5);

        let temp_claims = AdminClaims {
            sub: discord_user.id.clone(),
            jti: "pending_2fa".to_string(),
            username: discord_user.username.clone(),
            role: "pending_2fa".to_string(),
            exp: temp_expires.timestamp() as usize,
            iat: now.timestamp() as usize,
        };

        let temp_token = jsonwebtoken::encode(
            &Header::default(),
            &temp_claims,
            &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
        )
        .map_err(|e| ApiError::InternalError(format!("JWT encoding failed: {e}")))?;

        return Ok(Json(AuthResponse {
            token: temp_token,
            expires_at: temp_expires.to_rfc3339(),
            requires_2fa: true,
            user: AdminUserInfo {
                discord_id: discord_user.id,
                discord_username: discord_user.username,
                discord_avatar: discord_user.avatar,
                role: admin.role,
            },
        }));
    }

    // 6. Create session (no 2FA)
    let session_id = generate_random_hex(32);
    let now = chrono::Utc::now();
    let expires_at = now + chrono::Duration::hours(24);

    sqlx::query(
        "INSERT INTO admin_sessions (id, discord_id, expires_at) VALUES (?, ?, ?)",
    )
    .bind(&session_id)
    .bind(&discord_user.id)
    .bind(expires_at.to_rfc3339())
    .execute(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?;

    // 7. Cleanup expired sessions
    sqlx::query("DELETE FROM admin_sessions WHERE expires_at < datetime('now')")
        .execute(&state.db)
        .await
        .ok();

    // 8. Create JWT
    let claims = AdminClaims {
        sub: discord_user.id.clone(),
        jti: session_id,
        username: discord_user.username.clone(),
        role: admin.role.clone(),
        exp: expires_at.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let token = jsonwebtoken::encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|e| ApiError::InternalError(format!("JWT encoding failed: {e}")))?;

    // 9. Audit log
    sqlx::query(
        "INSERT INTO admin_audit_log (discord_id, action) VALUES (?, 'login')",
    )
    .bind(&discord_user.id)
    .execute(&state.db)
    .await
    .ok();

    Ok(Json(AuthResponse {
        token,
        expires_at: expires_at.to_rfc3339(),
        requires_2fa: false,
        user: AdminUserInfo {
            discord_id: discord_user.id,
            discord_username: discord_user.username,
            discord_avatar: discord_user.avatar,
            role: admin.role,
        },
    }))
}

/// GET /api/v1/admin/auth/me (protected)
pub async fn me(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
) -> Result<Json<AdminUserInfo>, ApiError> {
    let user: AdminUserInfo = sqlx::query_as(
        "SELECT discord_id, discord_username, discord_avatar, role FROM admin_users WHERE discord_id = ?",
    )
    .bind(&claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| ApiError::InternalError(e.to_string()))?
    .ok_or(ApiError::Unauthorized)?;

    Ok(Json(user))
}

/// POST /api/v1/admin/auth/logout (protected)
pub async fn logout(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
) -> Result<impl IntoResponse, ApiError> {
    sqlx::query("DELETE FROM admin_sessions WHERE id = ?")
        .bind(&claims.jti)
        .execute(&state.db)
        .await
        .ok();

    sqlx::query(
        "INSERT INTO admin_audit_log (discord_id, action) VALUES (?, 'logout')",
    )
    .bind(&claims.sub)
    .execute(&state.db)
    .await
    .ok();

    Ok(StatusCode::NO_CONTENT)
}
