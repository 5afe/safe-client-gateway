use crate::models::service::utils::{
    DataDecoderRequest, SafeTransactionEstimation, SafeTransactionEstimationRequest,
};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::serde::json::Error;
use rocket::serde::json::Json;

#[post(
    "/<chain_id>/data_decoder",
    format = "application/json",
    data = "<data_decoder_request>"
)]
pub async fn post_data_decoder<'e>(
    context: Context<'_>,
    chain_id: String,
    data_decoder_request: Result<Json<DataDecoderRequest>, Error<'e>>,
) -> ApiResult<()> {
    // transactions_proposal::propose_transaction(
    //     &context,
    //     &safe_address,
    //     &multisig_transaction_request?.0,
    // )
    // .await
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
) -> ApiResult<SafeTransactionEstimation> {
    // transactions_proposal::propose_transaction(
    //     &context,
    //     &safe_address,
    //     &multisig_transaction_request?.0,
    // )
    // .await
}
