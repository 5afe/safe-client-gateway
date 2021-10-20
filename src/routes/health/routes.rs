use crate::cache::cache_operations::CacheResponse;
use crate::utils::context::{Context, RequestContext};
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/health")]
pub async fn health(context: RequestContext) -> ApiResult<content::Json<String>> {
    CacheResponse::new(&context)
        .resp_generator(|| async { Ok(String::new()) })
        .execute()
        .await
}
