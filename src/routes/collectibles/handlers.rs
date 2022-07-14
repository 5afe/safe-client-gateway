use std::cmp::max;

use crate::cache::cache_operations::RequestCached;
use crate::cache::manager::ChainCache;
use crate::common::models::page::{Page, PageMetadata};
use crate::config::collectibles_request_timeout;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::collectibles::models::Collectible as ServiceCollectible;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::urls::build_absolute_uri;
use rocket::response::content::RawJson;

pub async fn collectibles(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    trusted: Option<bool>,
    exclude_spam: Option<bool>,
) -> ApiResult<RawJson<String>> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);

    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/collectibles/?trusted={}&exclude_spam={}",
        safe_address,
        trusted.unwrap_or(false),
        exclude_spam.unwrap_or(true)
    )?;

    Ok(RawJson(
        RequestCached::new_from_context(url, &context, ChainCache::from(chain_id))
            .request_timeout(collectibles_request_timeout())
            .execute()
            .await?,
    ))
}

/// Returns paginated collectibles.
pub async fn collectibles_paginated(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    trusted: Option<bool>,
    exclude_spam: Option<bool>,
    cursor: &Option<String>,
) -> ApiResult<Page<ServiceCollectible>> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);
    let page_metadata = cursor
        .as_ref()
        .map(|cursor| PageMetadata::from_cursor(cursor));
    let url = core_uri!(
        info_provider,
        "/v2/safes/{}/collectibles/?{}&trusted={}&exclude_spam={}", /* paginated service in core is behind v2 api. */
        safe_address,
        page_metadata
            .as_ref()
            .unwrap_or(&PageMetadata::default())
            .to_url_string(),
        trusted.unwrap_or(false),
        exclude_spam.unwrap_or(true)
    )?;

    let body = RequestCached::new_from_context(url, &context, ChainCache::from(chain_id))
        .request_timeout(collectibles_request_timeout())
        .execute()
        .await?;

    let page = serde_json::from_str::<Page<ServiceCollectible>>(&body)?;

    Ok(page.map_inner(|link| {
        map_link(
            context,
            link,
            chain_id.to_string(),
            safe_address.to_string(),
            trusted,
            exclude_spam,
        )
    }))
}

fn map_link(
    context: &RequestContext,
    original_link: Option<String>,
    chain_id: String,
    safe_address: String,
    trusted: Option<bool>,
    exclude_spam: Option<bool>,
) -> Option<String> {
    original_link.as_ref().map(|link| {
        let cursor =
            PageMetadata::from_cursor(link.split("?").collect::<Vec<&str>>().get(1).unwrap_or(&""))
                .to_url_string();
        let uri = build_absolute_uri(
            context,
            uri!(
                crate::routes::collectibles::routes::get_collectibles_paginated(
                    chain_id,
                    safe_address,
                    Some(cursor),
                    trusted,
                    exclude_spam
                )
            ),
        );
        String::from(uri)
    })
}
