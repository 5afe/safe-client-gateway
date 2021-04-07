use crate::cache::cache::CacheExt;
use crate::config::request_cache_duration;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/health")]
pub fn health(context: Context) -> ApiResult<content::Json<String>> {
    context
        .cache()
        .cache_resp("/health", request_cache_duration(), || Ok(String::new()))
}
