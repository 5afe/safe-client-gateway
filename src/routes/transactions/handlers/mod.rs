use crate::common::models::page::PageMetadata;
use std::cmp::max;

pub mod details;
pub mod history;
pub mod proposal;
pub mod queued;

#[cfg(test)]
mod tests;

pub fn offset_page_meta(meta: &PageMetadata, offset: i64) -> String {
    PageMetadata {
        offset: (max(0, (meta.offset as i64) + offset)) as u64,
        limit: meta.limit,
    }
    .to_url_string()
}
