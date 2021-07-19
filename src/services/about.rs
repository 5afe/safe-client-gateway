extern crate reqwest;

use crate::cache::cache_operations::RequestCached;
use crate::config::{about_cache_duration, build_number, default_request_timeout, version};
use crate::models::backend::safes::MasterCopy;
use crate::models::service::about::About;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;

pub async fn about(context: &Context<'_>, chain_id: &str) -> ApiResult<About> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);
    let chain_info = info_provider.chain_info().await?;
    Ok(About {
        transaction_service_base_uri: chain_info.transaction_service,
        name: env!("CARGO_PKG_NAME").to_string(),
        version: version(),
        build_number: build_number(),
    })
}

pub async fn get_master_copies(
    context: &Context<'_>,
    chain_id: &str,
) -> ApiResult<Vec<MasterCopy>> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);
    let url = core_uri!(info_provider, "/v1/about/master-copies/")?;
    request_master_copies(&context, url).await
}

pub async fn request_master_copies(
    context: &Context<'_>,
    url: String,
) -> ApiResult<Vec<MasterCopy>> {
    let body = RequestCached::new(url)
        .cache_duration(about_cache_duration())
        .request_timeout(default_request_timeout())
        .execute(context.client(), context.cache())
        .await?;

    Ok(serde_json::from_str(&body)?)
}
