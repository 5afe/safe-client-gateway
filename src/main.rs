#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

pub mod routes;
pub mod services;
pub mod models;
use rocket_contrib::json::JsonValue;

use routes::transaction_routes;

fn main() {
    rocket::ignite()
        .mount("/", transaction_routes())
        .register(catchers![not_found, panic])
        .launch();
}

#[catch(404)]
fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
#[catch(500)]
fn panic() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Server error occurred."
    })
}