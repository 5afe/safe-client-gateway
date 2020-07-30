#![feature(proc_macro_hygiene, decl_macro, option_result_contains)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

extern crate dotenv;

mod cache;
mod config;
mod routes;
mod services;
mod models;
mod utils;
mod providers;

use dotenv::dotenv;
use cache::{ServiceCache};
use utils::cors::{CORS};
use routes::transaction_routes;
use crate::routes::error_catchers;

fn main() {
    dotenv().ok();
    
    rocket::ignite()
        .mount("/", transaction_routes())
        .manage(reqwest::blocking::Client::new())
        .attach(CORS())
        .attach(ServiceCache::fairing())
        .register(error_catchers())
        .launch();
}
