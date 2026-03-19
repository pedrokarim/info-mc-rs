use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::extract::{ConnectInfo, Extension};
use axum::http::{Request, StatusCode};
use axum::middleware::Next;
use axum::response::{IntoResponse, Response};
use tokio::sync::Mutex;

/// Simple in-memory rate limiter using a sliding window.
#[derive(Clone)]
pub struct RateLimiter {
    inner: Arc<Mutex<HashMap<IpAddr, Vec<Instant>>>>,
    max_requests: usize,
    window: Duration,
}

impl RateLimiter {
    pub fn new(max_requests: usize, window: Duration) -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            max_requests,
            window,
        }
    }

    pub fn from_env() -> Self {
        let max_requests: usize = std::env::var("RATE_LIMIT_MAX")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        let window_secs: u64 = std::env::var("RATE_LIMIT_WINDOW_SECS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);

        Self::new(max_requests, Duration::from_secs(window_secs))
    }

    async fn check(&self, ip: IpAddr) -> bool {
        let mut map = self.inner.lock().await;
        let now = Instant::now();
        let cutoff = now - self.window;

        let entries = map.entry(ip).or_default();
        entries.retain(|t| *t > cutoff);

        if entries.len() >= self.max_requests {
            return false;
        }

        entries.push(now);
        true
    }
}

/// Axum middleware layer for rate limiting.
pub async fn rate_limit_middleware(
    ConnectInfo(addr): ConnectInfo<std::net::SocketAddr>,
    Extension(limiter): Extension<RateLimiter>,
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    // Skip rate limiting for loopback IPs (trusted SSR calls from SvelteKit)
    if addr.ip().is_loopback() {
        return next.run(request).await;
    }

    if !limiter.check(addr.ip()).await {
        let body = serde_json::json!({
            "error": "rate_limit_exceeded",
            "message": "Too many requests. Please try again later."
        });
        return (StatusCode::TOO_MANY_REQUESTS, axum::Json(body)).into_response();
    }

    next.run(request).await
}
