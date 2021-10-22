#![deny(unused_must_use)]

extern crate dotenv;
extern crate log;
extern crate semver;

#[macro_use]
extern crate rocket;

#[doc(hidden)]
#[macro_use]
pub mod macros;

#[doc(hidden)]
mod cache;
#[doc(hidden)]
mod common;
#[doc(hidden)]
mod config;

#[doc(hidden)]
mod monitoring;
#[doc(hidden)]
mod providers;

/// Collection of all endpoints all endpoints
mod routes;
#[doc(hidden)]
mod utils;

#[cfg(test)]
mod tests;

use crate::cache::redis::create_service_cache;
use crate::cache::Cache;
use crate::routes::error_catchers;
use crate::utils::http_client::HttpClient;
use dotenv::dotenv;
use routes::active_routes;
use std::sync::Arc;
use std::time::Duration;
use utils::cors::CORS;

#[doc(hidden)]
#[launch]
fn rocket() -> _ {
    dotenv().ok();
    env_logger::init();

    let client = reqwest::Client::builder()
        .connect_timeout(Duration::from_millis(
            config::internal_client_connect_timeout(),
        ))
        .build()
        .unwrap();

    let cache = create_service_cache();

    rocket::build()
        .mount("/", active_routes())
        .register("/", error_catchers())
        .manage(Arc::new(cache) as Arc<dyn Cache>)
        .manage(Arc::new(client) as Arc<dyn HttpClient>)
        .attach(monitoring::performance::PerformanceMonitor())
        .attach(CORS())
}
