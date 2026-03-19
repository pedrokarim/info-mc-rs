mod error;
mod middleware;
mod routes;
mod state;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::http::{HeaderValue, Method};
use axum::{Extension, Router, middleware as axum_mw, routing::{get, post}};
use tower_http::cors::{AllowOrigin, Any, CorsLayer};
use tower_http::set_header::SetResponseHeaderLayer;
use tracing_subscriber::EnvFilter;

use crate::middleware::admin_auth::admin_auth_middleware;
use crate::middleware::maintenance::maintenance_middleware;
use crate::middleware::rate_limit::{RateLimiter, rate_limit_middleware};
use crate::state::AppState;

fn build_cors_layer() -> CorsLayer {
    let cors = CorsLayer::new()
        .allow_methods([Method::GET, Method::POST, Method::PATCH, Method::DELETE])
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
    // Load .env file if present (silently ignore if missing)
    dotenvy::dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let state = Arc::new(AppState::new().await);
    let rate_limiter = RateLimiter::from_env();

    // Admin routes — public (no auth required)
    let admin_public = Router::new()
        .route(
            "/api/v1/admin/auth/login",
            get(routes::admin::auth::login),
        )
        .route(
            "/api/v1/admin/auth/callback",
            get(routes::admin::auth::callback),
        )
        .route(
            "/api/v1/admin/auth/2fa/verify",
            post(routes::admin::totp::verify_2fa),
        );

    // Admin routes — protected (JWT auth required)
    let admin_protected = Router::new()
        .route(
            "/api/v1/admin/auth/me",
            get(routes::admin::auth::me),
        )
        .route(
            "/api/v1/admin/auth/logout",
            post(routes::admin::auth::logout),
        )
        // 2FA management (protected)
        .route(
            "/api/v1/admin/auth/2fa/setup",
            post(routes::admin::totp::setup_2fa),
        )
        .route(
            "/api/v1/admin/auth/2fa/confirm",
            post(routes::admin::totp::confirm_2fa),
        )
        .route(
            "/api/v1/admin/auth/2fa",
            axum::routing::delete(routes::admin::totp::disable_2fa),
        )
        .route(
            "/api/v1/admin/dashboard",
            get(routes::admin::dashboard::dashboard),
        )
        // Players — list + moderation
        .route(
            "/api/v1/admin/players",
            get(routes::admin::players::list_players),
        )
        .route(
            "/api/v1/admin/players/{uuid}",
            axum::routing::patch(routes::admin::players::moderate_player)
                .delete(routes::admin::players::delete_player),
        )
        // Servers — list + moderation
        .route(
            "/api/v1/admin/servers",
            get(routes::admin::servers::list_servers),
        )
        .route(
            "/api/v1/admin/servers/{address}",
            axum::routing::patch(routes::admin::servers::moderate_server)
                .delete(routes::admin::servers::delete_server),
        )
        // Admin users — CRUD (super_admin only, enforced in handlers)
        .route(
            "/api/v1/admin/users",
            get(routes::admin::users::list_admins)
                .post(routes::admin::users::add_admin),
        )
        .route(
            "/api/v1/admin/users/{discord_id}",
            axum::routing::patch(routes::admin::users::update_admin)
                .delete(routes::admin::users::delete_admin),
        )
        // Audit log
        .route(
            "/api/v1/admin/audit",
            get(routes::admin::audit::list_audit),
        )
        // Config
        .route(
            "/api/v1/admin/config",
            get(routes::admin::config::get_config)
                .patch(routes::admin::config::update_config),
        )
        // Alerts
        .route(
            "/api/v1/admin/alerts",
            get(routes::admin::alerts::list_alerts),
        )
        .route(
            "/api/v1/admin/alerts/{id}",
            axum::routing::patch(routes::admin::alerts::resolve_alert),
        )
        // Analytics
        .route(
            "/api/v1/admin/analytics/growth",
            get(routes::admin::analytics::growth),
        )
        .route(
            "/api/v1/admin/analytics/activity",
            get(routes::admin::analytics::activity),
        )
        .route(
            "/api/v1/admin/analytics/top",
            get(routes::admin::analytics::top),
        )
        // Export CSV
        .route(
            "/api/v1/admin/export/players",
            get(routes::admin::export::export_players),
        )
        .route(
            "/api/v1/admin/export/servers",
            get(routes::admin::export::export_servers),
        )
        .layer(axum_mw::from_fn_with_state(
            state.clone(),
            admin_auth_middleware,
        ));

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
        // Popular & Recent
        .route(
            "/api/v1/popular/players",
            get(routes::popular::popular_players),
        )
        .route(
            "/api/v1/popular/servers",
            get(routes::popular::popular_servers),
        )
        .route(
            "/api/v1/recent/players",
            get(routes::popular::recent_players),
        )
        .route(
            "/api/v1/recent/servers",
            get(routes::popular::recent_servers),
        )
        // Likes — players
        .route(
            "/api/v1/player/{uuid}/like",
            get(routes::likes::get_player_like)
                .post(routes::likes::like_player)
                .delete(routes::likes::unlike_player),
        )
        // Likes — servers
        .route(
            "/api/v1/server/{address}/like",
            get(routes::likes::get_server_like)
                .post(routes::likes::like_server)
                .delete(routes::likes::unlike_server),
        )
        // Favorites
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
        .merge(admin_public)
        .merge(admin_protected)
        .layer(axum_mw::from_fn_with_state(
            state.clone(),
            maintenance_middleware,
        ))
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
