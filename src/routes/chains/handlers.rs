use crate::cache::cache_operations::RequestCached;
use crate::common::models::backend::chains::ChainInfo as BackendChainInfo;
use crate::common::models::page::{Page, PageMetadata};
use crate::config::{chain_info_cache_duration, chain_info_request_timeout};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::chains::models::ChainInfo as ServiceChainInfo;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;

pub async fn get_chains_paginated(
    context: &RequestContext,
    limit: &Option<String>,
) -> ApiResult<Page<ServiceChainInfo>> {
    let url = config_uri!(
        "/v1/chains/?limit={}",
        limit.as_ref().unwrap_or(&"".to_string())
    );

    let body = RequestCached::new_from_context(url, context)
        .request_timeout(chain_info_request_timeout())
        .cache_duration(chain_info_cache_duration())
        .execute()
        .await?;

    let page = serde_json::from_str::<Page<BackendChainInfo>>(&body)?;

    Ok(page.map_inner(map_link))
}

pub async fn get_single_chain(
    context: &RequestContext,
    chain_id: &str,
) -> ApiResult<ServiceChainInfo> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    Ok(info_provider.chain_info().await?.into())
}

fn map_link(original_link: Option<String>) -> Option<String> {
    original_link
}
