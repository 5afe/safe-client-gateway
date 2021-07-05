use crate::cache::cache_operations::CacheResponse;
use crate::config::chain_info_cache_duration;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::services::chains::get_chains_paginated;
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
    CacheResponse::new(context.uri())
        .resp_generator(|| get_chains_paginated(&context))
        .execute(context.cache())
        .await
}
