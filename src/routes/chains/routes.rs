use crate::cache::cache_operations::CacheResponse;
use crate::cache::manager::ChainCache;
use crate::config::chain_info_response_cache_duration;
use crate::routes::chains::handlers::{get_chains_paginated, get_single_chain};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::response::content;

/// `/v1/chains/<chain_id>/` <br/>
/// Returns [ChainInfo](crate::routes::chains::models::ChainInfo)
///
/// # Chains
///
/// This endpoint returns the [ChainInfo](crate::routes::chains::models::ChainInfo) for a given `chain_id`
///
/// ## Path
///
/// - `/v1/chains/<chain_id>/`returns the `ChainInfo` for `<chain_id>`
#[get("/v1/chains/<chain_id>")]
pub async fn get_chain(
    context: RequestContext,
    chain_id: String,
) -> ApiResult<content::RawJson<String>> {
    CacheResponse::new(&context, ChainCache::from(chain_id.as_str()))
        .duration(chain_info_response_cache_duration())
        .resp_generator(|| get_single_chain(&context, &chain_id))
        .execute()
        .await
}

/// `/v1/chains/` <br/>
/// Returns a [Page](crate::common::models::page::Page) of [ChainInfo](crate::routes::chains::models::ChainInfo)
///
/// # Chains
///
/// Returns a paginated list of all the supported [ChainInfo](crate::routes::chains::models::ChainInfo)
///
/// ## Path
///
/// - `/v1/chains/` Returns the `ChainInfo` for our services supported networks
#[get("/v1/chains?<cursor>")]
pub async fn get_chains(
    context: RequestContext,
    cursor: Option<String>,
) -> ApiResult<content::RawJson<String>> {
    CacheResponse::new(&context, ChainCache::Other)
        .duration(chain_info_response_cache_duration())
        .resp_generator(|| get_chains_paginated(&context, &cursor))
        .execute()
        .await
}
