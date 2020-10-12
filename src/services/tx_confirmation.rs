use crate::utils::context::Context;
use crate::models::service::transactions::tx_requests::TxConfirmationRequest;
use crate::utils::errors::ApiResult;

pub fn submit_confirmation(context: &Context, tx_id: &str, confirmation_request: &TxConfirmationRequest) -> ApiResult<()> {
    Ok(())
}