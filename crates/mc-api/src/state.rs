use std::sync::Arc;
use std::time::Duration;

use mc_cache::TtlCache;
use mc_mojang::MojangClient;
use sqlx::SqlitePool;

use crate::routes::player::PlayerResponse;
use crate::routes::server::ServerResponse;

/// Shared application state, accessible in all route handlers.
#[derive(Clone)]
pub struct AppState {
    pub server_cache: TtlCache<String, ServerResponse>,
    pub player_cache: TtlCache<String, PlayerResponse>,
    pub mojang: MojangClient,
    pub http: reqwest::Client,
    pub db: SqlitePool,
}

impl AppState {
    pub async fn new() -> Self {
        let http = reqwest::Client::builder()
            .user_agent("MCInfo-RS/0.1")
            .timeout(Duration::from_secs(3))
            .build()
            .expect("failed to build HTTP client");

        // Ensure data directory exists
        tokio::fs::create_dir_all("./data").await.ok();

        let db = SqlitePool::connect("sqlite:./data/mcinfo.db?mode=rwc")
            .await
            .expect("failed to connect to SQLite");

        // Create tables
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS favorites (
                uuid TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                favorited_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
        )
        .execute(&db)
        .await
        .expect("failed to create favorites table");

        Self {
            server_cache: TtlCache::new(Duration::from_secs(60), 10_000),
            player_cache: TtlCache::new(Duration::from_secs(300), 10_000),
            mojang: MojangClient::new(),
            http,
            db,
        }
    }
}

pub type SharedState = Arc<AppState>;
