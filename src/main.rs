#![feature(proc_macro_hygiene, decl_macro, option_result_contains)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

extern crate dotenv;

mod config;
mod routes;
mod services;
mod models;
mod utils;
mod providers;

use dotenv::dotenv;
use utils::cache::{ServiceCache};
use utils::cors::{CORS};
use routes::active_routes;
use crate::routes::error_catchers;

fn main() {
    dotenv().ok();
    
    rocket::ignite()
        .mount("/", active_routes())
        .manage(reqwest::blocking::Client::new())
        .attach(CORS())
        .attach(ServiceCache::fairing())
        .register(error_catchers())
        .launch();
}
