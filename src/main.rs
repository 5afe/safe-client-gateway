#![deny(unused_must_use)]

extern crate log;
extern crate semver;

#[macro_use]
extern crate rocket;

extern crate dotenv;

use std::time::Duration;

use dotenv::dotenv;

use cache::redis::create_pool;
use routes::active_routes;
use utils::cors::CORS;

use crate::routes::error_catchers;

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

    rocket::build()
        .mount("/", active_routes())
        .register("/", error_catchers())
        .manage(create_pool())
        .manage(client)
        .attach(monitoring::performance::PerformanceMonitor())
        .attach(CORS())
}
