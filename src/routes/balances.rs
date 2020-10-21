use crate::config::request_cache_duration;
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
    let exclude_spam = exclude_spam.unwrap_or(false);
    context
        .cache()
        .cache_resp(&context.uri(), request_cache_duration(), || {
            balances(
                &context,
                safe_address.as_str(),
                fiat.as_str(),
                trusted,
                exclude_spam,
            )
        })
}
