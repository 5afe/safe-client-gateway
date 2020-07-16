#![feature(proc_macro_hygiene, decl_macro, option_result_contains)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

#[macro_use]
extern crate dotenv_codegen;

pub mod routes;
pub mod services;
pub mod models;
pub mod utils;

use routes::transaction_routes;
use crate::routes::error_catchers;

fn main() {
    rocket::ignite()
        .mount("/", transaction_routes())
        .register(error_catchers())
        .launch();
}
