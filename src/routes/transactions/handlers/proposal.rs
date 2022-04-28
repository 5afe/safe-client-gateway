use crate::cache::cache_operations::{Invalidate, InvalidationPattern, InvalidationScope};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::transactions::models::requests::MultisigTransactionRequest;
use crate::utils::context::{RequestContext, UNSPECIFIED_CHAIN};
use crate::utils::errors::ApiResult;
use crate::utils::http_client::Request;
use serde_json::json;

// The transaction details are returned at the route level. The same as for transaction submission
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

    let client = context.http_client();
    let request = {
        let mut request = Request::new(url);
        request.body(Some(json!({ "signature": signature }).to_string()));
        request
    };

    client.post(request).await?;
    Invalidate::new(
        InvalidationPattern::Any(InvalidationScope::Both, String::from(safe_tx_hash)),
        context.cache(&chain_id),
    )
    .execute()
    .await;

    Ok(())
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
    let request = {
        let mut request = Request::new(url);
        request.body(Some(serde_json::to_string(&transaction_request)?));
        request
    };
    client.post(request).await?;

    Invalidate::new(
        InvalidationPattern::Any(InvalidationScope::Both, String::from(safe_address)),
        context.cache(UNSPECIFIED_CHAIN),
    )
    .execute()
    .await;
    Invalidate::new(
        InvalidationPattern::Any(
            InvalidationScope::Both,
            String::from(&transaction_request.safe_tx_hash),
        ),
        context.cache(&chain_id),
    )
    .execute()
    .await;
    Ok(())
}
