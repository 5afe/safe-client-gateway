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
    let trusted = trusted.unwrap_or(false);
    let exclude_spam = exclude_spam.unwrap_or(true);

    let result = CacheResponse::new()
        .key(context.uri())
        .timeout(balances_cache_duration())
        .resp_generator(|| {
            balances(
                &context,
                safe_address.as_str(),
                fiat.as_str(),
                trusted,
                exclude_spam,
            )
        })
        .execute(context.cache());
    result
}

#[get("/v1/balances/supported-fiat-codes")]
pub fn get_supported_fiat(context: Context) -> ApiResult<content::Json<String>> {
    context
        .cache()
        .cache_resp(&context.uri(), request_cache_duration(), || {
            fiat_codes(&context)
        })
}
