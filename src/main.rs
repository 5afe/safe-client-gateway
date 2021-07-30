#![feature(async_closure, proc_macro_hygiene, decl_macro, option_result_contains)]
#![deny(unused_must_use)]

extern crate log;
extern crate semver;

#[macro_use]
extern crate rocket;

extern crate dotenv;

#[doc(hidden)]
#[macro_use]
pub mod macros;

#[doc(hidden)]
mod cache;
#[doc(hidden)]
mod config;

/// Models exposed by this service
///
/// *Important:* Names, Enums and Polymorphism
///
/// Every field in the structs that you will see in this documentation is **camelCased** on serialisation.
///
/// Enums are **SCREAMING_SNAKE_CASED** on serialization and the variant is always put into a `type` json field for polymorphic cases.
mod models;
#[doc(hidden)]
mod monitoring;
#[doc(hidden)]
mod providers;

/// Collection of all endpoints all endpoints
mod routes;
#[doc(hidden)]
mod services;
#[doc(hidden)]
mod utils;

#[cfg(test)]
mod json;

use crate::monitoring::rate_limiter::RateLimiterConfig;
use crate::routes::error_catchers;
use cache::redis::create_pool;
use dotenv::dotenv;
use routes::active_routes;
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

    rocket::build()
        .mount("/", active_routes())
        .register("/", error_catchers())
        .manage(create_pool())
        .manage(client)
        .manage(RateLimiterConfig::new())
        .attach(monitoring::performance::PerformanceMonitor())
        .attach(CORS())
}
