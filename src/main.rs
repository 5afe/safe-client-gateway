#![feature(proc_macro_hygiene, decl_macro, option_result_contains)]

extern crate log;

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

extern crate dotenv;

#[macro_use]
pub mod macros;

mod config;
mod models;
mod monitoring;
mod providers;
mod routes;
mod services;
mod utils;

#[cfg(test)]
mod json;

use crate::routes::error_catchers;
use dotenv::dotenv;
use routes::active_routes;
use utils::cache::ServiceCache;
use utils::cors::CORS;

fn main() {
    dotenv().ok();
    env_logger::init();

    rocket::ignite()
        .mount("/", active_routes())
        .manage(reqwest::blocking::Client::new())
        .attach(monitoring::performance::PerformanceMonitor())
        .attach(CORS())
        .attach(ServiceCache::fairing())
        .register(error_catchers())
        .launch();
}
