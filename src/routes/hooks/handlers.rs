use crate::cache::cache_operations::{Invalidate, InvalidationPattern, InvalidationScope};
use crate::cache::Cache;
use crate::common::models::backend::hooks::{Payload, PayloadDetails};
use crate::utils::errors::ApiResult;
use std::sync::Arc;

pub async fn invalidate_caches(cache: Arc<dyn Cache>, payload: &Payload) -> ApiResult<()> {
    Invalidate::new(
        InvalidationPattern::Any(InvalidationScope::Both, payload.address.to_owned()),
        cache.clone(),
    )
    .execute()
    .await;
    payload.details.as_ref().map(|d| async move {
        match d {
            PayloadDetails::NewConfirmation(data) => {
                Invalidate::new(
                    InvalidationPattern::Any(
                        InvalidationScope::Both,
                        String::from(&data.safe_tx_hash),
                    ),
                    cache.clone(),
                )
                .execute()
                .await;
            }
            PayloadDetails::ExecutedMultisigTransaction(data) => {
                Invalidate::new(
                    InvalidationPattern::Any(
                        InvalidationScope::Both,
                        String::from(&data.safe_tx_hash),
                    ),
                    cache.clone(),
                )
                .execute()
                .await;
            }
            PayloadDetails::PendingMultisigTransaction(data) => {
                Invalidate::new(
                    InvalidationPattern::Any(
                        InvalidationScope::Both,
                        String::from(&data.safe_tx_hash),
                    ),
                    cache.clone(),
                )
                .execute()
                .await;
            }
            _ => {}
        }
    });
    Ok(())
}
