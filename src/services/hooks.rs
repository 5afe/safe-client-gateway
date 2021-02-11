use crate::models::backend::webhooks::{Payload, PayloadDetails};
use crate::utils::cache::Cache;
use crate::utils::errors::ApiResult;

pub fn invalidate_caches(cache: &impl Cache, payload: &Payload) -> ApiResult<()> {
    cache.invalidate_caches(&payload.address);
    payload.details.as_ref().map(|d| match d {
        PayloadDetails::NewConfirmation(data) => {
            cache.invalidate_caches(&data.safe_tx_hash);
        }
        PayloadDetails::ExecutedMultisigTransaction(data) => {
            cache.invalidate_caches(&data.safe_tx_hash);
        }
        PayloadDetails::PendingMultisigTransaction(data) => {
            cache.invalidate_caches(&data.safe_tx_hash);
        }
        _ => {}
    });
    Ok(())
}
