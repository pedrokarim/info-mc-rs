use std::sync::Arc;

use axum::Json;
use axum::extract::{Extension, State};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use totp_rs::{Algorithm, Secret, TOTP};

use crate::error::ApiError;
use crate::middleware::admin_auth::AdminClaims;
use crate::state::AppState;

fn build_totp(secret: &str, username: &str) -> Result<TOTP, ApiError> {
    let secret_bytes = Secret::Encoded(secret.to_string())
        .to_bytes()
        .map_err(|e| ApiError::InternalError(format!("invalid TOTP secret: {e}")))?;

    TOTP::new(
        Algorithm::SHA1,
        6,
        1,
        30,
        secret_bytes,
        Some("MCInfo".to_string()),
        username.to_string(),
    )
    .map_err(|e| ApiError::InternalError(format!("TOTP error: {e}")))
}

// -- Setup: generate secret --

#[derive(Serialize)]
pub struct TotpSetupResponse {
    pub secret: String,
    pub otpauth_uri: String,
}

/// POST /api/v1/admin/auth/2fa/setup (protected)
/// Generates a TOTP secret. Does NOT enable 2FA yet — call /confirm with a valid code.
pub async fn setup_2fa(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
) -> Result<Json<TotpSetupResponse>, ApiError> {
    // Check if already enabled
    let existing: Option<String> =
        sqlx::query_scalar("SELECT totp_secret FROM admin_users WHERE discord_id = ?")
            .bind(&claims.sub)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?
            .flatten();

    if existing.is_some() {
        return Err(ApiError::InvalidAddress(
            "2FA is already enabled. Disable it first to reconfigure.".into(),
        ));
    }

    let secret = Secret::generate_secret();
    let secret_encoded = secret.to_encoded().to_string();
    let totp = build_totp(&secret_encoded, &claims.username)?;
    let otpauth_uri = totp.get_url();

    // Store temporarily in a pending state — we'll use the session to track it
    // For simplicity, store it directly but mark as unconfirmed by using a prefix
    sqlx::query("UPDATE admin_users SET totp_secret = ? WHERE discord_id = ?")
        .bind(format!("pending:{secret_encoded}"))
        .bind(&claims.sub)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    Ok(Json(TotpSetupResponse {
        secret: secret_encoded,
        otpauth_uri,
    }))
}

// -- Confirm: validate code to activate 2FA --

#[derive(Deserialize)]
pub struct TotpCodeBody {
    pub code: String,
}

/// POST /api/v1/admin/auth/2fa/confirm (protected)
/// Confirms 2FA setup by validating a TOTP code against the pending secret.
pub async fn confirm_2fa(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
    Json(body): Json<TotpCodeBody>,
) -> Result<impl IntoResponse, ApiError> {
    let stored: Option<String> =
        sqlx::query_scalar("SELECT totp_secret FROM admin_users WHERE discord_id = ?")
            .bind(&claims.sub)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?
            .flatten();

    let stored = stored.ok_or(ApiError::InvalidAddress("no 2FA setup in progress".into()))?;

    let secret = stored
        .strip_prefix("pending:")
        .ok_or(ApiError::InvalidAddress(
            "2FA is already confirmed. Disable it first to reconfigure.".into(),
        ))?;

    let totp = build_totp(secret, &claims.username)?;

    if !totp
        .check_current(&body.code)
        .map_err(|e| ApiError::InternalError(e.to_string()))?
    {
        return Err(ApiError::InvalidAddress("invalid TOTP code".into()));
    }

    // Confirm: remove the "pending:" prefix
    sqlx::query("UPDATE admin_users SET totp_secret = ? WHERE discord_id = ?")
        .bind(secret)
        .bind(&claims.sub)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    sqlx::query("INSERT INTO admin_audit_log (discord_id, action) VALUES (?, '2fa_enabled')")
        .bind(&claims.sub)
        .execute(&state.db)
        .await
        .ok();

    Ok(StatusCode::OK)
}

// -- Disable 2FA --

/// DELETE /api/v1/admin/auth/2fa (protected)
pub async fn disable_2fa(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
) -> Result<impl IntoResponse, ApiError> {
    sqlx::query("UPDATE admin_users SET totp_secret = NULL WHERE discord_id = ?")
        .bind(&claims.sub)
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    sqlx::query("INSERT INTO admin_audit_log (discord_id, action) VALUES (?, '2fa_disabled')")
        .bind(&claims.sub)
        .execute(&state.db)
        .await
        .ok();

    Ok(StatusCode::NO_CONTENT)
}

// -- Verify: used during login when 2FA is enabled --

#[derive(Deserialize)]
pub struct TotpVerifyBody {
    pub temp_token: String,
    pub code: String,
}

#[derive(Serialize)]
pub struct TotpVerifyResponse {
    pub token: String,
    pub expires_at: String,
}

/// POST /api/v1/admin/auth/2fa/verify (public — temp_token is the auth)
/// Exchanges a temporary 2FA-pending token + TOTP code for a full JWT session.
pub async fn verify_2fa(
    State(state): State<Arc<AppState>>,
    Json(body): Json<TotpVerifyBody>,
) -> Result<Json<TotpVerifyResponse>, ApiError> {
    // Decode the temp token
    let temp_claims = jsonwebtoken::decode::<AdminClaims>(
        &body.temp_token,
        &jsonwebtoken::DecodingKey::from_secret(state.jwt_secret.as_bytes()),
        &jsonwebtoken::Validation::default(),
    )
    .map_err(|_| ApiError::Unauthorized)?
    .claims;

    // The temp token must have role "pending_2fa"
    if temp_claims.role != "pending_2fa" {
        return Err(ApiError::Unauthorized);
    }

    // Fetch TOTP secret
    let totp_secret: Option<String> =
        sqlx::query_scalar("SELECT totp_secret FROM admin_users WHERE discord_id = ?")
            .bind(&temp_claims.sub)
            .fetch_optional(&state.db)
            .await
            .map_err(|e| ApiError::InternalError(e.to_string()))?
            .flatten();

    let secret = totp_secret.ok_or(ApiError::Unauthorized)?;

    // Must be confirmed (no "pending:" prefix)
    if secret.starts_with("pending:") {
        return Err(ApiError::InvalidAddress(
            "2FA setup not confirmed yet".into(),
        ));
    }

    let totp = build_totp(&secret, &temp_claims.username)?;

    if !totp
        .check_current(&body.code)
        .map_err(|e| ApiError::InternalError(e.to_string()))?
    {
        return Err(ApiError::InvalidAddress("invalid TOTP code".into()));
    }

    // Fetch the real role
    let role: String = sqlx::query_scalar("SELECT role FROM admin_users WHERE discord_id = ?")
        .bind(&temp_claims.sub)
        .fetch_one(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    // Create a real session
    let session_id = {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        (0..32)
            .map(|_| format!("{:02x}", rng.r#gen::<u8>()))
            .collect::<String>()
    };
    let now = chrono::Utc::now();
    let expires_at = now + chrono::Duration::hours(24);

    sqlx::query("INSERT INTO admin_sessions (id, discord_id, expires_at) VALUES (?, ?, ?)")
        .bind(&session_id)
        .bind(&temp_claims.sub)
        .bind(expires_at.to_rfc3339())
        .execute(&state.db)
        .await
        .map_err(|e| ApiError::InternalError(e.to_string()))?;

    let claims = AdminClaims {
        sub: temp_claims.sub.clone(),
        jti: session_id,
        username: temp_claims.username,
        role,
        exp: expires_at.timestamp() as usize,
        iat: now.timestamp() as usize,
    };

    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::default(),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|e| ApiError::InternalError(format!("JWT encoding failed: {e}")))?;

    sqlx::query("INSERT INTO admin_audit_log (discord_id, action) VALUES (?, '2fa_verified')")
        .bind(&temp_claims.sub)
        .execute(&state.db)
        .await
        .ok();

    Ok(Json(TotpVerifyResponse {
        token,
        expires_at: expires_at.to_rfc3339(),
    }))
}
