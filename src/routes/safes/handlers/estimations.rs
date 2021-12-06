use std::sync::Arc;

use crate::common::models::backend::transactions::{
    MultisigTransaction as BackendMultisigTransaction,
    SafeTransactionEstimation as BackendSafeTransactionEstimation,
};
use crate::common::models::page::Page;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::safes::models::{SafeTransactionEstimation, SafeTransactionEstimationRequest};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::{HttpClient, Request};

pub async fn estimate_safe_tx_gas(
    context: &RequestContext,
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
    let current_nonce = info_provider.safe_info(safe_address).await?.nonce;

    let latest_nonce = fetch_latest_nonce(context.http_client(), latest_multisig_tx_url).await?;
    let safe_tx_gas = fetch_estimation(
        context.http_client(),
        estimation_url,
        safe_transaction_estimation_request,
    )
    .await?;

    Ok(SafeTransactionEstimation {
        current_nonce,
        latest_nonce,
        safe_tx_gas,
    })
}

async fn fetch_estimation(
    client: Arc<dyn HttpClient>,
    request_url: String,
    safe_transaction_estimation_request: &SafeTransactionEstimationRequest,
) -> ApiResult<String> {
    let request = {
        let mut request = Request::new(request_url);
        request.body(Some(serde_json::to_string(
            safe_transaction_estimation_request,
        )?));
        request
    };
    let estimation_response = client.post(request).await?;

    Ok(
        serde_json::from_str::<BackendSafeTransactionEstimation>(&estimation_response.body)?
            .safe_tx_gas,
    )
}

async fn fetch_latest_nonce(client: Arc<dyn HttpClient>, request_url: String) -> ApiResult<u64> {
    let request = Request::new(request_url);
    let latest_multisig_tx_response = client.get(request).await?;
    let nonce = serde_json::from_str::<Page<BackendMultisigTransaction>>(
        &latest_multisig_tx_response.body,
    )?
    .results
    .first()
    .map(|it| it.nonce)
    .unwrap_or(0);

    Ok(nonce)
}
