use crate::utils::context::Context;
use crate::config::request_cache_duration;
use crate::utils::cache::CacheExt;
use crate::utils::errors::ApiResult;

#[get("/health")]
pub fn health(context: Context) -> ApiResult<String> {
    context.cache().cache_string("/health", request_cache_duration())
}