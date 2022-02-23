extern crate dotenv;

use crate::cache::{Cache, MockCache};
use crate::create_service_cache;
use crate::utils::http_client::{HttpClient, MockHttpClient};
use dotenv::dotenv;
use rocket::{Build, Rocket, Route};
use std::sync::Arc;

#[cfg(test)]
pub async fn setup_rocket(
    mock_http_client: MockHttpClient,
    routes: impl Into<Vec<Route>>,
) -> Rocket<Build> {
    dotenv().ok();
    let cache = create_service_cache().await;
    cache.invalidate_pattern("*").await; // Clearing cache for test

    rocket::build()
        .mount("/", routes)
        .manage(Arc::new(mock_http_client) as Arc<dyn HttpClient>)
        .manage(Arc::new(cache) as Arc<dyn Cache>)
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
