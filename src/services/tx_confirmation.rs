use crate::utils::context::Context;
use crate::models::service::transactions::requests::ConfirmationRequest;
use crate::models::backend::requests::ConfirmationRequest as BackendConfirmationRequest;
use crate::utils::errors::{ApiResult, ApiError};
use crate::services::transactions_details::{parse_id, get_multisig_transaction_details};
use crate::models::service::transactions::details::TransactionDetails;
use crate::models::service::transactions::TransactionIdParts;
use crate::utils::cache::Cache;
use crate::config::base_transaction_service_url;

pub fn submit_confirmation(context: &Context, tx_id: &str, confirmation_request: &ConfirmationRequest) -> ApiResult<TransactionDetails> {
    if let Ok(TransactionIdParts::Multisig { safe_address, safe_tx_hash }) = parse_id(tx_id) {
        let transaction_details = get_multisig_transaction_details(context, &safe_tx_hash)?;
        let backend_confirmation_request = confirmation_request.build_confirmation_request(&safe_address, &safe_tx_hash, transaction_details)?;

        context.cache().invalidate_pattern(&format!("*{}*", &safe_tx_hash));

        let url = format!("{}/v1/safes/{}/multisig-transactions/", base_transaction_service_url(), &safe_address);
        context.client().post(&url)
            .body(serde_json::to_string::<BackendConfirmationRequest>(&backend_confirmation_request)?)
            .send()?;

        get_multisig_transaction_details(context, &safe_tx_hash)
    } else {
        Err(ApiError::new_from_message(String::from("Bad id for confirmation submission")))
    }
}
