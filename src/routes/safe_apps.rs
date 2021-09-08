use crate::cache::cache_operations::CacheResponse;
// use crate::config::owners_for_safes_cache_duration;
// use crate::services::safes::{get_owners_for_safe, get_safe_info_ex};
use crate::services::safe_apps::safe_apps;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/v1/chains/<chain_id>/safes-apps")]
pub async fn get_safe_apps(
    context: Context<'_>,
    chain_id: String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .resp_generator(|| safe_apps(&context, &chain_id))
        .duration(1)
        .execute(context.cache())
        .await
}
