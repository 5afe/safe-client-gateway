use crate::cache::cache_operations::CacheResponse;
use crate::services::safes::get_safe_info_ex;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

/**
 * `/<chain_id>/v1/safes/<safe_address>` <br />
 * Returns [SafeState](crate::models::service::safes::SafeState)
 */
#[get("/<chain_id>/v1/safes/<safe_address>")]
pub async fn get_safe_info(
    context: Context<'_>,
    chain_id: String,
    safe_address: String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .resp_generator(|| get_safe_info_ex(&context, &chain_id, &safe_address))
        .execute(context.cache())
        .await
}
