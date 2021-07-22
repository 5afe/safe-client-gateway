extern crate reqwest;

use crate::config::{build_number, version};
use crate::models::service::about::About;
use crate::models::service::safes::Implementation;
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
) -> ApiResult<Vec<Implementation>> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);
    Ok(info_provider
        .master_copies()
        .await?
        .into_iter()
        .map(|master_copy| master_copy.into())
        .collect())
}
