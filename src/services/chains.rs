use crate::cache::cache_operations::RequestCached;
use crate::config::{chain_info_cache_duration, chain_info_request_timeout};
use crate::models::chains::ChainInfo;
use crate::models::commons::Page;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;

pub async fn get_chains_paginated(
    context: &Context<'_>,
    limit: &Option<String>,
) -> ApiResult<Page<ChainInfo>> {
    let url = config_uri!(
        "/v1/chains/?limit={}",
        limit.as_ref().unwrap_or(&"".to_string())
    );

    let body = RequestCached::new(url)
        .request_timeout(chain_info_request_timeout())
        .cache_duration(chain_info_cache_duration())
        .execute(context.client(), context.cache())
        .await?;

    let page: Page<ChainInfo> = serde_json::from_str::<Page<ChainInfo>>(&body)?;
    Ok(page)
}
