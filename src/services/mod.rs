use crate::models::commons::PageMetadata;
use crate::providers::info::InfoProvider;
use crate::utils::errors::ApiResult;
use std::cmp::max;

pub mod about;
pub mod balances;
pub mod hooks;
pub mod safes;
pub mod transactions_details;
pub mod transactions_history;
pub mod transactions_proposal;
pub mod transactions_queued;

#[cfg(test)]
mod tests;

pub fn offset_page_meta(meta: &PageMetadata, offset: i64) -> String {
    PageMetadata {
        offset: (max(0, (meta.offset as i64) + offset)) as u64,
        limit: meta.limit,
    }
    .to_url_string()
}

pub async fn backend_url(
    chain_id: &str,
    info_provider: impl InfoProvider,
    path_query: impl Fn() -> String,
) -> ApiResult<String> {
    let chain_info = info_provider.chain_info(chain_id).await?;
    Ok(format!("{}{}", chain_info.tx_service_url, path_query()))
}
