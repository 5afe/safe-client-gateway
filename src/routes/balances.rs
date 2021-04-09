use crate::cache::cache::CacheExt;
use crate::cache::cache_operations::CacheResponse;
use crate::config::{balances_cache_duration, request_cache_duration};
use crate::services::balances::*;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/v1/safes/<safe_address>/balances/<fiat>?<trusted>&<exclude_spam>")]
pub fn get_balances(
    context: Context,
    safe_address: String,
    fiat: String,
    trusted: Option<bool>,
    exclude_spam: Option<bool>,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new()
        .key(context.uri())
        .duration(balances_cache_duration())
        .resp_generator(|| {
            balances(
                &context,
                safe_address.as_str(),
                fiat.as_str(),
                trusted.unwrap_or(false),
                exclude_spam.unwrap_or(true),
            )
        })
        .execute(context.cache())
}

#[get("/v1/balances/supported-fiat-codes")]
pub fn get_supported_fiat(context: Context) -> ApiResult<content::Json<String>> {
    context
        .cache()
        .cache_resp(&context.uri(), request_cache_duration(), || {
            fiat_codes(&context)
        })
}
