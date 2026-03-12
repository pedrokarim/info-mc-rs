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

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn make_cache() -> TtlCache<String, String> {
        TtlCache::new(Duration::from_secs(60), 100)
    }

    #[tokio::test]
    async fn test_insert_and_get() {
        let cache = make_cache();
        cache.insert("key".into(), "value".into()).await;
        let result = cache.get(&"key".into()).await;
        assert_eq!(*result.unwrap(), "value");
    }

    #[tokio::test]
    async fn test_get_missing_key() {
        let cache = make_cache();
        assert!(cache.get(&"missing".into()).await.is_none());
    }

    #[tokio::test]
    async fn test_invalidate() {
        let cache = make_cache();
        cache.insert("key".into(), "value".into()).await;
        cache.invalidate(&"key".into()).await;
        assert!(cache.get(&"key".into()).await.is_none());
    }

    #[tokio::test]
    async fn test_entry_count() {
        let cache = make_cache();
        cache.insert("a".into(), "1".into()).await;
        cache.insert("b".into(), "2".into()).await;
        cache.insert("c".into(), "3".into()).await;
        // moka may need a sync to reflect entry_count accurately
        cache.inner.run_pending_tasks().await;
        assert_eq!(cache.entry_count(), 3);
    }

    #[tokio::test]
    async fn test_get_or_insert_with() {
        let cache = make_cache();
        let val = cache
            .get_or_insert_with("key".into(), || async { "computed".into() })
            .await;
        assert_eq!(*val, "computed");
        // Should be cached now
        let cached = cache.get(&"key".into()).await;
        assert_eq!(*cached.unwrap(), "computed");
    }

    #[tokio::test]
    async fn test_get_or_try_insert_with_ok() {
        let cache = make_cache();
        let result: Result<_, String> = cache
            .get_or_try_insert_with("key".into(), || async { Ok("ok_value".into()) })
            .await;
        assert_eq!(*result.unwrap(), "ok_value");
        // Should be cached
        let cached = cache.get(&"key".into()).await;
        assert_eq!(*cached.unwrap(), "ok_value");
    }

    #[tokio::test]
    async fn test_get_or_try_insert_with_err() {
        let cache = make_cache();
        let result: Result<Arc<String>, String> = cache
            .get_or_try_insert_with("key".into(), || async {
                Err("something failed".to_string())
            })
            .await;
        assert_eq!(result.unwrap_err(), "something failed");
        // Should NOT be cached
        assert!(cache.get(&"key".into()).await.is_none());
    }
}
