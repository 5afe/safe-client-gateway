use crate::cache::cache_operations::CacheResponse;
use crate::utils::context::{RequestContext, UNSPECIFIED_CHAIN};
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/health")]
pub async fn health(context: RequestContext) -> ApiResult<content::Json<String>> {
    CacheResponse::new(&context, UNSPECIFIED_CHAIN)
        .resp_generator(|| async { Ok(String::new()) })
        .execute()
        .await
}
