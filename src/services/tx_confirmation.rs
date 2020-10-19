use crate::utils::context::Context;
use crate::models::service::transactions::requests::ConfirmationRequest;
use crate::models::backend::requests::ConfirmationRequest as BackendConfirmationRequest;
use crate::utils::errors::{ApiResult, ApiError, ErrorDetails};
use crate::services::transactions_details::{parse_id, get_multisig_transaction_details};
use crate::models::service::transactions::details::TransactionDetails;
use crate::models::service::transactions::TransactionIdParts;
use crate::utils::cache::Cache;
use crate::config::base_transaction_service_url;
use anyhow::Result;
use rocket::logger::error;

pub fn submit_confirmation(context: &Context, safe_tx_hash: &str) -> ApiResult<()> {
    context.cache().invalidate_pattern(&format!("*{}*", &safe_tx_hash));

    let url = format!("{}/v1/multisig-transactions/{}/confirmations/", base_transaction_service_url(), &safe_tx_hash);
    let result = context.client().post(&url).send()?;

    if result.status().is_success() { Ok(()) } else {
        Err(
            ApiError::from_backend_error(
                result.status().as_u16(),
                result.text().unwrap_or(String::from("Unexpected tx confirmation error")).as_str()))
    }
}
