use crate::cache::cache_operations::{Invalidate, InvalidationPattern};
use crate::cache::Cache;
use crate::models::backend::webhooks::{Payload, PayloadDetails};
use crate::utils::errors::ApiResult;

pub fn invalidate_caches(cache: &impl Cache, payload: &Payload) -> ApiResult<()> {
    Invalidate::new(InvalidationPattern::SafeAddress(payload.address.to_owned())).execute(cache);
    payload.details.as_ref().map(|d| match d {
        PayloadDetails::NewConfirmation(data) => {
            Invalidate::new(InvalidationPattern::SafeAddress(String::from(
                &data.safe_tx_hash,
            )))
            .execute(cache);
        }
        PayloadDetails::ExecutedMultisigTransaction(data) => {
            Invalidate::new(InvalidationPattern::SafeAddress(String::from(
                &data.safe_tx_hash,
            )))
            .execute(cache);
        }
        PayloadDetails::PendingMultisigTransaction(data) => {
            Invalidate::new(InvalidationPattern::SafeAddress(String::from(
                &data.safe_tx_hash,
            )))
            .execute(cache);
        }
        _ => {}
    });
    Ok(())
}
