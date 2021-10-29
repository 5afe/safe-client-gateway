use crate::cache::redis::ServiceCache;
use crate::cache::Cache;
use std::sync::Arc;

pub trait CacheManager: Sync + Send {
    fn info_cache(&self) -> Arc<dyn Cache>;
    fn default_cache(&self) -> Arc<dyn Cache>;
}

pub struct RedisCacheManager {
    info_cache: Arc<dyn Cache>,
    default_cache: Arc<dyn Cache>,
}

impl RedisCacheManager {
    pub fn new() -> Self {
        RedisCacheManager {
            info_cache: Arc::new(ServiceCache::new_info_cache()),
            default_cache: Arc::new(ServiceCache::new_default_cache()),
        }
    }

    #[cfg(test)]
    pub fn new_with_mocks(
        info_cache: crate::cache::MockCache,
        default_cache: crate::cache::MockCache,
    ) -> Self {
        RedisCacheManager {
            info_cache: Arc::new(info_cache) as Arc<dyn Cache>,
            default_cache: Arc::new(default_cache) as Arc<dyn Cache>,
        }
    }
}

impl CacheManager for RedisCacheManager {
    fn info_cache(&self) -> Arc<dyn Cache> {
        self.info_cache.clone()
    }

    fn default_cache(&self) -> Arc<dyn Cache> {
        self.default_cache.clone()
    }
}
