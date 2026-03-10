use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use std::time::{Duration, Instant};

use axum::extract::ConnectInfo;
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
    request: Request<axum::body::Body>,
    next: Next,
) -> Response {
    // Access the rate limiter from extensions
    let limiter = request
        .extensions()
        .get::<RateLimiter>()
        .cloned();

    if let Some(limiter) = limiter {
        if !limiter.check(addr.ip()).await {
            let body = serde_json::json!({
                "error": "rate_limit_exceeded",
                "message": "Too many requests. Please try again later."
            });
            return (
                StatusCode::TOO_MANY_REQUESTS,
                axum::Json(body),
            )
                .into_response();
        }
    }

    next.run(request).await
}
