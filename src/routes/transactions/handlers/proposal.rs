use crate::cache::cache_operations::{Invalidate, InvalidationPattern, InvalidationScope};
use crate::config::default_request_timeout;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::transactions::models::requests::MultisigTransactionRequest;
use crate::utils::context::RequestContext;
use crate::utils::errors::{ApiError, ApiResult};
use std::collections::HashMap;
use std::time::Duration;

pub async fn submit_confirmation(
    context: &RequestContext,
    chain_id: &str,
    safe_tx_hash: &str,
    signature: &str,
) -> ApiResult<()> {
    let info_provider = DefaultInfoProvider::new(chain_id, context);
    let url = core_uri!(
        info_provider,
        "/v1/multisig-transactions/{}/confirmations/",
        &safe_tx_hash
    )?;
    let mut json = HashMap::new();
    json.insert("signature", signature);

    let client = context.http_client();

    let response = client
        .post(&url)
        .json(&json)
        .timeout(Duration::from_millis(default_request_timeout()))
        .send()
        .await?;

    if response.status().is_success() {
        Invalidate::new(
            InvalidationPattern::Any(InvalidationScope::Both, String::from(safe_tx_hash)),
            context.cache(),
        )
        .execute();
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
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    transaction_request: &MultisigTransactionRequest,
) -> ApiResult<()> {
    let info_provider = DefaultInfoProvider::new(chain_id, context);
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/multisig-transactions/",
        &safe_address
    )?;

    let client = context.http_client();

    let response = client
        .post(&url)
        .json(&transaction_request)
        .timeout(Duration::from_millis(default_request_timeout()))
        .send()
        .await?;

    if response.status().is_success() {
        Invalidate::new(
            InvalidationPattern::Any(InvalidationScope::Both, String::from(safe_address)),
            context.cache(),
        )
        .execute();
        Invalidate::new(
            InvalidationPattern::Any(
                InvalidationScope::Both,
                String::from(&transaction_request.safe_tx_hash),
            ),
            context.cache(),
        )
        .execute();
        Ok(())
    } else {
        Err(ApiError::from_http_response(
            response,
            String::from("Unexpected multisig tx proposal error"),
        )
        .await)
    }
}
