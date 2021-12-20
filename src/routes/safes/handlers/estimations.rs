use std::sync::Arc;

use crate::common::models::backend::transactions::{
    MultisigTransaction as BackendMultisigTransaction,
    SafeTransactionEstimation as BackendSafeTransactionEstimation,
};
use crate::common::models::page::Page;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::safes::models::{
    SafeTransactionEstimation, SafeTransactionEstimationRequest, SafeTransactionEstimationV2,
};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::{HttpClient, Request};
use std::cmp::max;

async fn get_last_known_nonce_and_estimate(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    safe_transaction_estimation_request: &SafeTransactionEstimationRequest,
) -> ApiResult<(String, u64, Option<u64>)> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);

    let current_nonce = info_provider.safe_info(safe_address).await?.nonce;

    let latest_multisig_tx_url = core_uri!(
        info_provider,
        "/v1/safes/{}/multisig-transactions/?ordering=-nonce&trusted=true&limit=1",
        safe_address
    )?;
    let last_known_nonce =
        fetch_last_known_nonce(context.http_client(), latest_multisig_tx_url).await?;

    let estimation_url = core_uri!(
        info_provider,
        "/v1/safes/{}/multisig-transactions/estimations/",
        safe_address
    )?;
    let safe_tx_gas = fetch_estimation(
        context.http_client(),
        estimation_url,
        safe_transaction_estimation_request,
    )
    .await?;

    Ok((safe_tx_gas, current_nonce, last_known_nonce))
}

pub async fn estimate_safe_tx_gas(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    safe_transaction_estimation_request: &SafeTransactionEstimationRequest,
) -> ApiResult<SafeTransactionEstimation> {
    let (safe_tx_gas, current_nonce, last_known_nonce) = get_last_known_nonce_and_estimate(
        context,
        chain_id,
        safe_address,
        safe_transaction_estimation_request,
    )
    .await?;

    // If there is no transaction available on the tx service, we default to 0
    // Note: This is not really correct and therefore clients should use the recommended nonce returned
    let latest_nonce = last_known_nonce.unwrap_or(0);

    Ok(SafeTransactionEstimation {
        current_nonce,
        latest_nonce,
        safe_tx_gas,
    })
}

pub async fn estimate_safe_tx_gas_v2(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    safe_transaction_estimation_request: &SafeTransactionEstimationRequest,
) -> ApiResult<SafeTransactionEstimationV2> {
    let (safe_tx_gas, current_nonce, last_known_nonce) = get_last_known_nonce_and_estimate(
        context,
        chain_id,
        safe_address,
        safe_transaction_estimation_request,
    )
    .await?;

    // The next nonce recommended to use for a new transaction is the maximum of
    // - the nonce of the latest transaction from the tx service plus 1, or 0 if there is no such transaction
    // - the current Safe nonce
    let recommended_nonce = max(
        current_nonce,
        last_known_nonce.map(|it| it + 1).unwrap_or(0),
    );

    Ok(SafeTransactionEstimationV2 {
        current_nonce,
        recommended_nonce,
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

async fn fetch_last_known_nonce(
    client: Arc<dyn HttpClient>,
    request_url: String,
) -> ApiResult<Option<u64>> {
    let request = Request::new(request_url);
    let latest_multisig_tx_response = client.get(request).await?;
    let nonce = serde_json::from_str::<Page<BackendMultisigTransaction>>(
        &latest_multisig_tx_response.body,
    )?
    .results
    .first()
    .map(|it| it.nonce);

    Ok(nonce)
}
