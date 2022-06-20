use crate::cache::cache_operations::CacheResponse;
use crate::cache::manager::ChainCache;
use crate::routes::contracts::handlers;
use crate::routes::contracts::models::DataDecoderRequest;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket::serde::json::{Error, Json};
use rocket_okapi::{openapi};
/// `/v1/chains/<chain_id>/data-decoder` <br/>
/// Returns [DataDecoded](crate::common::models::data_decoded::DataDecoded)
///
/// # Data Decoder
///
/// This endpoint requires the client to send in the body of the request a hexadecimal `String` containing the `data` field of a transaction for decoding
///
/// The result is of the type [DataDecoded](crate::common::models::data_decoded::DataDecoded)
///
/// ## Path
///
/// - `/v1/chains/<chain_id>/data-decoder`
///
/// ## Examples
///
/// Example request body:
///
/// ```json
/// {
///   "data": "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000"
/// }
/// ```
/// This results in:
///
/// ```json
/// {
///   "method": "approve",
///   "parameters": [
///    {
///       "name": "spender",
///       "type": "address",
///       "value": "0xae9844F89D98c150F5e61bfC676D68b492155990"
///     },
///     {
///       "name": "value",
///       "type": "uint256",
///       "value": "500000000000000"
///     }
///   ]
/// }
/// ```
//TODO add to swagger 
#[post(
    "/v1/chains/<chain_id>/data-decoder",
    format = "application/json",
    data = "<data_decoder_request>"
)]
pub async fn post_data_decoder<'e>(
    context: RequestContext,
    chain_id: String,
    data_decoder_request: Result<Json<DataDecoderRequest>, Error<'e>>,
) -> ApiResult<content::RawJson<String>> {
    Ok(content::RawJson(serde_json::to_string(
        &handlers::request_data_decoded(&context, &chain_id, &data_decoder_request?.0).await?,
    )?))
}

/// `/v1/chains/<chain_id>/contracts/<address>` <br/>
/// Returns [ContractInfo](crate::providers::address_info::ContractInfo)
///
/// # Contract Info
///
/// This endpoint is chain dependant and returns the details of a Contract such as: name, logoUri, ABI, among others
///
/// The result is of the type [ContractInfo](crate::providers::address_info::ContractInfo)
///
/// ## Path
///
/// - `GET /v1/chains/<chain_id>/contract/<contract_address>`
#[openapi(tag = "Contracts")]
#[get("/v1/chains/<chain_id>/contracts/<contract_address>")]
pub async fn get_contract(
    context: RequestContext,
    chain_id: String,
    contract_address: String,
) -> ApiResult<content::RawJson<String>> {
    CacheResponse::new(&context, ChainCache::from(chain_id.as_str()))
        .resp_generator(|| handlers::get_contract(&context, &chain_id, &contract_address))
        .execute()
        .await
}
