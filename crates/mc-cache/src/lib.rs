use std::hash::Hash;
use std::sync::Arc;
use std::time::Duration;

use moka::future::Cache;

/// Generic async cache with configurable TTL, backed by moka.
#[derive(Clone)]
pub struct TtlCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    inner: Cache<K, Arc<V>>,
}

impl<K, V> TtlCache<K, V>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    /// Create a new cache with the given TTL and max capacity.
    pub fn new(ttl: Duration, max_capacity: u64) -> Self {
        let inner = Cache::builder()
            .time_to_live(ttl)
            .max_capacity(max_capacity)
            .build();
        Self { inner }
    }

    /// Get a cached value, or compute it if missing/expired.
    pub async fn get_or_insert_with<F, Fut>(&self, key: K, f: F) -> Arc<V>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = V>,
        K: Clone,
    {
        if let Some(cached) = self.inner.get(&key).await {
            return cached;
        }

        let value = Arc::new(f().await);
        self.inner.insert(key, value.clone()).await;
        value
    }

    /// Try to get from cache, or compute with a fallible function.
    pub async fn get_or_try_insert_with<F, Fut, E>(&self, key: K, f: F) -> Result<Arc<V>, E>
    where
        F: FnOnce() -> Fut,
        Fut: std::future::Future<Output = Result<V, E>>,
        K: Clone,
    {
        if let Some(cached) = self.inner.get(&key).await {
            return Ok(cached);
        }

        let value = Arc::new(f().await?);
        self.inner.insert(key, value.clone()).await;
        Ok(value)
    }

    /// Get a value from cache without computing.
    pub async fn get(&self, key: &K) -> Option<Arc<V>> {
        self.inner.get(key).await
    }

    /// Manually insert a value.
    pub async fn insert(&self, key: K, value: V) {
        self.inner.insert(key, Arc::new(value)).await;
    }

    /// Invalidate a cached entry.
    pub async fn invalidate(&self, key: &K) {
        self.inner.invalidate(key).await;
    }

    /// Number of entries currently in the cache.
    pub fn entry_count(&self) -> u64 {
        self.inner.entry_count()
    }
}
