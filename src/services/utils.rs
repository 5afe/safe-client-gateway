use crate::config::default_request_timeout;
use crate::models::commons::DataDecoded;
use crate::models::service::utils::{
    DataDecoderRequest, SafeTransactionEstimation, SafeTransactionEstimationRequest,
};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use std::collections::HashMap;
use std::time::Duration;

pub async fn request_data_decoded(
    context: &Context<'_>,
    chain_id: &str,
    data_decoder_request: &DataDecoderRequest,
) -> ApiResult<DataDecoded> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);
    let url = core_uri!(info_provider, "/v1/data-decoder/")?;
    let mut json = HashMap::new();
    json.insert("data", &data_decoder_request.data);

    let response = context
        .client()
        .post(url)
        .json(&json)
        .timeout(Duration::from_millis(default_request_timeout()))
        .send()
        .await?;

    Ok(serde_json::from_str::<DataDecoded>(
        &response.text().await?,
    )?)
}

pub async fn estimate_safe_tx_gas(
    context: &Context<'_>,
    chain_id: &str,
    safe_address: &str,
    safe_transaction_estimation_request: &SafeTransactionEstimationRequest,
) -> ApiResult<SafeTransactionEstimation> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/multisig-transactions/estimations/",
        safe_address
    )?;

    let response = context
        .client()
        .post(url)
        .json(safe_transaction_estimation_request)
        .timeout(Duration::from_millis(default_request_timeout()))
        .send()
        .await?;

    Ok(serde_json::from_str::<SafeTransactionEstimation>(
        &response.text().await?,
    )?)
}
