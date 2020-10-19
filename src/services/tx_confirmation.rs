use crate::config::base_transaction_service_url;
use crate::utils::cache::Cache;
use crate::utils::context::Context;
use crate::utils::errors::{ApiError, ApiResult};
use std::collections::HashMap;

pub fn submit_confirmation(
    context: &Context,
    safe_tx_hash: &str,
    signature: &str,
) -> ApiResult<()> {
    context
        .cache()
        .invalidate_pattern(&format!("*{}*", &safe_tx_hash));

    let url = format!(
        "{}/v1/multisig-transactions/{}/confirmations/",
        base_transaction_service_url(),
        &safe_tx_hash
    );
    let mut json = HashMap::new();
    json.insert("signature", signature);

    let result = context.client().post(&url).json(&json).send()?;

    if result.status().is_success() {
        Ok(())
    } else {
        Err(ApiError::from_backend_error(
            result.status().as_u16(),
            result
                .text()
                .unwrap_or(String::from("Unexpected tx confirmation error"))
                .as_str(),
        ))
    }
}
