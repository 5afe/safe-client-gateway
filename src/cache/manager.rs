use std::sync::Arc;

use crate::cache::redis::{new_service_cache, new_service_cache_mainnet};
use crate::cache::Cache;

pub enum ChainCache {
    Mainnet,
    Other,
}

impl From<&str> for ChainCache {
    fn from(id: &str) -> Self {
        match id {
            "1" => ChainCache::Mainnet,
            _ => ChainCache::Other,
        }
    }
}

#[rocket::async_trait]
pub trait RedisCacheManager: Send + Sync {
    fn cache_for_chain(&self, chain_cache: ChainCache) -> Arc<dyn Cache>;
}

pub struct DefaultRedisCacheManager {
    mainnet_cache: Arc<dyn Cache>,
    default_cache: Arc<dyn Cache>,
}

pub async fn create_cache_manager() -> DefaultRedisCacheManager {
    DefaultRedisCacheManager {
        mainnet_cache: Arc::new(new_service_cache_mainnet().await),
        default_cache: Arc::new(new_service_cache().await),
    }
}

#[rocket::async_trait]
impl RedisCacheManager for DefaultRedisCacheManager {
    fn cache_for_chain(&self, chain_cache: ChainCache) -> Arc<dyn Cache> {
        match chain_cache {
            ChainCache::Mainnet => self.mainnet_cache.clone(),
            ChainCache::Other => self.default_cache.clone(),
        }
    }
}
