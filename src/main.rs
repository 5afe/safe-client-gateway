#![feature(proc_macro_hygiene, decl_macro, option_result_contains)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate dotenv_codegen;

mod cache;
mod routes;
mod services;
mod models;
mod utils;

use cache::{ServiceCache};
use utils::cors::{CORS};
use routes::transaction_routes;
use crate::routes::error_catchers;

fn main() {
    rocket::ignite()
        .mount("/", transaction_routes())
        .manage(reqwest::blocking::Client::new())
        .attach(CORS())
        .attach(ServiceCache::fairing())
        .register(error_catchers())
        .launch();
}
