mod error;
mod middleware;
mod routes;
mod state;

use std::net::SocketAddr;
use std::sync::Arc;

use axum::routing::get;
use axum::Router;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::EnvFilter;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| "info".into()))
        .init();

    let state = Arc::new(AppState::new());

    // CORS - allow all origins for now (restrict in production)
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/health", get(routes::health::health))
        .route("/api/v1/server/{address}", get(routes::server::get_server))
        .route("/api/v1/player/{identifier}", get(routes::player::get_player))
        .route("/api/v1/render/{identifier}", get(routes::render::render_skin))
        .layer(cors)
        .with_state(state);

    let port: u16 = std::env::var("PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(3001);
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("MCInfo API listening on {addr}");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>())
        .await
        .unwrap();
}
