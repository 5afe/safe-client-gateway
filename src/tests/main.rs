extern crate dotenv;

use crate::cache::manager::{CacheManager, RedisCacheManager};
use crate::cache::redis::create_service_cache;
use crate::cache::{Cache, MockCache};
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

    let cache_manager = RedisCacheManager::new();

    rocket::build()
        .mount("/", routes)
        .manage(Arc::new(create_service_cache()) as Arc<dyn Cache>)
        .manage(Arc::new(mock_http_client) as Arc<dyn HttpClient>)
        .manage(Arc::new(cache_manager) as Arc<dyn CacheManager>)
}

#[cfg(test)]
pub fn setup_rocket_with_mock_cache(
    mock_http_client: MockHttpClient,
    mock_cache: MockCache,
    routes: impl Into<Vec<Route>>,
) -> Rocket<Build> {
    dotenv().ok();

    let cache_manager = RedisCacheManager::new();

    rocket::build()
        .mount("/", routes)
        .manage(Arc::new(mock_cache) as Arc<dyn Cache>)
        .manage(Arc::new(mock_http_client) as Arc<dyn HttpClient>)
        .manage(Arc::new(cache_manager) as Arc<dyn CacheManager>)
}
