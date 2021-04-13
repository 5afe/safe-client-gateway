use crate::cache::cache::Cache;
use crate::cache::cache_operations::{Invalidate, InvalidationPattern};
use crate::models::backend::webhooks::{Payload, PayloadDetails};
use crate::utils::errors::ApiResult;

pub fn invalidate_caches(cache: &impl Cache, payload: &Payload) -> ApiResult<()> {
    Invalidate::new()
        .pattern(InvalidationPattern::RequestsResponses(
            payload.address.to_owned(),
        ))
        .execute(cache);
    payload.details.as_ref().map(|d| match d {
        PayloadDetails::NewConfirmation(data) => {
            Invalidate::new()
                .pattern(InvalidationPattern::RequestsResponses(String::from(
                    &data.safe_tx_hash,
                )))
                .execute(cache);
        }
        PayloadDetails::ExecutedMultisigTransaction(data) => {
            Invalidate::new()
                .pattern(InvalidationPattern::RequestsResponses(String::from(
                    &data.safe_tx_hash,
                )))
                .execute(cache);
        }
        PayloadDetails::PendingMultisigTransaction(data) => {
            Invalidate::new()
                .pattern(InvalidationPattern::RequestsResponses(String::from(
                    &data.safe_tx_hash,
                )))
                .execute(cache);
        }
        _ => {}
    });
    Ok(())
}
