use crate::models::backend::balances::Balance;
use crate::utils::errors::ApiResult;
use crate::utils::context::Context;
use crate::config::request_cache_duration;
use rocket::response::content;
use crate::utils::cache::CacheExt;
use crate::services::balances::*;

#[get("/v1/safes/<safe_address>/balances/usd")]
pub fn get_balances_usd(context: Context, safe_address: String) -> ApiResult<content::Json<String>> {
    context.cache().cache_resp(&context.uri(), request_cache_duration(), || {
        balances_usd(safe_address.as_str())
    })
}