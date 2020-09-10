use crate::utils::context::Context;
use crate::config::{about_cache_duration, base_transaction_service_url};
use crate::utils::cache::CacheExt;
use crate::utils::errors::ApiResult;

#[get("/health")]
pub fn health(context: Context) -> ApiResult<String> {
    let url = format!("{}/about", base_transaction_service_url());
    context.cache().request_cached(context.client(), &url, about_cache_duration())
}