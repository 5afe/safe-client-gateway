#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_contrib;

pub mod routes;
pub mod services;
pub mod models;

use routes::transaction_routes;
use crate::routes::error_catchers;

fn main() {
    rocket::ignite()
        .mount("/", transaction_routes())
        .register(error_catchers())
        .launch();
}
