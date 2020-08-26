use crate::utils::context::Context;
use crate::models::backend::webhooks::{Payload, PayloadDetails};
use anyhow::Result;
use crate::utils::cache::Cache;

pub fn invalidate_caches(context: &Context, payload: &Payload) -> Result<()> {
    let cache = context.cache();
    invalidate(payload, cache);
    Ok(())
}

pub fn invalidate(payload: &Payload, cache: &impl Cache) {
    cache.invalidate_pattern(&format!("*{}*", &payload.address));
    payload.details.as_ref().map(|d| {
        match d {
            PayloadDetails::NewConfirmation(data) => {
                cache.invalidate_pattern(&format!("*{}*", data.safe_tx_hash));
            }
            PayloadDetails::ExecutedMultisigTransaction(data) => {
                cache.invalidate_pattern(&format!("*{}*", data.safe_tx_hash));
            }
            PayloadDetails::PendingMultisigTransaction(data) => {
                cache.invalidate_pattern(&format!("*{}*", data.safe_tx_hash));
            }
            _ => {}
        }
    });
}
