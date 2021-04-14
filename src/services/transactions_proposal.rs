use crate::cache::Cache;
use crate::config::base_transaction_service_url;
use crate::models::service::transactions::requests::MultisigTransactionRequest;
use crate::utils::context::Context;
use crate::utils::errors::{ApiError, ApiResult};
use std::collections::HashMap;

pub async fn submit_confirmation(
    context: &Context<'_>,
    safe_tx_hash: &str,
    signature: &str,
) -> ApiResult<()> {
    let url = format!(
        "{}/v1/multisig-transactions/{}/confirmations/",
        base_transaction_service_url(),
        &safe_tx_hash
    );
    let mut json = HashMap::new();
    json.insert("signature", signature);

    let response = context.client().post(&url).json(&json).send().await?;

    if response.status().is_success() {
        context
            .cache()
            .invalidate_pattern(&format!("*{}*", &safe_tx_hash));
        Ok(())
    } else {
        Err(ApiError::from_http_response(
            response,
            String::from("Unexpected tx confirmation error"),
        )
        .await)
    }
}

pub async fn propose_transaction(
    context: &Context<'_>,
    safe_address: &str,
    transaction_request: &MultisigTransactionRequest,
) -> ApiResult<()> {
    let url = format!(
        "{}/v1/safes/{}/multisig-transactions/",
        base_transaction_service_url(),
        &safe_address
    );
    let response = context
        .client()
        .post(&url)
        .json(&transaction_request)
        .send()
        .await?;

    if response.status().is_success() {
        context
            .cache()
            .invalidate_pattern(&format!("*{}*", &safe_address));
        Ok(())
    } else {
        Err(ApiError::from_http_response(
            response,
            String::from("Unexpected multisig tx proposal error"),
        )
        .await)
    }
}
