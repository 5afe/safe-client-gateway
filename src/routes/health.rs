use crate::utils::context::Context;
use crate::utils::cache::CacheExt;
use crate::utils::errors::ApiResult;
use crate::config::request_cache_duration;
use rocket::response::content;

#[get("/health")]
pub fn health(context: Context) -> ApiResult<content::Json<String>> {
    context.cache().cache_resp("/health", request_cache_duration(), || Ok(String::new()))
}