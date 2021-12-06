use rocket::response::content;

use crate::cache::cache_operations::CacheResponse;
use crate::common::models::page::Page;
use crate::config::chain_info_response_cache_duration;
use crate::routes::chains::handlers::{get_chains_paginated, get_single_chain};
use crate::routes::chains::models::ChainInfo;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;

/**
 * `/v1/chains/<chain_id>/` <br/>
 * Returns [ChainInfo]
 *
 * # Chains
 *
 * This endpoint returns the [ChainInfo] for a given `chainId`
 *
 * ## Path
 *
 * - `/v1/chains/<chain_id>/`returns the [ChainInfo] for `<chain_id>`
 *
 */
#[get("/v1/chains/<chain_id>")]
pub async fn get_chain(
    context: RequestContext,
    chain_id: String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(&context)
        .duration(chain_info_response_cache_duration())
        .resp_generator(|| get_single_chain(&context, &chain_id))
        .execute()
        .await
}

/**
 * `/v1/chains/` <br/>
 * Returns a [Page] of [ChainInfo]
 *
 * # Chains
 *
 * Returns a paginated list of all the supported [ChainInfo]
 *
 * ## Path
 *
 * - `/v1/chains/` Returns the [ChainInfo] for our services supported networks
 *
 */
#[get("/v1/chains?<limit>")]
pub async fn get_chains(
    context: RequestContext,
    limit: Option<String>,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(&context)
        .duration(chain_info_response_cache_duration())
        .resp_generator(|| get_chains_paginated(&context, &limit))
        .execute()
        .await
}
