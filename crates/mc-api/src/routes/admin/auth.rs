use std::sync::Arc;

use axum::Json;
use axum::extract::{Extension, State};
use axum::http::header::SET_COOKIE;
use axum::http::{HeaderMap, HeaderValue, StatusCode};
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::ascencia::{
    AdminUserInfo, clear_session_cookie, create_session, delete_session, session_cookie,
};
use crate::error::ApiError;
use crate::middleware::admin_auth::AdminClaims;
use crate::state::AppState;

#[derive(Deserialize)]
pub struct ExchangeBody {
    pub code: String,
    pub code_verifier: String,
    pub redirect_uri: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    authenticated: bool,
    user: AdminUserInfo,
}

/// POST /api/v1/admin/auth/exchange — échange le code du widget côté serveur.
pub async fn exchange(
    State(state): State<Arc<AppState>>,
    headers: HeaderMap,
    Json(body): Json<ExchangeBody>,
) -> Result<impl IntoResponse, ApiError> {
    if body.code.is_empty() || body.code_verifier.is_empty() {
        return Err(ApiError::InvalidAddress(
            "code and code_verifier are required".into(),
        ));
    }
    if !state.ascencia.accepts_redirect_uri(&body.redirect_uri) {
        return Err(ApiError::InvalidAddress("invalid redirect_uri".into()));
    }
    if let Some(origin) = headers
        .get(axum::http::header::ORIGIN)
        .and_then(|value| value.to_str().ok())
    {
        if origin != state.ascencia.allowed_origin {
            return Err(ApiError::Forbidden("invalid origin".into()));
        }
    }

    let session =
        create_session(&state, &body.code, &body.code_verifier, &body.redirect_uri).await?;

    sqlx::query("INSERT INTO admin_audit_log (account_id, action) VALUES (?, 'login')")
        .bind(&session.user.account_id)
        .execute(&state.db)
        .await
        .ok();

    let cookie = session_cookie(&session.token, state.ascencia.cookie_secure);
    let mut response = Json(AuthResponse {
        authenticated: true,
        user: session.user,
    })
    .into_response();
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie)
            .map_err(|error| ApiError::InternalError(error.to_string()))?,
    );
    response.headers_mut().insert(
        axum::http::header::CACHE_CONTROL,
        HeaderValue::from_static("no-store"),
    );
    Ok(response)
}

/// GET /api/v1/admin/auth/me — renvoie la session Ascencia courante.
pub async fn me(Extension(claims): Extension<AdminClaims>) -> impl IntoResponse {
    (
        [(axum::http::header::CACHE_CONTROL, "no-store")],
        Json(AuthResponse {
            authenticated: true,
            user: claims.user,
        }),
    )
}

/// POST /api/v1/admin/auth/logout — révoque la session applicative.
pub async fn logout(
    State(state): State<Arc<AppState>>,
    Extension(claims): Extension<AdminClaims>,
) -> Result<impl IntoResponse, ApiError> {
    delete_session(&state, &claims.session_token_hash).await;
    sqlx::query("INSERT INTO admin_audit_log (account_id, action) VALUES (?, 'logout')")
        .bind(&claims.sub)
        .execute(&state.db)
        .await
        .ok();

    let cookie = clear_session_cookie(state.ascencia.cookie_secure);
    let mut response = StatusCode::NO_CONTENT.into_response();
    response.headers_mut().insert(
        SET_COOKIE,
        HeaderValue::from_str(&cookie)
            .map_err(|error| ApiError::InternalError(error.to_string()))?,
    );
    Ok(response)
}
