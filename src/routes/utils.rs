use crate::models::service::utils::{
    DataDecoderRequest, SafeTransactionEstimation, SafeTransactionEstimationRequest,
};
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
) { //-> ApiResult<content::Json<String>> {
     // transactions_proposal::propose_transaction(
     //     &context,
     //     &safe_address,
     //     &multisig_transaction_request?.0,
     // )
     // .await
}
