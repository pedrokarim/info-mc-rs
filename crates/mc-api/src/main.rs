mod error;
mod middleware;
mod routes;
mod state;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::{HeaderValue, Method};
use axum::{Extension, Router, middleware as axum_mw, routing::get};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tracing_subscriber::EnvFilter;

use crate::middleware::rate_limit::{RateLimiter, rate_limit_middleware};
use crate::state::AppState;

fn build_cors_layer() -> CorsLayer {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    match std::env::var("CORS_ORIGIN") {
        Ok(origin) if !origin.is_empty() && origin != "*" => {
            let origins: Vec<HeaderValue> = origin
                .split(',')
                .filter_map(|o| o.trim().parse().ok())
                .collect();
            cors.allow_origin(AllowOrigin::list(origins))
        }
        _ => cors.allow_origin(Any),
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let state = Arc::new(AppState::new().await);
    let rate_limiter = RateLimiter::from_env();

    let app = Router::new()
        .route("/health", get(routes::health::health))
        .route("/api/docs", get(routes::docs::api_docs))
        .route("/api/v1/server/{address}", get(routes::server::get_server))
        .route(
            "/api/v1/player/{identifier}",
            get(routes::player::get_player),
        )
        .route(
            "/api/v1/render/{identifier}",
            get(routes::render::render_skin),
        )
        .route(
            "/api/v1/render3d/{identifier}",
            get(routes::render3d::render_skin_3d),
        )
        .route(
            "/api/v1/cape/{source}/{identifier}",
            get(routes::cape_proxy::proxy_cape),
        )
        .route(
            "/api/v1/favorites",
            get(routes::favorites::list_favorites),
        )
        .route(
            "/api/v1/favorites/{uuid}",
            get(routes::favorites::is_favorite)
                .post(routes::favorites::add_favorite)
                .delete(routes::favorites::remove_favorite),
        )
        .layer(axum_mw::from_fn(rate_limit_middleware))
        .layer(Extension(rate_limiter))
        .layer(build_cors_layer())
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::X_CONTENT_TYPE_OPTIONS,
            HeaderValue::from_static("nosniff"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::X_FRAME_OPTIONS,
            HeaderValue::from_static("DENY"),
        ))
        .layer(SetResponseHeaderLayer::overriding(
            axum::http::header::CONTENT_SECURITY_POLICY,
            HeaderValue::from_static("default-src 'none'"),
        ))
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3001);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("MCInfo API listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
