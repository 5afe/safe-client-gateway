use crate::cache::redis::{RedisPool, ServiceCache};
use crate::cache::{Cache, MockCache};
use crate::config::{default_redis_uri, info_redis_pool_size, info_redis_uri, redis_uri};
use mockall::automock;
use r2d2::Pool;
use std::sync::Arc;

fn create_info_pool() -> RedisPool {
    let client = redis::Client::open(info_redis_uri()).unwrap();
    Pool::builder()
        .max_size(info_redis_pool_size())
        .build(client)
        .unwrap()
}

fn create_default_pool() -> RedisPool {
    let client = redis::Client::open(default_redis_uri()).unwrap();
    Pool::builder()
        .max_size(info_redis_pool_size())
        .build(client)
        .unwrap()
}

#[automock]
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
            info_cache: Arc::new(ServiceCache::new(create_info_pool())),
            default_cache: Arc::new(ServiceCache::new(create_default_pool())),
        }
    }

    #[cfg(test)]
    pub fn new_with_mocks(info_cache: MockCache, default_cache: MockCache) -> Self {
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
