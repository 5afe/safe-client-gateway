use crate::common::models::backend::transactions::{
    MultisigTransaction as BackendMultisigTransaction,
    SafeTransactionEstimation as BackendSafeTransactionEstimation,
};
use crate::common::models::page::Page;
use crate::config::default_request_timeout;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::safes::models::{SafeTransactionEstimation, SafeTransactionEstimationRequest};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use std::time::Duration;

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
        "/v1/safes/{}/multisig-transactions/?ordering=-nonce&trusted=true&limit=1",
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

    let nonce = serde_json::from_str::<Page<BackendMultisigTransaction>>(
        &latest_multisig_tx_response.text().await?,
    )?
    .results
    .first()
    .map(|it| it.nonce)
    .unwrap_or(0);

    Ok(nonce)
}
