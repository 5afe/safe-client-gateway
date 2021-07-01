use crate::cache::cache_operations::RequestCached;
use crate::config::{
    base_config_service_url, chain_info_cache_duration, chain_info_request_timeout,
};
use crate::models::chains::ChainInfo;
use crate::models::commons::Page;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use reqwest::Url;

pub async fn get_chains_paginated(
    context: &Context<'_>,
    limit: &Option<String>,
) -> ApiResult<Page<ChainInfo>> {
    let mut queries: Vec<(String, String)> = vec![];
    if let Some(limit) = limit {
        queries.push(("limit".to_string(), limit.to_string()))
    }
    let mut url = Url::parse_with_params(base_config_service_url().as_str(), queries)
        .expect("Bad base config service url");
    url.path_segments_mut()
        .expect("Cannot add chain_id to path")
        .extend(["v1", "chains"]);

    let body = RequestCached::new(url.to_string())
        .request_timeout(chain_info_request_timeout())
        .cache_duration(chain_info_cache_duration())
        .execute(context.client(), context.cache())
        .await?;

    let page: Page<ChainInfo> = serde_json::from_str::<Page<ChainInfo>>(&body)?;
    Ok(page)
}
