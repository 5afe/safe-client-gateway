#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

pub mod routes;
pub mod services;

use routes::transaction_routes;

fn main() {
    rocket::ignite().mount("/", transaction_routes()).launch();
}
