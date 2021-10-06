use crate::cache::redis::ServiceCache;

pub trait CacheManager {
    type CacheWrapperType;

    fn info_cache(&self) -> &Self::CacheWrapperType;

    fn default_cache(&self) -> &Self::CacheWrapperType;
}

pub struct RedisCacheManager<'r> {
    info_cache: &'r ServiceCache<'r>,
    default_cache: &'r ServiceCache<'r>,
}

impl<'r> RedisCacheManager<'r> {
    pub fn new(info_cache: &'r ServiceCache, default_cache: &'r ServiceCache) -> Self {
        RedisCacheManager {
            info_cache,
            default_cache,
        }
    }
}

impl<'r> CacheManager for RedisCacheManager<'r> {
    type CacheWrapperType = ServiceCache<'r>;

    fn info_cache(&self) -> &Self::CacheWrapperType {
        self.info_cache
    }

    fn default_cache(&self) -> &Self::CacheWrapperType {
        self.default_cache
    }
}
