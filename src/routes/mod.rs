use rocket::response::Redirect;
use rocket::serde::json::{json, Value};
use rocket::{Catcher, Route};

/// # About endpoint
pub mod about;
/// # Balance endpoints
pub mod balances;
/// # Chain endpoints
pub mod chains;
/// # Collectibles endpoint
pub mod collectibles;
/// # Utility endpoints
pub mod contracts;
pub mod delegates;
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
/// As presented by the endpoints in this handlers, we are taking in the types returned by the [transaction handlers](https://github.com/gnosis/safe-transaction-service-example), which to this data are `Multisig`, `Module` and `Ethereum` transaction types.
///
/// The types served by the gate way are `Transfer`, `SettingsChange` and `Custom`. Additionally, we treat the `Creation` transaction as one additional type, as it is meant to be group with the rest of the items in the same UI component in the apps.
pub mod transactions;

#[doc(hidden)]
pub fn active_routes() -> Vec<Route> {
    routes![
        root,
        about::routes::backbone,
        about::routes::get_about,
        about::routes::get_chains_about,
        about::routes::redis,
        about::routes::get_master_copies,
        balances::routes::get_balances,
        balances::routes::get_supported_fiat,
        chains::routes::get_chain,
        chains::routes::get_chains,
        collectibles::routes::get_collectibles,
        contracts::routes::post_data_decoder,
        contracts::routes::get_contract,
        delegates::routes::delete_delegate,
        delegates::routes::delete_safe_delegate,
        delegates::routes::get_delegates,
        delegates::routes::post_delegate,
        notifications::routes::post_notification_registration,
        notifications::routes::delete_notification_registration,
        safes::routes::get_safe_info,
        safes::routes::get_owners,
        safes::routes::post_safe_gas_estimation,
        safes::routes::post_safe_gas_estimation_v2,
        safe_apps::routes::get_safe_apps,
        transactions::routes::get_transactions,
        transactions::routes::get_transactions_history,
        transactions::routes::get_transactions_queued,
        transactions::routes::get_incoming_transfers,
        transactions::routes::get_module_transactions,
        transactions::routes::get_multisig_transactions,
        transactions::routes::post_transaction,
        transactions::routes::post_confirmation,
        hooks::routes::update,
        hooks::routes::post_hook_update,
        hooks::routes::post_hooks_events,
        hooks::routes::post_flush_events,
        hooks::routes::flush,
        health::routes::health
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
    Redirect::temporary("https://safe.global/safe-client-gateway/")
}
