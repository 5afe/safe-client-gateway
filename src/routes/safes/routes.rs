use crate::cache::cache_operations::CacheResponse;
use crate::config::owners_for_safes_cache_duration;
use crate::routes::safes::handlers::estimations;
use crate::routes::safes::handlers::safes::{get_owners_for_safe, get_safe_info_ex};
use crate::routes::safes::models::SafeTransactionEstimationRequest;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket::serde::json::Error;
use rocket::serde::json::Json;

/**
 * `/v1/chains/<chain_id>/safes/<safe_address>` <br />
 * Returns [SafeState](crate::routes::safes::models::SafeState)
 */
#[get("/v1/chains/<chain_id>/safes/<safe_address>")]
pub async fn get_safe_info(
    context: RequestContext,
    chain_id: String,
    safe_address: String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(&context)
        .resp_generator(|| get_safe_info_ex(&context, &chain_id, &safe_address))
        .execute()
        .await
}

/**
 * `/v1/chains/<chain_id>/owners/<safe_address>/safes` <br/>
 * Returns [Vec] of [String]
 *
 * Returns a list of Safes for which the address is an owner
 */
#[get("/v1/chains/<chain_id>/owners/<owner_address>/safes")]
pub async fn get_owners(
    context: RequestContext,
    chain_id: String,
    owner_address: String,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(&context)
        .resp_generator(|| get_owners_for_safe(&context, &chain_id, &owner_address))
        .duration(owners_for_safes_cache_duration())
        .execute()
        .await
}

/**
 * DEPRECATED
 * `/v1/chains/<chain_id>/safes/<safe_address>/multisig-transactions/estimations` <br />
 * Returns [SafeTransactionEstimation](crate::routes::safes::models::SafeTransactionEstimation)
 *
 * # Safe Gas Estimation
 *
 * This endpoint provides a `safeTxGas` according to the transaction passed as part of the request body
 *
 * ## Path
 *
 * - `/v1/chains/<chain_id>/safes/<safe_address>/multisig-transactions/estimations
 *
 * ## Examples
 *
 * Example request body:
 *
 * ```json
 * {
 *   "to": "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02",
 *   "value": "0",
 *   "data": "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000",
 *   "operation": 0
 * }
 * ```
 *
 * This results (at the time of writing this documentation) in:
 *
 * ```json
 * {
 *   "latestNonce": 76,
 *   "safeTxGas": "63417"
 * }
 * ```
 *
 */
#[post(
    "/v1/chains/<chain_id>/safes/<safe_address>/multisig-transactions/estimations",
    format = "application/json",
    data = "<safe_transaction_estimation_request>"
)]
pub async fn post_safe_gas_estimation<'e>(
    context: RequestContext,
    chain_id: String,
    safe_address: String,
    safe_transaction_estimation_request: Result<Json<SafeTransactionEstimationRequest>, Error<'e>>,
) -> ApiResult<content::Json<String>> {
    Ok(content::Json(serde_json::to_string(
        &estimations::estimate_safe_tx_gas(
            &context,
            &chain_id,
            &safe_address,
            &safe_transaction_estimation_request?.0,
        )
        .await?,
    )?))
}

/**
 * `/v2/chains/<chain_id>/safes/<safe_address>/multisig-transactions/estimations` <br />
 * Returns [SafeTransactionEstimation](crate::routes::safes::models::SafeTransactionEstimation)
 *
 * # Safe Transaction Estimations
 *
 * This endpoint provides a `safeTxGas` according to the transaction passed as part of the request body,
 * the `currentNonce` indicating what the nonce of the Safe currently is,
 * and a `recommendedNonce` that should be used for the new transaction.
 *
 * ## Path
 *
 * - `/v2/chains/<chain_id>/safes/<safe_address>/multisig-transactions/estimations
 *
 * ## Examples
 *
 * Example request body:
 *
 * ```json
 * {
 *   "to": "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02",
 *   "value": "0",
 *   "data": "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000",
 *   "operation": 0
 * }
 * ```
 *
 * This results (at the time of writing this documentation) in:
 *
 * ```json
 * {
 *   "currentNonce": 7,
 *   "recommendedNonce": 76,
 *   "safeTxGas": "63417"
 * }
 * ```
 *
 */
#[post(
    "/v2/chains/<chain_id>/safes/<safe_address>/multisig-transactions/estimations",
    format = "application/json",
    data = "<safe_transaction_estimation_request>"
)]
pub async fn post_safe_gas_estimation_v2<'e>(
    context: RequestContext,
    chain_id: String,
    safe_address: String,
    safe_transaction_estimation_request: Result<Json<SafeTransactionEstimationRequest>, Error<'e>>,
) -> ApiResult<content::Json<String>> {
    Ok(content::Json(serde_json::to_string(
        &estimations::estimate_safe_tx_gas_v2(
            &context,
            &chain_id,
            &safe_address,
            &safe_transaction_estimation_request?.0,
        )
        .await?,
    )?))
}
