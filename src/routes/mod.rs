extern crate rocket;

use rocket::response::Redirect;
use rocket::serde::json::{json, Value};
use rocket::Catcher;
use rocket::Route;

/// # About endpoint
pub mod about;
/// # Balance endpoints
pub mod balances;
/// # Chain endpoints
pub mod chains;
/// # Collectibles endpoint
pub mod collectibles;
#[doc(hidden)]
pub mod health;
#[doc(hidden)]
pub mod hooks;
/// # Notification endpoints
pub mod notifications;
/// # SafeApps endpoints
pub mod safe_apps;
/// # Safe endpoints
pub mod safes;
/// # Transactions endpoints
///
/// As presented by the endpoints in this service, we are taking in the types returned by the [transaction service](https://github.com/gnosis/safe-transaction-service-example), which to this data are `Multisig`, `Module` and `Ethereum` transaction types.
///
/// The types served by the gate way are `Transfer`, `SettingsChange` and `Custom`. Additionally, we treat the `Creation` transaction as one additional type, as it is meant to be group with the rest of the items in the same UI component in the apps.
pub mod transactions;
/// # Utility endpoints
pub mod utils;

#[doc(hidden)]
pub fn active_routes() -> Vec<Route> {
    routes![
        root,
        about::backbone,
        about::get_about,
        about::redis,
        about::get_master_copies,
        balances::get_balances,
        balances::get_supported_fiat,
        chains::get_chain,
        chains::get_chains,
        collectibles::get_collectibles,
        notifications::post_notification_registration,
        notifications::delete_notification_registration,
        safes::get_safe_info,
        safes::get_owners,
        safe_apps::get_safe_apps,
        transactions::get_transactions,
        transactions::get_transactions_history,
        transactions::get_transactions_queued,
        transactions::post_transaction,
        transactions::post_confirmation,
        hooks::update,
        hooks::flush,
        health::health,
        utils::post_data_decoder,
        utils::post_safe_gas_estimation
    ]
}

#[doc(hidden)]
pub fn error_catchers() -> Vec<Catcher> {
    catchers![not_found, panic]
}

#[doc(hidden)]
#[catch(404)]
fn not_found() -> Value {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}

#[doc(hidden)]
#[catch(500)]
fn panic() -> Value {
    json!({
        "status": "error",
        "reason": "Server error occurred."
    })
}

#[doc(hidden)]
#[get("/")]
pub fn root() -> Redirect {
    Redirect::temporary("https://gnosis.github.io/safe-client-gateway/")
}
