use crate::common::models::page::PageMetadata;
use std::cmp::max;

pub mod commons;
pub mod details;
pub mod history;
pub mod module;
pub mod multisig;
pub mod preview;
pub mod proposal;
pub mod queued;
pub mod transfers;

#[cfg(test)]
mod tests;

pub(super) fn offset_page_meta(meta: &PageMetadata, offset: i64) -> String {
    PageMetadata {
        offset: (max(0, (meta.offset as i64) + offset)) as u64,
        limit: meta.limit,
    }
    .to_url_string()
}
