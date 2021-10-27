extern crate dotenv;

use crate::cache::manager::{CacheManager, RedisCacheManager};
use crate::cache::MockCache;
use crate::utils::http_client::{HttpClient, MockHttpClient};
use dotenv::dotenv;
use rocket::{Build, Rocket, Route};
use std::sync::Arc;

#[cfg(test)]
pub fn setup_rocket(
    mock_http_client: MockHttpClient,
    routes: impl Into<Vec<Route>>,
) -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .mount("/", routes)
        .manage(Arc::new(mock_http_client) as Arc<dyn HttpClient>)
        .manage(Arc::new(RedisCacheManager::new()) as Arc<dyn CacheManager>)
}

#[cfg(test)]
pub fn setup_rocket_with_mock_cache(
    mock_http_client: MockHttpClient,
    mock_default_cache: MockCache,
    mock_info_cache: MockCache,
    routes: impl Into<Vec<Route>>,
) -> Rocket<Build> {
    dotenv().ok();

    let cache_manager = RedisCacheManager::new_with_mocks(mock_info_cache, mock_default_cache);
    rocket::build()
        .mount("/", routes)
        .manage(Arc::new(mock_http_client) as Arc<dyn HttpClient>)
        .manage(Arc::new(cache_manager) as Arc<dyn CacheManager>)
}
