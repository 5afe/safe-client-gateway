use crate::config::request_cache_duration
use crate::utils::context::Context;
use crate::services::transactions;
use rocket::response::content;
use anyhow::Result;

#[get("/transactions/<safe_address>")]
pub fn all(context: Context, safe_address: String) -> Result<content::Json<String>> {
    context.cache().cache_resp(&safe_address, request_cache_duration(), || {
        transactions::get_all_transactions(&context, &safe_address)
    })
}

#[get("/transaction/<tx_hash>")]
pub fn details(tx_hash: String) -> content::Json<String> {
    content::Json(transactions::get_transactions_details(tx_hash))
}

#[get("/transactions/about")]
pub fn about(context: Context) -> Result<content::Json<String>> {
    context.cache().cache_resp(&String::from("about_page"), 60 * 200, transactions::get_about)
}
