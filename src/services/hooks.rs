use crate::cache::cache_operations::{Invalidate, InvalidationPattern, InvalidationScope};
use crate::cache::Cache;
use crate::models::backend::webhooks::{Payload, PayloadDetails};
use crate::utils::errors::ApiResult;

pub fn invalidate_caches(cache: &impl Cache, payload: &Payload) -> ApiResult<()> {
    Invalidate::new(InvalidationPattern::Any(
        payload.address.to_owned(),
        InvalidationScope::Both,
    ))
    .execute(cache);
    payload.details.as_ref().map(|d| match d {
        PayloadDetails::NewConfirmation(data) => {
            Invalidate::new(InvalidationPattern::Any(
                String::from(&data.safe_tx_hash),
                InvalidationScope::Both,
            ))
            .execute(cache);
        }
        PayloadDetails::ExecutedMultisigTransaction(data) => {
            Invalidate::new(InvalidationPattern::Any(
                String::from(&data.safe_tx_hash),
                InvalidationScope::Both,
            ))
            .execute(cache);
        }
        PayloadDetails::PendingMultisigTransaction(data) => {
            Invalidate::new(InvalidationPattern::Any(
                String::from(&data.safe_tx_hash),
                InvalidationScope::Both,
            ))
            .execute(cache);
        }
        _ => {}
    });
    Ok(())
}
