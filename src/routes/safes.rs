use crate::cache::cache_operations::CacheResponse;
use crate::services::safes::get_safe_info_ex;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

/**
 * `/v1/safes/<safe_address>` <br />
 * Returns [crate::models::service::safes::SafeState]
 */
#[get("/v1/safes/<safe_address>")]
pub async fn safe_info(
    context: Context<'_>,
    safe_address: String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .resp_generator(|| get_safe_info_ex(&context, &safe_address))
        .execute(context.cache())
        .await
}
