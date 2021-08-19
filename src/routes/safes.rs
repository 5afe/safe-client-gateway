use crate::cache::cache_operations::CacheResponse;
use crate::config::owners_for_safes_cache_duration;
use crate::services::safes::{get_owners_for_safe, get_safe_info_ex};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

/**
 * `/v1/chains/<chain_id>/safes/<safe_address>` <br />
 * Returns [SafeState](crate::models::service::safes::SafeState)
 */
#[get("/v1/chains/<chain_id>/safes/<safe_address>")]
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

/**
 * `/v1/chains/<chain_id>/owners/<safe_address>/safes` <br/>
 * Returns [Vec] of [String]
 *
 * Returns a list of owner addresses for a given Safe
 */
#[get("/v1/chains/<chain_id>/owners/<safe_address>/safes")]
pub async fn get_owners(
    context: Context<'_>,
    chain_id: String,
    safe_address: String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .resp_generator(|| get_owners_for_safe(&context, &chain_id, &safe_address))
        .duration(owners_for_safes_cache_duration())
        .execute(context.cache())
        .await
}
