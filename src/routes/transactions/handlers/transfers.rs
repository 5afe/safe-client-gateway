use crate::{
    cache::cache_operations::RequestCached,
    common::models::{
        backend::transactions::Transaction,
        page::{Page, PageMetadata},
    },
    config::transaction_request_timeout,
    providers::info::{DefaultInfoProvider, InfoProvider},
    routes::transactions::models::{
        filters::{QueryParam, TransferFilters},
        summary::TransactionListItem,
    },
    utils::{context::RequestContext, errors::ApiResult},
};

pub async fn get_incoming_transfers(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    cursor: &Option<String>,
    filters: &TransferFilters,
) -> ApiResult<Page<TransactionListItem>> {
    let info_provider = DefaultInfoProvider::new(chain_id, context);
    let incoming_page_metadata =
        PageMetadata::from_cursor(cursor.as_ref().unwrap_or(&"".to_string()));

    return Ok(Page {
        next: None,
        previous: None,
        results: vec![],
    });
}

async fn fetch_backend_paged_txs(
    context: &RequestContext,
    info_provider: &impl InfoProvider,
    safe_address: &str,
    cursor: &Option<String>,
    filters: &impl QueryParam,
) -> ApiResult<Page<Transaction>> {
    let other_filters = filters.as_query_param();
    let page_metadata = PageMetadata::from_cursor(cursor.as_ref().unwrap_or(&"".to_string()));
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/incoming-transfers/?{}&{}",
        safe_address,
        page_metadata.to_url_string(),
        other_filters
    )?;
    log::debug!("request URL: {}", &url);
    log::debug!("cursor: {:#?}", &cursor);
    log::debug!("page_metadata: {:#?}", &page_metadata);
    let body = RequestCached::new_from_context(url, context)
        .request_timeout(transaction_request_timeout())
        .execute()
        .await?;
    Ok(serde_json::from_str::<Page<Transaction>>(&body)?)
}
