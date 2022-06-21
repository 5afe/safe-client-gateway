#![deny(unused_must_use)]
#![deny(rustdoc::broken_intra_doc_links)]

#[macro_use]
extern crate rocket;

use std::sync::Arc;

use dotenv::dotenv;
use rocket::{Build, Rocket};

use routes::active_routes;
use utils::cors::CORS;

use crate::cache::manager::{create_cache_manager, RedisCacheManager};
use crate::routes::error_catchers;
use crate::utils::http_client::{setup_http_client, HttpClient};
use rocket_okapi::rapidoc::*;
use rocket_okapi::swagger_ui::*;
use rocket_okapi::{openapi, openapi_get_routes};

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
    let cache_manager = create_cache_manager().await;

    rocket::build()
        .mount("/", active_routes())
        .mount(
            "/swagger",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .register("/", error_catchers())
        .manage(Arc::new(cache_manager) as Arc<dyn RedisCacheManager>)
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
