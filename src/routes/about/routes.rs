use crate::cache::cache_operations::CacheResponse;
use crate::cache::Cache;
use crate::config::{about_cache_duration, webhook_token};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::about::handlers;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

/**
 * `/v1/chains/<chain_id>/about` <br />
 * Returns [ChainAbout](crate::models::handlers::about::ChainAbout)
 *
 * # Chain's About
 *
 * The about endpoint provides information of the environmental variables set for the instance of `safe-client-gateway`. This would allow to identify on which commit and version the last deployment happened and to which safe transaction handlers backend environment the current instance of the gateway is pointing to.
 *
 * ## Path
 *
 * `/v1/chains/<chain_id>/about`
 *
 * ## Query parameters
 *
 * There are no query parameters for this endpoint
 */
#[get("/v1/chains/<chain_id>/about")]
pub async fn get_chains_about(
    context: Context<'_>,
    chain_id: String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .duration(about_cache_duration())
        .resp_generator(|| handlers::chains_about(&context, &chain_id))
        .execute(context.cache())
        .await
}

/**
 * `/about` <br />
 * [About](crate::models::handlers::about::About)
 *
 * # About
 *
 * This endpoint is chain independent, and returns non cached information regarding this current CGW instance.
 *
 * ## Path
 *
 * `/about`
 *
 */
#[get("/about")]
pub async fn get_about() -> ApiResult<content::Json<String>> {
    Ok(content::Json(serde_json::to_string(&handlers::about())?))
}
/**
 * `/v1/chains/<chain_id>/about/master-copies` <br />
 * Returns a list of `MasterCopy`
 *
 * # Master Copies
 *
 * This endpoint returns a list of `MasterCopy` objects just as documented in the core services
 *
 * ## Path
 *
 * `/v1/chains/<chain_id>/about/master-copies` where `chain_id` correspond to the chain id of the desired network
 *
 * ## Sample Json
 * <details>
 * <summary>JSON sample</summary>
 *
 * ```json
 * [
 *   {
 *     "address": "0x8942595A2dC5181Df0465AF0D7be08c8f23C93af",
 *     "version": "0.1.0"
 *   }
 * ]
 * ```
 * </details>
 */
#[get("/v1/chains/<chain_id>/about/master-copies")]
pub async fn get_master_copies(
    context: Context<'_>,
    chain_id: String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .duration(about_cache_duration())
        .resp_generator(|| handlers::get_master_copies(&context, chain_id.as_str()))
        .execute(context.cache())
        .await
}

#[doc(hidden)]
#[get("/v1/chains/<chain_id>/about/backbone")]
pub async fn backbone(context: Context<'_>, chain_id: String) -> ApiResult<content::Json<String>> {
    let info_provider = DefaultInfoProvider::new(chain_id.as_str(), &context);
    let url = core_uri!(info_provider, "/v1/about/")?;
    Ok(content::Json(
        context.client().get(&url).send().await?.text().await?,
    ))
}

#[doc(hidden)]
#[get("/about/redis/<token>")]
pub fn redis(context: Context<'_>, token: String) -> ApiResult<String> {
    if token != webhook_token() {
        bail!("Invalid token");
    }
    Ok(context.cache().info().unwrap_or(String::new()))
}
