use crate::config::{request_cache_duration};
use crate::utils::context::Context;
use crate::config::{base_transaction_service_url};
use crate::utils::cache::CacheExt;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/v1/safes/<safe_address>/collectibles")]
pub fn list(context: Context, safe_address: String) -> ApiResult<content::Json<String>> {
    let url = format!(
        "{}/v1/safes/{}/collectibles",
        base_transaction_service_url(),
        safe_address
    );
    Ok(content::Json(context.cache().request_cached(&context.client(), url.as_str(), request_cache_duration())?))
}