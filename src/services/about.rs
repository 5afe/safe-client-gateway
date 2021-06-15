extern crate reqwest;

use crate::config::{build_number, version};
use crate::models::service::about::About;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;

pub async fn about(context: &Context<'_>, chain_id: &str) -> ApiResult<About> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);
    let chain_info = info_provider.chain_info().await?;
    Ok(About {
        transaction_service_base_url: chain_info.transaction_service,
        name: env!("CARGO_PKG_NAME").to_string(),
        version: version(),
        build_number: build_number(),
    })
}
