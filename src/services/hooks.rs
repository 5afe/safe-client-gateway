use crate::cache::cache_operations::{Invalidate, InvalidationPattern, Something};
use crate::cache::Cache;
use crate::models::backend::webhooks::{Payload, PayloadDetails};
use crate::utils::errors::ApiResult;

pub fn invalidate_caches(cache: &impl Cache, payload: &Payload) -> ApiResult<()> {
    Invalidate::new(InvalidationPattern::Any(
        // safe variant
        payload.address.to_owned(),
        Something::Both,
    ))
    .execute(cache);
    payload.details.as_ref().map(|d| match d {
        PayloadDetails::NewConfirmation(data) => {
            Invalidate::new(InvalidationPattern::Any(
                //TODO investigate impact of using Transaction variant
                String::from(&data.safe_tx_hash),
                Something::Both,
            ))
            .execute(cache);
        }
        PayloadDetails::ExecutedMultisigTransaction(data) => {
            Invalidate::new(InvalidationPattern::Any(
                //TODO investigate impact of using Transaction variant
                String::from(&data.safe_tx_hash),
                Something::Both,
            ))
            .execute(cache);
        }
        PayloadDetails::PendingMultisigTransaction(data) => {
            Invalidate::new(InvalidationPattern::Any(
                //TODO investigate impact of using Transaction variant
                String::from(&data.safe_tx_hash),
                Something::Both,
            ))
            .execute(cache);
        }
        _ => {}
    });
    Ok(())
}
