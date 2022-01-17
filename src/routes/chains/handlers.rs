use crate::cache::cache_operations::RequestCached;
use crate::common::models::backend::chains::ChainInfo as BackendChainInfo;
use crate::common::models::page::{Page, PageMetadata};
use crate::config::{chain_info_cache_duration, chain_info_request_timeout};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::chains::models::ChainInfo as ServiceChainInfo;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::urls::build_absolute_uri;

pub async fn get_chains_paginated(
    context: &RequestContext,
    cursor: &Option<String>,
) -> ApiResult<Page<ServiceChainInfo>> {
    let page_metadata = cursor
        .as_ref()
        .map(|cursor| PageMetadata::from_cursor(cursor));
    let url = config_uri!(
        "/v1/chains/?{}",
        page_metadata
            .as_ref()
            .unwrap_or(&PageMetadata::default())
            .to_url_string()
    );

    let body = RequestCached::new_from_context(url, context)
        .request_timeout(chain_info_request_timeout())
        .cache_duration(chain_info_cache_duration())
        .execute()
        .await?;

    let page = serde_json::from_str::<Page<BackendChainInfo>>(&body)?;

    Ok(page.map_inner(|link| map_link(context, link)))
}

pub async fn get_single_chain(
    context: &RequestContext,
    chain_id: &str,
) -> ApiResult<ServiceChainInfo> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    Ok(info_provider.chain_info().await?.into())
}

fn map_link(context: &RequestContext, original_link: Option<String>) -> Option<String> {
    original_link.as_ref().map(|link| {
        let cursor =
            PageMetadata::from_cursor(link.split("?").collect::<Vec<&str>>().get(1).unwrap_or(&""))
                .to_url_string();
        let uri = build_absolute_uri(
            context,
            uri!(crate::routes::chains::routes::get_chains(Some(cursor))),
        );
        String::from(uri)
    })
}
