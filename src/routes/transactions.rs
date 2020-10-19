use crate::config::request_cache_duration;
use crate::services::transactions_details;
use crate::services::transactions_list;
use crate::services::tx_confirmation;
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use crate::models::service::transactions::requests::ConfirmationRequest;
use rocket::response::content;
use rocket_contrib::json::Json;

#[get("/v1/safes/<safe_address>/transactions?<page_url>")]
pub fn all(
    context: Context,
    safe_address: String,
    page_url: Option<String>,
) -> ApiResult<content::Json<String>> {
    context
        .cache()
        .cache_resp(&context.uri(), request_cache_duration(), || {
            transactions_list::get_all_transactions(&context, &safe_address, &page_url)
        })
}

#[get("/v1/transactions/<details_id>")]
pub fn details(context: Context, details_id: String) -> ApiResult<content::Json<String>> {
    context
        .cache()
        .cache_resp(&context.uri(), request_cache_duration(), || {
            transactions_details::get_transactions_details(&context, &details_id)
        })
}

#[post("/v1/transactions/<safe_tx_hash>/confirmations")]
pub fn submit_confirmation(context: Context, safe_tx_hash: String) -> ApiResult<content::Json<String>> {
    tx_confirmation::submit_confirmation(&context, &safe_tx_hash).and_then(|_| {
        context.cache().cache_resp(&context.uri(), request_cache_duration(), || {
            transactions_details::get_transactions_details(&context, &safe_tx_hash)
        })
    })
}
