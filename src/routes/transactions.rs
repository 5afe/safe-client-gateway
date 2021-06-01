use crate::cache::cache_operations::CacheResponse;
use crate::models::service::transactions::requests::{
    ConfirmationRequest, MultisigTransactionRequest,
};
use crate::services::{
    transactions_details, transactions_history, transactions_proposal, transactions_queued,
};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonError;

///
/// # Transaction Details
///
/// The transaction details endpoint provides additional information for a transaction, in much more detail than what the transaction summary endpoint does. It returns a single object that can be visualized in the [models](https://github.com/gnosis/safe-client-gateway/wiki/transaction_details#models) section of this article.
///
/// ## Path
///
/// `/v1/transactions/<transaction_id>`
///
/// `<transaction_id>` can be either an `id` returned by the transaction summary list endpoint or a `safe_tx_hash` from the Safe Transaction API.
///
/// ## Query paramets
///
/// There aren't any query parameters that can be passed to this endpoint. Returns [crate::models::service::transactions::details::TransactionDetails]
#[get("/v1/transactions/<details_id>")]
pub async fn details(context: Context<'_>, details_id: String) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .resp_generator(|| transactions_details::get_transactions_details(&context, &details_id))
        .execute(context.cache())
        .await
}

#[post(
    "/v1/transactions/<safe_tx_hash>/confirmations",
    format = "application/json",
    data = "<tx_confirmation_request>"
)]
pub async fn submit_confirmation<'e>(
    context: Context<'_>,
    safe_tx_hash: String,
    tx_confirmation_request: Result<Json<ConfirmationRequest>, JsonError<'e>>,
) -> ApiResult<content::Json<String>> {
    transactions_proposal::submit_confirmation(
        &context,
        &safe_tx_hash,
        &tx_confirmation_request?.0.signed_safe_tx_hash,
    )
    .await?;

    CacheResponse::new(context.uri())
        .resp_generator(|| transactions_details::get_transactions_details(&context, &safe_tx_hash))
        .execute(context.cache())
        .await
}

#[get("/v1/safes/<safe_address>/transactions/history?<page_url>&<timezone_offset>")]
pub async fn history_transactions(
    context: Context<'_>,
    safe_address: String,
    page_url: Option<String>,
    timezone_offset: Option<String>,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .resp_generator(|| {
            transactions_history::get_history_transactions(
                &context,
                &safe_address,
                &page_url,
                &timezone_offset,
            )
        })
        .execute(context.cache())
        .await
}

#[get("/v1/safes/<safe_address>/transactions/queued?<page_url>&<timezone_offset>&<trusted>")]
pub async fn queued_transactions(
    context: Context<'_>,
    safe_address: String,
    page_url: Option<String>,
    timezone_offset: Option<String>,
    trusted: Option<bool>,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .resp_generator(|| {
            transactions_queued::get_queued_transactions(
                &context,
                &safe_address,
                &page_url,
                &timezone_offset,
                &trusted,
            )
        })
        .execute(context.cache())
        .await
}

#[post(
    "/v1/transactions/<safe_address>/propose",
    format = "application/json",
    data = "<multisig_transaction_request>"
)]
pub async fn propose_transaction<'e>(
    context: Context<'_>,
    safe_address: String,
    multisig_transaction_request: Result<Json<MultisigTransactionRequest>, JsonError<'e>>,
) -> ApiResult<()> {
    transactions_proposal::propose_transaction(
        &context,
        &safe_address,
        &multisig_transaction_request?.0,
    )
    .await
}
