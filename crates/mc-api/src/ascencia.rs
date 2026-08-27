use aes_gcm::aead::{Aead, KeyInit};
use aes_gcm::{Aes256Gcm, Nonce};
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE_NO_PAD;
use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::{Algorithm, DecodingKey, Validation, decode, decode_header};
use rand::RngCore;
use reqwest::Url;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use sqlx::FromRow;

use crate::error::ApiError;
use crate::state::AppState;

pub const SESSION_COOKIE: &str = "mcinfo_ascencia_session";
const APPLICATION_SLUG: &str = "mcinfo";
const ADMIN_ROLE: &str = "admin";
const PLATFORM_SUPERADMIN_ROLE: &str = "superadmin";
const SESSION_TTL_DAYS: i64 = 7;

#[derive(Clone)]
pub struct AscenciaConfig {
    pub issuer: String,
    pub client_id: String,
    pub client_secret: String,
    pub redirect_uri: String,
    pub allowed_origin: String,
    pub session_secret: String,
    pub cookie_secure: bool,
}

impl AscenciaConfig {
    pub fn from_env() -> Self {
        let issuer = std::env::var("ASCENCIA_ISSUER")
            .unwrap_or_else(|_| "https://id.ascencia.re".to_string())
            .trim_end_matches('/')
            .to_string();
        let redirect_uri = std::env::var("ASCENCIA_REDIRECT_URI")
            .unwrap_or_else(|_| "http://localhost:3000/admin/login".to_string());
        let allowed_origin = std::env::var("ASCENCIA_ALLOWED_ORIGIN").unwrap_or_else(|_| {
            Url::parse(&redirect_uri)
                .map(|url| url.origin().ascii_serialization())
                .unwrap_or_else(|_| "http://localhost:3000".to_string())
        });
        let cookie_secure = std::env::var("ASCENCIA_COOKIE_SECURE")
            .map(|value| value != "false" && value != "0")
            .unwrap_or_else(|_| allowed_origin.starts_with("https://"));

        Self {
            issuer,
            client_id: std::env::var("ASCENCIA_CLIENT_ID").unwrap_or_default(),
            client_secret: std::env::var("ASCENCIA_CLIENT_SECRET").unwrap_or_default(),
            redirect_uri,
            allowed_origin,
            session_secret: std::env::var("ASCENCIA_SESSION_SECRET").unwrap_or_default(),
            cookie_secure,
        }
    }

    pub fn is_configured(&self) -> bool {
        !self.client_id.is_empty()
            && !self.client_secret.is_empty()
            && !self.session_secret.is_empty()
    }

    pub fn accepts_redirect_uri(&self, redirect_uri: &str) -> bool {
        redirect_uri == self.redirect_uri
            || redirect_uri == format!("{}/embed/callback", self.issuer)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AscenciaClaims {
    pub iss: String,
    pub sub: String,
    pub aud: serde_json::Value,
    pub exp: i64,
    pub iat: i64,
    pub app: Option<String>,
    #[serde(default)]
    pub roles: Vec<String>,
    #[serde(default)]
    pub prole: Vec<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct AscenciaProfile {
    #[serde(rename = "sub")]
    pub account_id: String,
    pub email: Option<String>,
    #[serde(rename = "name")]
    pub display_name: Option<String>,
    #[serde(rename = "preferred_username")]
    pub username: Option<String>,
    #[serde(rename = "picture")]
    pub avatar_url: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AdminUserInfo {
    pub account_id: String,
    pub email: Option<String>,
    pub display_name: Option<String>,
    pub username: Option<String>,
    pub avatar_url: Option<String>,
    pub role: String,
}

impl AdminUserInfo {
    pub fn from_identity(profile: AscenciaProfile, claims: &AscenciaClaims) -> Self {
        Self {
            account_id: profile.account_id,
            email: profile.email,
            display_name: profile.display_name,
            username: profile.username,
            avatar_url: profile.avatar_url,
            role: local_role(claims).to_string(),
        }
    }
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: String,
    expires_in: i64,
    refresh_token: Option<String>,
}

#[derive(FromRow)]
struct StoredSession {
    token_hash: String,
    account_id: String,
    refresh_token: Option<String>,
    access_expires_at: String,
    session_expires_at: String,
    claims: String,
    profile: String,
}

pub struct CreatedSession {
    pub token: String,
    pub user: AdminUserInfo,
}

pub struct LoadedSession {
    pub token_hash: String,
    pub user: AdminUserInfo,
}

pub fn can_access_admin(claims: &AscenciaClaims) -> bool {
    claims.app.as_deref() == Some(APPLICATION_SLUG)
        && (claims.roles.iter().any(|role| role == ADMIN_ROLE)
            || claims
                .prole
                .iter()
                .any(|role| role == PLATFORM_SUPERADMIN_ROLE))
}

fn local_role(claims: &AscenciaClaims) -> &'static str {
    if claims
        .prole
        .iter()
        .any(|role| role == PLATFORM_SUPERADMIN_ROLE)
    {
        "super_admin"
    } else {
        "admin"
    }
}

pub async fn create_session(
    state: &AppState,
    code: &str,
    code_verifier: &str,
    redirect_uri: &str,
) -> Result<CreatedSession, ApiError> {
    ensure_configured(&state.ascencia)?;

    let tokens = request_tokens(
        state,
        &[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("code_verifier", code_verifier),
            ("redirect_uri", redirect_uri),
        ],
    )
    .await?;
    let claims = verify_access_token(state, &tokens.access_token).await?;
    if !can_access_admin(&claims) {
        return Err(ApiError::Forbidden(
            "Ce compte n’est pas autorisé à administrer MCInfo.".into(),
        ));
    }
    let profile = load_profile(state, &tokens.access_token).await?;
    if profile.account_id != claims.sub {
        return Err(ApiError::Unauthorized);
    }

    let mut raw_token = [0_u8; 32];
    rand::thread_rng().fill_bytes(&mut raw_token);
    let raw_token = URL_SAFE_NO_PAD.encode(raw_token);
    let token_hash = hash_token(&raw_token);
    let now = Utc::now();
    let access_expires_at = now + Duration::seconds(tokens.expires_in);
    let session_expires_at = now + Duration::days(SESSION_TTL_DAYS);
    let user = AdminUserInfo::from_identity(profile.clone(), &claims);

    sqlx::query(
        "INSERT INTO ascencia_admin_sessions
         (token_hash, account_id, access_token, refresh_token, access_expires_at,
          session_expires_at, claims, profile)
         VALUES (?, ?, ?, ?, ?, ?, ?, ?)",
    )
    .bind(&token_hash)
    .bind(&profile.account_id)
    .bind(encrypt(&state.ascencia, &tokens.access_token)?)
    .bind(
        tokens
            .refresh_token
            .as_deref()
            .map(|token| encrypt(&state.ascencia, token))
            .transpose()?,
    )
    .bind(access_expires_at.to_rfc3339())
    .bind(session_expires_at.to_rfc3339())
    .bind(serde_json::to_string(&claims).map_err(internal_error)?)
    .bind(serde_json::to_string(&profile).map_err(internal_error)?)
    .execute(&state.db)
    .await
    .map_err(internal_error)?;

    sqlx::query(
        "DELETE FROM ascencia_admin_sessions
         WHERE datetime(session_expires_at) < datetime('now')",
    )
    .execute(&state.db)
    .await
    .ok();

    Ok(CreatedSession {
        token: raw_token,
        user,
    })
}

pub async fn load_session(state: &AppState, raw_token: &str) -> Result<LoadedSession, ApiError> {
    let token_hash = hash_token(raw_token);
    let mut stored = find_session(state, &token_hash)
        .await?
        .ok_or(ApiError::Unauthorized)?;

    let session_expires_at = parse_time(&stored.session_expires_at)?;
    if session_expires_at <= Utc::now() {
        delete_session(state, &token_hash).await;
        return Err(ApiError::Unauthorized);
    }

    if parse_time(&stored.access_expires_at)? <= Utc::now() + Duration::seconds(30) {
        let _guard = state.ascencia_refresh_lock.lock().await;
        stored = find_session(state, &token_hash)
            .await?
            .ok_or(ApiError::Unauthorized)?;
        if parse_time(&stored.access_expires_at)? <= Utc::now() + Duration::seconds(30) {
            stored = refresh_session(state, stored).await.map_err(|error| {
                tracing::warn!(account_id = %error.0, "Ascencia session refresh failed");
                ApiError::Unauthorized
            })?;
        }
    }

    let claims: AscenciaClaims = serde_json::from_str(&stored.claims).map_err(internal_error)?;
    if !can_access_admin(&claims) {
        delete_session(state, &token_hash).await;
        return Err(ApiError::Forbidden(
            "Ce compte n’est plus autorisé à administrer MCInfo.".into(),
        ));
    }
    let profile: AscenciaProfile = serde_json::from_str(&stored.profile).map_err(internal_error)?;

    Ok(LoadedSession {
        token_hash,
        user: AdminUserInfo::from_identity(profile, &claims),
    })
}

async fn refresh_session(
    state: &AppState,
    stored: StoredSession,
) -> Result<StoredSession, (String, ApiError)> {
    let account_id = stored.account_id.clone();
    let refresh_token = stored
        .refresh_token
        .as_deref()
        .ok_or_else(|| (account_id.clone(), ApiError::Unauthorized))
        .and_then(|token| {
            decrypt(&state.ascencia, token).map_err(|error| (account_id.clone(), error))
        })?;
    let tokens = request_tokens(
        state,
        &[
            ("grant_type", "refresh_token"),
            ("refresh_token", &refresh_token),
        ],
    )
    .await
    .map_err(|error| (account_id.clone(), error))?;
    let claims = verify_access_token(state, &tokens.access_token)
        .await
        .map_err(|error| (account_id.clone(), error))?;
    if !can_access_admin(&claims) {
        delete_session(state, &stored.token_hash).await;
        return Err((account_id, ApiError::Unauthorized));
    }

    let next_refresh_token = tokens.refresh_token.as_deref().unwrap_or(&refresh_token);
    let access_expires_at = Utc::now() + Duration::seconds(tokens.expires_in);
    sqlx::query(
        "UPDATE ascencia_admin_sessions
         SET access_token = ?, refresh_token = ?, access_expires_at = ?, claims = ?,
             updated_at = datetime('now')
         WHERE token_hash = ?",
    )
    .bind(encrypt(&state.ascencia, &tokens.access_token).map_err(|e| (account_id.clone(), e))?)
    .bind(encrypt(&state.ascencia, next_refresh_token).map_err(|e| (account_id.clone(), e))?)
    .bind(access_expires_at.to_rfc3339())
    .bind(serde_json::to_string(&claims).map_err(|e| (account_id.clone(), internal_error(e)))?)
    .bind(&stored.token_hash)
    .execute(&state.db)
    .await
    .map_err(|e| (account_id.clone(), internal_error(e)))?;

    find_session(state, &stored.token_hash)
        .await
        .map_err(|e| (account_id.clone(), e))?
        .ok_or((account_id, ApiError::Unauthorized))
}

async fn find_session(
    state: &AppState,
    token_hash: &str,
) -> Result<Option<StoredSession>, ApiError> {
    sqlx::query_as(
        "SELECT token_hash, account_id, refresh_token, access_expires_at,
                session_expires_at, claims, profile
         FROM ascencia_admin_sessions WHERE token_hash = ?",
    )
    .bind(token_hash)
    .fetch_optional(&state.db)
    .await
    .map_err(internal_error)
}

pub async fn delete_session(state: &AppState, token_hash: &str) {
    sqlx::query("DELETE FROM ascencia_admin_sessions WHERE token_hash = ?")
        .bind(token_hash)
        .execute(&state.db)
        .await
        .ok();
}

async fn request_tokens(
    state: &AppState,
    fields: &[(&str, &str)],
) -> Result<TokenResponse, ApiError> {
    let mut form = fields.to_vec();
    form.push(("client_id", &state.ascencia.client_id));
    form.push(("client_secret", &state.ascencia.client_secret));
    let response = state
        .admin_http
        .post(format!("{}/oauth/token", state.ascencia.issuer))
        .form(&form)
        .send()
        .await
        .map_err(internal_error)?;
    if !response.status().is_success() {
        tracing::warn!(status = %response.status(), "Ascencia token exchange refused");
        return Err(ApiError::Unauthorized);
    }
    response.json().await.map_err(internal_error)
}

async fn verify_access_token(
    state: &AppState,
    access_token: &str,
) -> Result<AscenciaClaims, ApiError> {
    let header = decode_header(access_token).map_err(|_| ApiError::Unauthorized)?;
    if header.alg != Algorithm::ES256 {
        return Err(ApiError::Unauthorized);
    }
    let key_id = header.kid.ok_or(ApiError::Unauthorized)?;
    let jwks: JwkSet = state
        .admin_http
        .get(format!("{}/.well-known/jwks.json", state.ascencia.issuer))
        .send()
        .await
        .map_err(internal_error)?
        .error_for_status()
        .map_err(internal_error)?
        .json()
        .await
        .map_err(internal_error)?;
    let jwk = jwks.find(&key_id).ok_or(ApiError::Unauthorized)?;
    let key = DecodingKey::from_jwk(jwk).map_err(|_| ApiError::Unauthorized)?;
    let mut validation = Validation::new(Algorithm::ES256);
    validation.leeway = 5;
    validation.set_audience(&[&state.ascencia.client_id]);
    validation.set_issuer(&[&state.ascencia.issuer]);
    validation.set_required_spec_claims(&["exp", "iss", "aud", "sub"]);
    decode::<AscenciaClaims>(access_token, &key, &validation)
        .map(|data| data.claims)
        .map_err(|_| ApiError::Unauthorized)
}

async fn load_profile(state: &AppState, access_token: &str) -> Result<AscenciaProfile, ApiError> {
    state
        .admin_http
        .get(format!("{}/oauth/userinfo", state.ascencia.issuer))
        .bearer_auth(access_token)
        .send()
        .await
        .map_err(internal_error)?
        .error_for_status()
        .map_err(|_| ApiError::Unauthorized)?
        .json()
        .await
        .map_err(internal_error)
}

fn encrypt(config: &AscenciaConfig, value: &str) -> Result<String, ApiError> {
    let key = Sha256::digest(config.session_secret.as_bytes());
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(internal_error)?;
    let mut nonce = [0_u8; 12];
    rand::thread_rng().fill_bytes(&mut nonce);
    let ciphertext = cipher
        .encrypt(Nonce::from_slice(&nonce), value.as_bytes())
        .map_err(internal_error)?;
    Ok(format!(
        "v1.{}.{}",
        URL_SAFE_NO_PAD.encode(nonce),
        URL_SAFE_NO_PAD.encode(ciphertext)
    ))
}

fn decrypt(config: &AscenciaConfig, value: &str) -> Result<String, ApiError> {
    let mut parts = value.split('.');
    if parts.next() != Some("v1") {
        return Err(ApiError::Unauthorized);
    }
    let nonce = parts
        .next()
        .and_then(|value| URL_SAFE_NO_PAD.decode(value).ok())
        .ok_or(ApiError::Unauthorized)?;
    let ciphertext = parts
        .next()
        .and_then(|value| URL_SAFE_NO_PAD.decode(value).ok())
        .ok_or(ApiError::Unauthorized)?;
    if parts.next().is_some() || nonce.len() != 12 {
        return Err(ApiError::Unauthorized);
    }
    let key = Sha256::digest(config.session_secret.as_bytes());
    let cipher = Aes256Gcm::new_from_slice(&key).map_err(internal_error)?;
    let plaintext = cipher
        .decrypt(Nonce::from_slice(&nonce), ciphertext.as_ref())
        .map_err(|_| ApiError::Unauthorized)?;
    String::from_utf8(plaintext).map_err(|_| ApiError::Unauthorized)
}

pub fn hash_token(token: &str) -> String {
    format!("{:x}", Sha256::digest(token.as_bytes()))
}

pub fn session_cookie(token: &str, secure: bool) -> String {
    format!(
        "{SESSION_COOKIE}={token}; Path=/; Max-Age={}; HttpOnly; SameSite=Lax{}",
        SESSION_TTL_DAYS * 86_400,
        if secure { "; Secure" } else { "" }
    )
}

pub fn clear_session_cookie(secure: bool) -> String {
    format!(
        "{SESSION_COOKIE}=; Path=/; Max-Age=0; HttpOnly; SameSite=Lax{}",
        if secure { "; Secure" } else { "" }
    )
}

pub fn cookie_value(headers: &axum::http::HeaderMap) -> Option<&str> {
    headers
        .get(axum::http::header::COOKIE)?
        .to_str()
        .ok()?
        .split(';')
        .map(str::trim)
        .find_map(|part| part.strip_prefix(&format!("{SESSION_COOKIE}=")))
        .filter(|value| !value.is_empty())
}

fn ensure_configured(config: &AscenciaConfig) -> Result<(), ApiError> {
    if config.is_configured() {
        Ok(())
    } else {
        Err(ApiError::InternalError(
            "Ascencia ID is not configured".into(),
        ))
    }
}

fn parse_time(value: &str) -> Result<DateTime<Utc>, ApiError> {
    DateTime::parse_from_rfc3339(value)
        .map(|value| value.with_timezone(&Utc))
        .map_err(internal_error)
}

fn internal_error(error: impl std::fmt::Display) -> ApiError {
    ApiError::InternalError(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn claims(app: &str, roles: &[&str], platform_roles: &[&str]) -> AscenciaClaims {
        AscenciaClaims {
            iss: "https://id.ascencia.re".into(),
            sub: "account".into(),
            aud: serde_json::Value::String("client".into()),
            exp: 1,
            iat: 1,
            app: Some(app.into()),
            roles: roles.iter().map(|role| (*role).into()).collect(),
            prole: platform_roles.iter().map(|role| (*role).into()).collect(),
        }
    }

    #[test]
    fn accepts_application_admins_and_platform_superadmins() {
        assert!(can_access_admin(&claims("mcinfo", &["admin"], &[])));
        assert!(can_access_admin(&claims("mcinfo", &[], &["superadmin"])));
    }

    #[test]
    fn refuses_other_apps_and_unprivileged_members() {
        assert!(!can_access_admin(&claims("cardmyanime", &["admin"], &[])));
        assert!(!can_access_admin(&claims("mcinfo", &["member"], &[])));
    }

    #[test]
    fn encrypts_tokens_with_authenticated_encryption() {
        let config = AscenciaConfig {
            issuer: String::new(),
            client_id: String::new(),
            client_secret: String::new(),
            redirect_uri: String::new(),
            allowed_origin: String::new(),
            session_secret: "secret de test".into(),
            cookie_secure: false,
        };
        let encrypted = encrypt(&config, "access-token").unwrap();
        assert_ne!(encrypted, "access-token");
        assert_eq!(decrypt(&config, &encrypted).unwrap(), "access-token");
    }
}
