use crate::cache::cache_operations::RequestCached;
use crate::common::models::backend::chains::ChainInfo as BackendChainInfo;
use crate::common::models::Page;
use crate::config::{chain_info_cache_duration, chain_info_request_timeout};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::chains::models::ChainInfo as ServiceChainInfo;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;

pub async fn get_chains_paginated(
    context: &Context<'_>,
    limit: &Option<String>,
) -> ApiResult<Page<ServiceChainInfo>> {
    let url = config_uri!(
        "/v1/chains/?limit={}",
        limit.as_ref().unwrap_or(&"".to_string())
    );

    let body = RequestCached::new(url)
        .request_timeout(chain_info_request_timeout())
        .cache_duration(chain_info_cache_duration())
        .execute(context.client(), context.cache())
        .await?;

    let page = serde_json::from_str::<Page<BackendChainInfo>>(&body)?;
    Ok(page.map_inner())
}

pub async fn get_single_chain(
    context: &Context<'_>,
    chain_id: &str,
) -> ApiResult<ServiceChainInfo> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    Ok(info_provider.chain_info().await?.into())
}
