use crate::cache::cache_operations::CacheResponse;
use crate::cache::manager::ChainCache;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket_okapi::openapi;

#[openapi(tag = "Health")]
#[get("/health")]
pub async fn health(context: RequestContext) -> ApiResult<content::RawJson<String>> {
    CacheResponse::new(&context, ChainCache::Other)
        .resp_generator(|| async { Ok(String::new()) })
        .execute()
        .await
}
