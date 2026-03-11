use std::sync::Arc;
use std::time::Duration;

use mc_cache::TtlCache;
use mc_mojang::MojangClient;

use crate::routes::player::PlayerResponse;
use crate::routes::server::ServerResponse;

/// Shared application state, accessible in all route handlers.
#[derive(Clone)]
pub struct AppState {
    pub server_cache: TtlCache<String, ServerResponse>,
    pub player_cache: TtlCache<String, PlayerResponse>,
    pub mojang: MojangClient,
    pub http: reqwest::Client,
}

impl AppState {
    pub fn new() -> Self {
        let http = reqwest::Client::builder()
            .user_agent("MCInfo-RS/0.1")
            .timeout(Duration::from_secs(3))
            .build()
            .expect("failed to build HTTP client");

        Self {
            server_cache: TtlCache::new(Duration::from_secs(60), 10_000),
            player_cache: TtlCache::new(Duration::from_secs(300), 10_000),
            mojang: MojangClient::new(),
            http,
        }
    }
}

pub type SharedState = Arc<AppState>;
