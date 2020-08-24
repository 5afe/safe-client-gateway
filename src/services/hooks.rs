use crate::utils::context::Context;
use crate::models::backend::webhooks::{Payload, PayloadDetails};
use anyhow::Result;

pub fn invalidate_caches(context: &Context, payload: &Payload) -> Result<()> {
    let cache = context.cache();
    cache.invalidate_pattern(&format!("*{}*", &payload.address));
    payload.details.map(|d| {
        match d {
            PayloadDetails::NewConfirmation(data) => {
                cache.invalidate_pattern(&format!("*{}*", data.safe_tx_hash));
            },
            PayloadDetails::ExecutedMultisigTransaction(data) => {
                cache.invalidate_pattern(&format!("*{}*", data.safe_tx_hash));
            },
            PayloadDetails::PendingMultisigTransaction(data) => {
                cache.invalidate_pattern(&format!("*{}*", data.safe_tx_hash));
            },
            _ => {}
        }
    });
    Ok(())
}
