extern crate rocket;

use rocket::Route;
use rocket::Catcher;
use rocket_contrib::json::JsonValue;

pub mod about;
pub mod balances;
pub mod collectibles;
pub mod transactions;
pub mod hooks;
pub mod health;

pub fn active_routes() -> Vec<Route> {
    routes![about::backbone, about::info, collectibles::list, transactions::details, transactions::all, hooks::update, health::health]
}

pub fn error_catchers() -> Vec<Catcher> {
    catchers![not_found, panic]
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