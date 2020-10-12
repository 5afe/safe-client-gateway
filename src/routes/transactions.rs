use crate::config::request_cache_duration;
use crate::services::transactions_details;
use crate::services::transactions_list;
use crate::services::tx_confirmation;
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket_contrib::json::Json;
use crate::models::service::transactions::tx_requests::TxConfirmationRequest;

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

#[post("/v1/transactions/<tx_id>/confirmations", format = "json", data = "<confirmation_request>")]
pub fn submit_confirmation(context: Context, tx_id: String, confirmation_request: Json<TxConfirmationRequest>) -> ApiResult<content::Json<String>> {
    context.cache().cache_resp(&context.uri(), request_cache_duration(), || {
        tx_confirmation::submit_confirmation(&context, &tx_id, &confirmation_request)
    })
}
