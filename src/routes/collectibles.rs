use crate::cache::cache::CacheExt;
use crate::config::request_cache_duration;
use crate::config::{base_transaction_service_url, request_error_cache_timeout};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/v1/safes/<safe_address>/collectibles?<trusted>&<exclude_spam>")]
pub fn list(
    context: Context,
    safe_address: String,
    trusted: Option<bool>,
    exclude_spam: Option<bool>,
) -> ApiResult<content::Json<String>> {
    let trusted = trusted.unwrap_or(false);
    let exclude_spam = exclude_spam.unwrap_or(true);
    let url = format!(
        "{}/v1/safes/{}/collectibles/?trusted={}&exclude_spam={}",
        base_transaction_service_url(),
        safe_address,
        trusted,
        exclude_spam
    );
    Ok(content::Json(context.cache().request_cached(
        &context.client(),
        url.as_str(),
        request_cache_duration(),
        request_error_cache_timeout(),
    )?))
}
