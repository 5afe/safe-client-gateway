use crate::config::request_cache_duration;
use crate::services::balances::*;
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/v1/safes/<safe_address>/balances/<fiat>")]
pub fn get_balances(
    context: Context,
    safe_address: String,
    fiat: String,
) -> ApiResult<content::Json<String>> {
    context
        .cache()
        .cache_resp(&context.uri(), request_cache_duration(), || {
            balances(&context, safe_address.as_str(), fiat.as_str())
        })
}
