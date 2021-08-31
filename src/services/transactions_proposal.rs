use crate::cache::cache_operations::{Invalidate, InvalidationPattern, InvalidationScope};
use crate::models::service::transactions::requests::MultisigTransactionRequest;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::{ApiError, ApiResult};
use std::collections::HashMap;

pub async fn submit_confirmation(
    context: &Context<'_>,
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

    let response = context.client().post(&url).json(&json).send().await?;

    if response.status().is_success() {
        Invalidate::new(InvalidationPattern::Any(
            InvalidationScope::Both,
            String::from(safe_tx_hash),
        ))
        .execute(context.cache());
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
    let response = context
        .client()
        .post(&url)
        .json(&transaction_request)
        .send()
        .await?;

    if response.status().is_success() {
        Invalidate::new(InvalidationPattern::Any(
            InvalidationScope::Both,
            String::from(safe_address),
        ))
        .execute(context.cache());
        Invalidate::new(InvalidationPattern::Any(
            InvalidationScope::Both,
            String::from(&transaction_request.safe_tx_gas),
        ))
        .execute(context.cache());
        Ok(())
    } else {
        Err(ApiError::from_http_response(
            response,
            String::from("Unexpected multisig tx proposal error"),
        )
        .await)
    }
}
