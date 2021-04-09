use crate::cache::cache::CacheExt;
use crate::cache::cache_operations::{CacheResponse, RequestCached};
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
    let url = format!(
        "{}/v1/safes/{}/collectibles/?trusted={}&exclude_spam={}",
        base_transaction_service_url(),
        safe_address,
        trusted.unwrap_or(false),
        exclude_spam.unwrap_or(true)
    );

    Ok(content::Json(
        RequestCached::new()
            .url(url)
            .request_timeout(30000) //TODO: extract to config
            .execute(context.client(), context.cache())?,
    ))
}
