use crate::utils::context::Context;
use crate::models::service::transactions::tx_requests::TxConfirmationRequest;
use crate::utils::errors::{ApiResult, ApiError};
use crate::services::transactions_details::{parse_id, get_multisig_transaction_details};
use crate::models::service::transactions::details::TransactionDetails;
use crate::models::service::transactions::TransactionIdParts;
use crate::utils::cache::Cache;

pub fn submit_confirmation(context: &Context, tx_id: &str, confirmation_request: &TxConfirmationRequest) -> ApiResult<TransactionDetails> {
    if let TransactionIdParts::Multisig(safe_tx_hash) = parse_id(tx_id)? {
        context.cache().invalidate_pattern(&format!("*{}*", &safe_tx_hash));
        // TODO submit the confirmation
        get_multisig_transaction_details(context, &safe_tx_hash)
    } else {
        Err(ApiError::new_from_message(String::from("Can't submit confirmation to non-MultiSig tx")))
    }
}