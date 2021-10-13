use crate::models::commons::PageMetadata;
use std::cmp::max;

pub mod about;
pub mod balances;
pub mod chains;
pub mod collectibles;
pub mod estimations;
pub mod hooks;
pub mod notifications;
pub mod safe_apps;
pub mod safes;
pub mod transactions_details;
pub mod transactions_history;
pub mod transactions_proposal;
pub mod transactions_queued;
pub mod utils;

#[cfg(test)]
mod tests;

pub fn offset_page_meta(meta: &PageMetadata, offset: i64) -> String {
    PageMetadata {
        offset: (max(0, (meta.offset as i64) + offset)) as u64,
        limit: meta.limit,
    }
    .to_url_string()
}
