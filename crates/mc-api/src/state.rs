use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use mc_cache::TtlCache;
use mc_mojang::MojangClient;
use rand::Rng;
use sqlx::SqlitePool;

use crate::routes::player::PlayerResponse;
use crate::routes::server::ServerResponse;

/// Shared application state, accessible in all route handlers.
#[derive(Clone)]
pub struct AppState {
    pub server_cache: TtlCache<String, ServerResponse>,
    pub player_cache: TtlCache<String, PlayerResponse>,
    #[allow(dead_code)]
    pub texture_cache: TtlCache<String, Vec<u8>>,
    pub render3d_cache: TtlCache<String, Vec<u8>>,
    pub mojang: MojangClient,
    pub http: reqwest::Client,
    pub admin_http: reqwest::Client,
    pub db: SqlitePool,
    pub ip_salt: String,
    pub jwt_secret: String,
    pub discord_client_id: String,
    pub discord_client_secret: String,
    pub discord_redirect_uri: String,
    pub maintenance_mode: Arc<AtomicBool>,
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

        let admin_http = reqwest::Client::builder()
            .user_agent("MCInfo-RS/0.1")
            .timeout(Duration::from_secs(10))
            .build()
            .expect("failed to build admin HTTP client");

        let ip_salt =
            std::env::var("IP_HASH_SALT").unwrap_or_else(|_| "mcinfo-default-salt".to_string());

        let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
            let secret: String = rand::thread_rng()
                .sample_iter(&rand::distributions::Alphanumeric)
                .take(32)
                .map(char::from)
                .collect();
            tracing::warn!(
                "JWT_SECRET not set, using random secret (sessions won't survive restarts)"
            );
            secret
        });

        let discord_client_id = std::env::var("DISCORD_CLIENT_ID").unwrap_or_default();
        let discord_client_secret = std::env::var("DISCORD_CLIENT_SECRET").unwrap_or_default();
        let discord_redirect_uri = std::env::var("DISCORD_REDIRECT_URI")
            .unwrap_or_else(|_| "http://127.0.0.1:3001/api/v1/admin/auth/callback".to_string());

        if discord_client_id.is_empty() {
            tracing::warn!("DISCORD_CLIENT_ID not set, admin OAuth will be unavailable");
        }

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

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS players (
                uuid TEXT PRIMARY KEY,
                username TEXT NOT NULL,
                skin_url TEXT,
                skin_model TEXT,
                status TEXT NOT NULL DEFAULT 'active',
                first_seen_at TEXT NOT NULL DEFAULT (datetime('now')),
                last_seen_at TEXT NOT NULL DEFAULT (datetime('now')),
                views INTEGER NOT NULL DEFAULT 1,
                likes INTEGER NOT NULL DEFAULT 0
            )",
        )
        .execute(&db)
        .await
        .expect("failed to create players table");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS servers (
                address TEXT PRIMARY KEY,
                hostname TEXT NOT NULL,
                ip TEXT NOT NULL,
                port INTEGER NOT NULL,
                edition TEXT NOT NULL,
                version_name TEXT,
                motd_clean TEXT,
                favicon TEXT,
                max_players INTEGER,
                status TEXT NOT NULL DEFAULT 'active',
                first_seen_at TEXT NOT NULL DEFAULT (datetime('now')),
                last_seen_at TEXT NOT NULL DEFAULT (datetime('now')),
                last_online_at TEXT,
                views INTEGER NOT NULL DEFAULT 1,
                likes INTEGER NOT NULL DEFAULT 0
            )",
        )
        .execute(&db)
        .await
        .expect("failed to create servers table");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS likes (
                entity_type TEXT NOT NULL,
                entity_id TEXT NOT NULL,
                ip_hash TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                PRIMARY KEY (entity_type, entity_id, ip_hash)
            )",
        )
        .execute(&db)
        .await
        .expect("failed to create likes table");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS admin_users (
                discord_id TEXT PRIMARY KEY,
                discord_username TEXT NOT NULL,
                discord_avatar TEXT,
                role TEXT NOT NULL DEFAULT 'admin',
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                last_login_at TEXT
            )",
        )
        .execute(&db)
        .await
        .expect("failed to create admin_users table");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS admin_sessions (
                id TEXT PRIMARY KEY,
                discord_id TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                expires_at TEXT NOT NULL
            )",
        )
        .execute(&db)
        .await
        .expect("failed to create admin_sessions table");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS admin_audit_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                discord_id TEXT NOT NULL,
                action TEXT NOT NULL,
                detail TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
        )
        .execute(&db)
        .await
        .expect("failed to create admin_audit_log table");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS admin_config (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
        )
        .execute(&db)
        .await
        .expect("failed to create admin_config table");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS admin_alerts (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                alert_type TEXT NOT NULL,
                severity TEXT NOT NULL DEFAULT 'info',
                message TEXT NOT NULL,
                entity_type TEXT,
                entity_id TEXT,
                resolved INTEGER NOT NULL DEFAULT 0,
                resolved_by TEXT,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                resolved_at TEXT
            )",
        )
        .execute(&db)
        .await
        .expect("failed to create admin_alerts table");

        // Migrations (ALTER TABLE, safe to re-run)
        sqlx::query("ALTER TABLE admin_users ADD COLUMN totp_secret TEXT")
            .execute(&db)
            .await
            .ok();
        sqlx::query("ALTER TABLE servers ADD COLUMN motd_html TEXT")
            .execute(&db)
            .await
            .ok();

        // Seed super admin
        sqlx::query(
            "INSERT OR IGNORE INTO admin_users (discord_id, discord_username, role)
             VALUES ('319842407829078016', 'owner', 'super_admin')",
        )
        .execute(&db)
        .await
        .expect("failed to seed super admin");

        // Seed default config values
        for (key, value) in [
            ("maintenance_mode", "false"),
            (
                "maintenance_message",
                "Service temporarily unavailable for maintenance",
            ),
            ("like_alert_threshold", "50"),
            ("admin_ip_whitelist", ""),
        ] {
            sqlx::query("INSERT OR IGNORE INTO admin_config (key, value) VALUES (?, ?)")
                .bind(key)
                .bind(value)
                .execute(&db)
                .await
                .ok();
        }

        // Load maintenance mode flag from DB
        let maintenance_mode = sqlx::query_scalar::<_, String>(
            "SELECT value FROM admin_config WHERE key = 'maintenance_mode'",
        )
        .fetch_optional(&db)
        .await
        .ok()
        .flatten()
        .map(|v| v == "true")
        .unwrap_or(false);

        Self {
            server_cache: TtlCache::new(Duration::from_secs(60), 10_000),
            player_cache: TtlCache::new(Duration::from_secs(300), 10_000),
            texture_cache: TtlCache::new(Duration::from_secs(600), 500),
            render3d_cache: TtlCache::new(Duration::from_secs(300), 1_000),
            mojang: MojangClient::new(),
            http,
            admin_http,
            db,
            ip_salt,
            jwt_secret,
            discord_client_id,
            discord_client_secret,
            discord_redirect_uri,
            maintenance_mode: Arc::new(AtomicBool::new(maintenance_mode)),
        }
    }
}

pub type SharedState = Arc<AppState>;
