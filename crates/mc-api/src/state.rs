use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

use mc_cache::TtlCache;
use mc_mojang::MojangClient;
use sqlx::SqlitePool;
use tokio::sync::Mutex;

use crate::ascencia::AscenciaConfig;
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
    pub ascencia: AscenciaConfig,
    pub ascencia_refresh_lock: Arc<Mutex<()>>,
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

        let ascencia = AscenciaConfig::from_env();
        if !ascencia.is_configured() {
            tracing::warn!("Ascencia ID is not configured; admin sign-in will be unavailable");
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
            "CREATE TABLE IF NOT EXISTS ascencia_admin_sessions (
                token_hash TEXT PRIMARY KEY,
                account_id TEXT NOT NULL,
                access_token TEXT NOT NULL,
                refresh_token TEXT,
                access_expires_at TEXT NOT NULL,
                session_expires_at TEXT NOT NULL,
                claims TEXT NOT NULL,
                profile TEXT NOT NULL,
                created_at TEXT NOT NULL DEFAULT (datetime('now')),
                updated_at TEXT NOT NULL DEFAULT (datetime('now'))
            )",
        )
        .execute(&db)
        .await
        .expect("failed to create Ascencia admin sessions table");

        sqlx::query(
            "CREATE INDEX IF NOT EXISTS ascencia_admin_sessions_account_idx
             ON ascencia_admin_sessions (account_id)",
        )
        .execute(&db)
        .await
        .expect("failed to index Ascencia admin sessions");

        sqlx::query(
            "CREATE TABLE IF NOT EXISTS admin_audit_log (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                account_id TEXT NOT NULL,
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

        // Migration de l'ancien journal Discord, conservé sans perdre l'historique.
        let audit_columns: Vec<String> =
            sqlx::query_scalar("SELECT name FROM pragma_table_info('admin_audit_log')")
                .fetch_all(&db)
                .await
                .unwrap_or_default();
        if audit_columns.iter().any(|column| column == "discord_id")
            && !audit_columns.iter().any(|column| column == "account_id")
        {
            sqlx::query("ALTER TABLE admin_audit_log RENAME COLUMN discord_id TO account_id")
                .execute(&db)
                .await
                .expect("failed to migrate the admin audit identity column");
        }
        sqlx::query("ALTER TABLE servers ADD COLUMN motd_html TEXT")
            .execute(&db)
            .await
            .ok();

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
            ascencia,
            ascencia_refresh_lock: Arc::new(Mutex::new(())),
            maintenance_mode: Arc::new(AtomicBool::new(maintenance_mode)),
        }
    }
}

pub type SharedState = Arc<AppState>;
