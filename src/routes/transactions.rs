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
use rocket::serde::json::Error;
use rocket::serde::json::Json;

/**
 * `/v1/chains/<chain_id>/transactions/<transaction_id>` <br />
 * Returns [TransactionDetails](crate::models::service::transactions::details::TransactionDetails)
 *
 * # Transaction Details
 *
 * The transaction details endpoint provides additional information for a transaction, in much more detail than what the transaction summary endpoint does. It returns a single object [crate::models::service::transactions::details::TransactionDetails]
 *
 * ## Path
 *
 * `GET /v1/chains/<chain_id>/transactions/<transaction_id>`
 *
 * `<transaction_id>` can be either an `id` returned by the transaction summary list endpoint or a `safe_tx_hash` from the Safe Transaction API.
 *
 * ## Query paramets
 *
 * There aren't any query parameters that can be passed to this endpoint.
 */
#[get("/v1/chains/<chain_id>/transactions/<details_id>")]
pub async fn get_transactions(
    context: Context<'_>,
    chain_id: String,
    details_id: String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .resp_generator(|| {
            transactions_details::get_transactions_details(&context, &chain_id, &details_id)
        })
        .execute(context.cache())
        .await
}

/**
 * `/v1/chains/<chain_id>/transactions/<safe_tx_hash>/confirmations` <br />
 * Returns [TransactionDetails](crate::models::service::transactions::details::TransactionDetails)
 *
 * # Transaction Confirmation
 *
 * This endpoint provides a way for submitting confirmations for clients making use of the `safe_tx_hash` as part of the path, and the very same `safe_tx_hash` signed by an owner corresponding to the safe from which the transaction is being sent.
 *
 * If the confirmation is submitted successfully to the core services, then the local cache for that specific transaction is invalidated and the updated transaction details with the confirmation are returned in the request.
 *
 * ## Path
 *
 * `POST /v1/chains/<chain_id>/transactions/<safe_tx_hash>/confirmations`
 *
 * The expected [crate::models::service::transactions::requests::ConfirmationRequest] body for this request, as well as the returned [crate::models::service::transactions::details::TransactionDetails]
 *
 * ## Query parameters
 *
 * No query parameters available for this endpoint.
 */
#[post(
    "/v1/chains/<chain_id>/transactions/<safe_tx_hash>/confirmations",
    format = "application/json",
    data = "<tx_confirmation_request>"
)]
pub async fn post_confirmation<'e>(
    context: Context<'_>,
    chain_id: String,
    safe_tx_hash: String,
    tx_confirmation_request: Result<Json<ConfirmationRequest>, Error<'e>>,
) -> ApiResult<content::Json<String>> {
    transactions_proposal::submit_confirmation(
        &context,
        &chain_id,
        &safe_tx_hash,
        &tx_confirmation_request?.0.signed_safe_tx_hash,
    )
    .await?;

    CacheResponse::new(context.uri())
        .resp_generator(|| {
            transactions_details::get_transactions_details(&context, &chain_id, &safe_tx_hash)
        })
        .execute(context.cache())
        .await
}

/**
 * `/v1/chains/<chain_id>/safes/<safe_address>/transactions/history?<cursor>&<timezone_offset>&<trusted>` <br />
 * Returns a [Page](crate::models::commons::Page) of [TransactionListItem](crate::models::service::transactions::summary::TransactionListItem)
 *
 * # Transactions History
 *
 * This endpoint returns all the transactions that have been executed for a given safe. Cancelled
 * transactions will not be shown in this endpoint. Therefore, there is no concept of conflicting `nonces`
 * for this endpoint, as there could potentially be for queued transactions.
 *
 * This endpoint does not return any `TransactionListItem::Label` nor `TransactionListItem::ConflictHeader`
 * as of the writing of this iteration of this document.
 *
 * Transaction are aggregated by day and for each day there is a `TransactionListItem::DateLabel` added.
 * The timestamp returned corresponds to the **date** only, **time** fields should be therefore ignored.
 * The dates are returned in UTC, in a later iteration this will be offset by the `timezone_offset`
 * sent by the clients in the query parameter.
 *
 * `TransactionListItem::Transaction` is returned with the same data layout as in the `/transactions/queued` endpoint.
 *
 * The structure of the `transaction` object corresponds to that of a [crate::models::service::transactions::summary::TransactionSummary]
 *
 * ## Path
 *
 * `GET /v1/chains/<chain_id>/safes/<safe_address>/transactions/history?<cursor>&<timezone_offset>&<trusted>`
 *
 * ## Query parameters
 *
 * - `<safe_address>` should be the checksummed address of the safe to be observed.
 * - `<cursor>` is the desired page of data to be loaded. Values for this parameter can be either `Page.next` or `Page.previous`. **WARNING:** Don't fiddle with the values of these 2 fields.
 * - `<timezone_offset>`: Currently ignored by the gateway.
 * - `<trusted>`: forwarded directly to the core services. Only for debugging purposes clients **should not** send it (unless they know what they are doing).
 */
#[get("/v1/chains/<chain_id>/safes/<safe_address>/transactions/history?<cursor>&<timezone_offset>")]
pub async fn get_transactions_history(
    context: Context<'_>,
    chain_id: String,
    safe_address: String,
    cursor: Option<String>,
    timezone_offset: Option<String>,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .resp_generator(|| {
            transactions_history::get_history_transactions(
                &context,
                &chain_id,
                &safe_address,
                &cursor,
                &timezone_offset,
            )
        })
        .execute(context.cache())
        .await
}

/**
 * `/v1/chains/<chain_id>/safes/<safe_address>/transactions/queued?<cursor>&<timezone_offset>&<trusted>` <br />
 * Returns a [Page](crate::models::commons::Page) of  [TransactionListItem](crate::models::service::transactions::summary::TransactionListItem)
 *
 * # Transactions Queued
 *
 * This endpoint returns all the transactions that are still awaiting execution for a given safe. The list will contain a `Next` marker if there is a transaction matching the nonce of the safe, which means, that it will be the next transaction to be executed, provided there aren't any other transactions proposed utilizing the same nonce. If that were, the case a `ConflictHeader` will be introduced for which the `nonce` field will hold the conflicting value.
 *
 * Additionally to the `Next` marker, there is also `Queued`. Under this marker, transactions that have a nonce greater than that of the safe are listed. Analogously to the `Next` section of the list, a `ConflictHeader` will be introduced for any group of transactions sharing the same `nonce`.
 *
 * The structure of the transaction object corresponds to that of a [crate::models::service::transactions::summary::TransactionSummary]
 *
 * A `TransactionListItem` can be either a `Label` (containing either `Next` or `Queued`), `ConflictHeader` (with the conflicting `nonce`) and a `Transaction`, for which there is a `TransactionSummary` and a `ConflictType` associated. The conflict type can have `HasNext` or `End` value. These values signal to which extent a group of conflicting transactions spans, ending as soon as a `Transaction` type item contains a `ConflictType::End`.
 *
 * ## Path
 *
 * `GET /v1/chains/<chain_id>/safes/<safe_address>/transactions/queued?<cursor>&<timezone_offset>&<trusted>`
 *
 * The response is a list of [crate::models::service::transactions::summary::TransactionListItem], which is a polymorphic struct. Details follow in the models sections.
 *
 * ## Query parameters
 *
 * - `<safe_address>` should be the checksummed address of the safe to be observed.
 * - `<cursor>` is the desired page of data to be loaded. Values for this parameter can be either `Page.next` or `Page.previous`. **WARNING:** Don't fiddle with the values of these 2 fields.
 * - `<timezone_offset>`: Currently ignored by the gateway.
 * - `<trusted>`: forwarded directly to the core services. Only for debugging purposes clients **should not** send it (unless they know what they are doing).
 */
#[get("/v1/chains/<chain_id>/safes/<safe_address>/transactions/queued?<cursor>&<timezone_offset>&<trusted>")]
pub async fn get_transactions_queued(
    context: Context<'_>,
    chain_id: String,
    safe_address: String,
    cursor: Option<String>,
    timezone_offset: Option<String>,
    trusted: Option<bool>,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .resp_generator(|| {
            transactions_queued::get_queued_transactions(
                &context,
                &chain_id,
                &safe_address,
                &cursor,
                &timezone_offset,
                &trusted,
            )
        })
        .execute(context.cache())
        .await
}

/**
 * `/v1/chains/<chain_id>/transactions/<safe_address>/propose` <br />
 * No return value
 *
 * # Transaction Proposal
 *
 * This endpoint provides a way for submitting transactions of any kind in the format expected by the core services.
 * See the example `json` to see how to submit a cancellation transaction (you would need to supply a `nonce`, `signature` and `contractTransactionHash` appropriate to the transaction you are submitting)
 *
 * ## Path
 *
 * `POST /v1/chains/<chain_id>/transactions/<safe_address>/propose`
 *
 * The expected [crate::models::service::transactions::requests::MultisigTransactionRequest] body for this request, can be found in the sections [models](https://github.com/gnosis/safe-client-gateway/wiki/transactions_confirmation#models)
 *
 * ## Query parameters
 *
 * No query parameters available for this endpoint.
 */
#[post(
    "/v1/chains/<chain_id>/transactions/<safe_address>/propose",
    format = "application/json",
    data = "<multisig_transaction_request>"
)]
pub async fn post_transaction<'e>(
    context: Context<'_>,
    chain_id: String,
    safe_address: String,
    multisig_transaction_request: Result<Json<MultisigTransactionRequest>, Error<'e>>,
) -> ApiResult<()> {
    transactions_proposal::propose_transaction(
        &context,
        &chain_id,
        &safe_address,
        &multisig_transaction_request?.0,
    )
    .await
}
