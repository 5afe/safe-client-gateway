use std::sync::Arc;

use dotenv::dotenv;
use rocket::{Build, Rocket, Route};

use crate::cache::manager::ChainCache;
use crate::cache::{Cache, MockCache};
use crate::utils::http_client::{HttpClient, MockHttpClient};
use crate::{create_cache_manager, RedisCacheManager};

#[cfg(test)]
pub async fn setup_rocket(
    mock_http_client: MockHttpClient,
    routes: impl Into<Vec<Route>>,
) -> Rocket<Build> {
    dotenv().ok();
    let cache_manager = create_cache_manager().await;
    cache_manager
        .cache_for_chain(ChainCache::Mainnet)
        .invalidate_pattern("*")
        .await;
    cache_manager
        .cache_for_chain(ChainCache::Other)
        .invalidate_pattern("*")
        .await;

    rocket::build()
        .mount("/", routes)
        .manage(Arc::new(mock_http_client) as Arc<dyn HttpClient>)
        .manage(Arc::new(cache_manager) as Arc<dyn RedisCacheManager>)
}

#[cfg(test)]
pub fn setup_rocket_with_mock_cache(
    mock_http_client: MockHttpClient,
    mock_cache: MockCache,
    routes: impl Into<Vec<Route>>,
) -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .mount("/", routes)
        .manage(Arc::new(mock_http_client) as Arc<dyn HttpClient>)
        .manage(Arc::new(mock_cache) as Arc<dyn Cache>)
}

#[rocket::async_test]
pub async fn main_produces_valid_rocket_instance() {
    crate::rocket().await;
}
