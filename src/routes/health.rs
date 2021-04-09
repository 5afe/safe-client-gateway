use crate::cache::cache::CacheExt;
use crate::cache::cache_operations::CacheResponse;
use crate::config::request_cache_duration;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/health")]
pub fn health(context: Context) -> ApiResult<content::Json<String>> {
    CacheResponse::new()
        .key(String::from("/health"))
        .resp_generator(|| Ok(String::new()))
        .execute(context.cache())
}
