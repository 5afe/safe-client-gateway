use crate::routes::contracts::handlers::request_data_decoded;
use crate::routes::contracts::models::DataDecoderRequest;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket::serde::json::Error;
use rocket::serde::json::Json;

/**
 * `/v1/chains/<chain_id>/data-decoder` <br/>
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
 * - `/v1/chains/<chain_id>/data-decoder`
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
    "/v1/chains/<chain_id>/data-decoder",
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
