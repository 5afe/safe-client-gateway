use crate::config::{balances_cache_duration, request_cache_duration};
use crate::providers::info::DefaultInfoProvider;
use crate::services::balances::*;
use crate::utils::cache::CacheExt;
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
    context
        .cache()
        .cache_resp(&context.uri(), balances_cache_duration(), || {
            balances(
                &context,
                safe_address.as_str(),
                fiat.as_str(),
                trusted,
                exclude_spam,
            )
        })
}

#[get("/v1/balances/supported-fiat-codes")]
pub fn get_supported_fiat(context: Context) -> ApiResult<content::Json<String>> {
    context
        .cache()
        .cache_resp(&context.uri(), request_cache_duration(), || {
            let info_provider = DefaultInfoProvider::new(&context);
            Ok(info_provider.available_currency_codes()?)
        })
}
