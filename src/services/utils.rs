use crate::config::default_request_timeout;
use crate::models::backend::transactions::{
    MultisigTransaction as BackendMultisigTransaction,
    SafeTransactionEstimation as BackendSafeTransactionEstimation,
};
use crate::models::commons::{DataDecoded, Page};
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
    let estimation_url = core_uri!(
        info_provider,
        "/v1/safes/{}/multisig-transactions/estimations/",
        safe_address
    )?;
    let latest_multisig_tx_url = core_uri!(
        info_provider,
        "/v1/safes/{}/multisig-transactions/?ordering=nonce&trusted=true&limit=1",
        safe_address
    )?;

    let latest_nonce = fetch_latest_nonce(&context, latest_multisig_tx_url).await?;
    let safe_tx_gas = fetch_estimation(
        &context,
        estimation_url,
        safe_transaction_estimation_request,
    )
    .await?;

    Ok(SafeTransactionEstimation {
        latest_nonce,
        safe_tx_gas,
    })
}

async fn fetch_estimation(
    context: &Context<'_>,
    request_url: String,
    safe_transaction_estimation_request: &SafeTransactionEstimationRequest,
) -> ApiResult<String> {
    let estimation_response = context
        .client()
        .post(request_url)
        .json(safe_transaction_estimation_request)
        .timeout(Duration::from_millis(default_request_timeout()))
        .send()
        .await?;

    Ok(serde_json::from_str::<BackendSafeTransactionEstimation>(
        &estimation_response.text().await?,
    )?
    .safe_tx_gas)
}

async fn fetch_latest_nonce(context: &Context<'_>, request_url: String) -> ApiResult<u64> {
    let latest_multisig_tx_response = context
        .client()
        .get(request_url)
        .timeout(Duration::from_millis(default_request_timeout()))
        .send()
        .await?;

    Ok(serde_json::from_str::<Page<BackendMultisigTransaction>>(
        &latest_multisig_tx_response.text().await?,
    )?
    .results
    .first()
    .unwrap()
    .nonce)
}
