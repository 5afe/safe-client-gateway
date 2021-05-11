extern crate rocket;

use rocket::response::Redirect;
use rocket::Catcher;
use rocket::Route;
use rocket_contrib::json::JsonValue;

pub mod about;
pub mod balances;
pub mod collectibles;
pub mod health;
pub mod hooks;
pub mod safes;
pub mod transactions;

pub fn active_routes() -> Vec<Route> {
    routes![
        root,
        about::backbone,
        about::info,
        about::redis,
        balances::get_balances,
        balances::get_supported_fiat,
        collectibles::list,
        safes::safe_info,
        transactions::details,
        transactions::all,
        transactions::history_transactions,
        transactions::queued_transactions,
        transactions::submit_confirmation,
        transactions::propose_transaction,
        hooks::update,
        hooks::flush_all,
        health::health
    ]
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

#[get("/")]
pub fn root() -> Redirect {
    Redirect::temporary("https://github.com/gnosis/safe-client-gateway/wiki")
}
