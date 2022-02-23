use rocket::response::content;

use crate::cache::cache_operations::CacheResponse;
use crate::common::routes::authorization::AuthorizationToken;
use crate::config::about_cache_duration;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::about::handlers;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::Request;

/// `/v1/chains/<chain_id>/about` <br />
/// Returns [ChainAbout](crate::routes::about::models::ChainAbout)
///
/// # Chain's About
///
/// The about endpoint provides information of the environmental variables set for the instance of `safe-client-gateway`. This would allow to identify on which commit and version the last deployment happened and to which safe transaction handlers backend environment the current instance of the gateway is pointing to.
///
/// ## Path
///
/// `/v1/chains/<chain_id>/about`
///
/// ## Query parameters
///
/// There are no query parameters for this endpoint
#[get("/v1/chains/<chain_id>/about")]
pub async fn get_chains_about(
    context: RequestContext,
    chain_id: String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(&context)
        .duration(about_cache_duration())
        .resp_generator(|| handlers::chains_about(&context, &chain_id))
        .execute()
        .await
}

/// `/about` <br />
/// [About](crate::routes::about::models::About)
///
/// # About
///
/// This endpoint is chain independent, and returns non cached information regarding this current CGW instance.
///
/// ## Path
///
/// `/about`
#[get("/about")]
pub async fn get_about() -> ApiResult<content::Json<String>> {
    Ok(content::Json(serde_json::to_string(&handlers::about())?))
}
/// `/v1/chains/<chain_id>/about/master-copies` <br />
/// Returns a list of `MasterCopy`
///
/// # Master Copies
///
/// This endpoint returns a list of `MasterCopy` objects just as documented in the core services
///
/// ## Path
///
/// `/v1/chains/<chain_id>/about/master-copies` where `chain_id` correspond to the chain id of the desired network
///
/// ## Sample Json
/// <details>
/// <summary>JSON sample</summary>
///
/// ```json
/// [
///   {
///     "address": "0x8942595A2dC5181Df0465AF0D7be08c8f23C93af",
///     "version": "0.1.0"
///   }
/// ]
/// ```
/// </details>
#[get("/v1/chains/<chain_id>/about/master-copies")]
pub async fn get_master_copies(
    context: RequestContext,
    chain_id: String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(&context)
        .duration(about_cache_duration())
        .resp_generator(|| handlers::get_master_copies(&context, chain_id.as_str()))
        .execute()
        .await
}

#[doc(hidden)]
#[get("/v1/chains/<chain_id>/about/backbone")]
pub async fn backbone(
    context: RequestContext,
    chain_id: String,
) -> ApiResult<content::Json<String>> {
    let client = context.http_client();
    let info_provider = DefaultInfoProvider::new(chain_id.as_str(), &context);
    let url = core_uri!(info_provider, "/v1/about/")?;
    let request = Request::new(url);
    Ok(content::Json(client.get(request).await?.body))
}

#[doc(hidden)]
#[get("/about/redis")]
pub fn redis(context: RequestContext, _token: AuthorizationToken) -> ApiResult<String> {
    Ok(context.cache().info().unwrap_or(String::new()))
}
