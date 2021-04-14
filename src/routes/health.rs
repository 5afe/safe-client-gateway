use crate::cache::cache_operations::CacheResponse;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/health")]
pub async fn health(context: Context<'_>) -> ApiResult<content::Json<String>> {
    CacheResponse::new(String::from("/health"))
        .resp_generator(|| async { Ok(String::new()) })
        .execute(context.cache())
        .await
}
