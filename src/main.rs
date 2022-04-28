#![deny(unused_must_use)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate rocket;

use std::sync::Arc;

use dotenv::dotenv;
use rocket::{Build, Rocket};

use routes::active_routes;
use utils::cors::CORS;

use crate::cache::redis::{create_service_cache, create_service_cache_mainnet, ServiceCache};
use crate::cache::Cache;
use crate::routes::error_catchers;
use crate::utils::http_client::{setup_http_client, HttpClient};

#[doc(hidden)]
#[macro_use]
pub mod macros;

#[doc(hidden)]
mod cache;
mod common;
#[doc(hidden)]
mod config;

#[doc(hidden)]
mod monitoring;
mod providers;

/// Collection of all endpoints all endpoints
mod routes;
#[doc(hidden)]
mod utils;

#[cfg(test)]
mod tests;

#[doc(hidden)]
#[launch]
async fn rocket() -> Rocket<Build> {
    dotenv().ok();
    setup_logger();

    let client = setup_http_client();
    let cache = create_service_cache().await;
    let mainnet_cache = create_service_cache_mainnet().await;

    rocket::build()
        .mount("/", active_routes())
        .register("/", error_catchers())
        .manage(Arc::new(cache) as Arc<dyn Cache>)
        .manage(Arc::new(mainnet_cache) as Arc<ServiceCache>) // This is incredibly hacky and should be removed as soon as a different redis instance is not required
        .manage(Arc::new(client) as Arc<dyn HttpClient>)
        .attach(monitoring::performance::PerformanceMonitor())
        .attach(CORS())
}

#[cfg(test)]
fn setup_logger() {
    // noop: no need to set the logger for tests
}

#[doc(hidden)]
#[cfg(not(test))]
fn setup_logger() {
    env_logger::init();
}
