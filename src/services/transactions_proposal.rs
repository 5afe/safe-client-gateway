use crate::config::base_transaction_service_url;
use crate::models::service::transactions::requests::SendEthRequest;
use crate::utils::context::Context;
use crate::utils::errors::{ApiError, ApiResult};

pub fn send_eth(
    context: &Context,
    safe_address: &str,
    send_eth_request: &SendEthRequest,
) -> ApiResult<()> {
    let url = format!(
        "{}/v1/safes/{}/transactions/",
        base_transaction_service_url(),
        &safe_address
    );
    let core_service_request = send_eth_request.to_multisig_transaction_request();

    let response = context
        .client()
        .post(&url)
        .json(&core_service_request)
        .send()?;

    if response.status().is_success() {
        Ok(())
    } else {
        Err(ApiError::from_http_response(
            response,
            String::from("Unexpected tx confirmation error"),
        ))
    }
}
