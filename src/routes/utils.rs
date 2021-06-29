use crate::models::service::utils::{DataDecoderRequest, SafeTransactionEstimationRequest};
use crate::services::utils;
use crate::services::utils::request_data_decoded;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket::serde::json::Error;
use rocket::serde::json::Json;

/**
 * `/<chain_id>/data-decoder` <br/>
 * Returns [DataDecoded](crate::models::commons::DataDecoded)
 *
 * # Data Decoder
 *
 * This endpoint requires the client to send in the body of the request a hexadecimal `String` containing the `data` field of a transaction for decoding
 *
 * The result is of the type [DataDecoded](crate::models::commons::DataDecoded)
 *
 * ## Path
 *
 * - `/<chain_id>/data-decoder`
 *
 * ## Examples
 *
 * Example request body:
 *
 * ```json
 * {
 *   "data": "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000"
 * }
 * ```
 * This results in:
 *
 * ```json
 * {
 *   "method": "approve",
 *   "parameters": [
 *    {
 *       "name": "spender",
 *       "type": "address",
 *       "value": "0xae9844F89D98c150F5e61bfC676D68b492155990"
 *     },
 *     {
 *       "name": "value",
 *       "type": "uint256",
 *       "value": "500000000000000"
 *     }
 *   ]
 * }
 * ```
 *
 */
#[post(
    "/<chain_id>/data-decoder",
    format = "application/json",
    data = "<data_decoder_request>"
)]
pub async fn post_data_decoder<'e>(
    context: Context<'_>,
    chain_id: String,
    data_decoder_request: Result<Json<DataDecoderRequest>, Error<'e>>,
) -> ApiResult<content::Json<String>> {
    Ok(content::Json(serde_json::to_string(
        &request_data_decoded(&context, &chain_id, &data_decoder_request?.0).await?,
    )?))
}

/**
 * `/<chain_id>/safes/<safe_address>/multisig-transactions/estimations` <br />
 * Returns [SafeTransactionEstimation](crate::models::service::utils::SafeTransactionEstimation)
 *
 * # Safe Gas Estimation
 *
 * This endpoint provides a `safeTxGas` according to the transaction passed as part of the request body
 *
 * ## Path
 *
 * - `/<chain_id>/safes/<safe_address>/multisig-transactions/estimations
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
 *   "safeTxGas": "43585"
 * }
 * ```
 *
 */
#[post(
    "/<chain_id>/safes/<safe_address>/multisig-transactions/estimations",
    format = "application/json",
    data = "<safe_transaction_estimation_request>"
)]
pub async fn post_safe_gas_estimation<'e>(
    context: Context<'_>,
    chain_id: String,
    safe_address: String,
    safe_transaction_estimation_request: Result<Json<SafeTransactionEstimationRequest>, Error<'e>>,
) -> ApiResult<content::Json<String>> {
    Ok(content::Json(serde_json::to_string(
        &utils::estimate_safe_tx_gas(
            &context,
            &chain_id,
            &safe_address,
            &safe_transaction_estimation_request?.0,
        )
        .await?,
    )?))
}
