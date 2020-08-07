use crate::config::{request_cache_duration};
use crate::utils::context::Context;
use crate::services::transactions_details;
use crate::services::transactions_list;
use rocket::response::content;
use anyhow::Result;

#[get("/v1/safes/<safe_address>/transactions?<next>")]
pub fn all(context: Context, safe_address: String, next: Option<String>) -> Result<content::Json<String>> {
    context.cache().cache_resp(&context.uri(), request_cache_duration(), || {
        transactions_list::get_all_transactions(&context, &safe_address, &next)
    })
}

#[get("/v1/transactions/<details_id>")]
pub fn details(context: Context, details_id: String) -> Result<content::Json<String>> {
    context.cache().cache_resp(&context.uri(), request_cache_duration(), || {
        transactions_details::get_transactions_details(&context, &details_id)
    })
}
