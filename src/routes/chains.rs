use crate::cache::cache_operations::{CacheResponse, RequestCached};
use crate::config::{
    base_config_service_url, chain_info_cache_duration, chain_info_request_timeout,
};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

/**
 * `/v1/chains/<chain_id>/` <br/>
 * Returns [ChainInfo](crate::models::chains::ChainInfo)
 *
 * # Chains
 *
 * This endpoint returns the [ChainInfo](crate::models::chains::ChainInfo) for a given `chainId`
 *
 * ## Path
 *
 * - `/v1/chains/<chain_id>/`returns the `ChainInfo` for `<chain_id>`
 *
 */
#[get("/v1/chains/<chain_id>")]
pub async fn get_chain(context: Context<'_>, chain_id: String) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .duration(chain_info_cache_duration())
        .resp_generator(async || {
            let info_provider = DefaultInfoProvider::new(&chain_id, &context);
            info_provider.chain_info().await
        })
        .execute(context.cache())
        .await
}

/**
 * `/v1/chains/` <br/>
 * Returns a [Page](crate::models::commons::Page) of [ChainInfo](crate::models::chains::ChainInfo)
 *
 * # Chains
 *
 * Returns a paginated list of all the supported [ChainInfo](crate::models::chains::ChainInfo)
 *
 * ## Path
 *
 * - `/v1/chains/` Returns the `ChainInfo` for our services supported networks
 *
 */
#[get("/v1/chains")]
pub async fn get_chains(context: Context<'_>) -> ApiResult<content::Json<String>> {
    let mut url = reqwest::Url::parse(base_config_service_url().as_str())
        .expect("Bad base config service url");
    url.path_segments_mut()
        .expect("Cannot add chain_id to path")
        .extend(["v1", "chains"]);

    Ok(content::Json(
        RequestCached::new(url.to_string())
            .request_timeout(chain_info_request_timeout())
            .cache_duration(chain_info_cache_duration())
            .execute(context.client(), context.cache())
            .await?,
    ))
}
