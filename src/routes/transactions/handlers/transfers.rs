use crate::{
    cache::cache_operations::RequestCached,
    common::models::{
        backend::{transactions::Transaction, transfers::Transfer},
        page::{Page, PageMetadata},
    },
    config::transaction_request_timeout,
    providers::info::{DefaultInfoProvider, InfoProvider},
    routes::transactions::{
        handlers::offset_page_meta,
        models::{
            filters::{QueryParam, TransferFilters},
            summary::{ConflictType, TransactionListItem},
        },
    },
    utils::{context::RequestContext, errors::ApiResult, urls::build_absolute_uri},
};

use super::commons::get_backend_page;

pub async fn get_incoming_transfers(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    cursor: &Option<String>,
    filters: &TransferFilters,
) -> ApiResult<Page<TransactionListItem>> {
    let info_provider = DefaultInfoProvider::new(chain_id, context);
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/incoming-transfers/",
        safe_address
    )?;

    let backend_txs = get_backend_page(&context, &url, cursor, filters).await?;
    let service_txs = backend_txs_to_summary_txs(
        &mut backend_txs.results.into_iter(),
        &info_provider,
        safe_address,
    )
    .await?;

    return Ok(Page {
        next: None,
        previous: None,
        results: service_txs,
    });
}

pub(super) async fn backend_txs_to_summary_txs(
    transfers: &mut impl Iterator<Item = Transfer>,
    info_provider: &(impl InfoProvider + Sync),
    safe_address: &str,
) -> ApiResult<Vec<TransactionListItem>> {
    let mut results = vec![];
    for transfer in transfers {
        let tx_summary = transfer
            .to_transaction_summary(
                info_provider,
                transfer.get_execution_time().unwrap_or(0), //TODO does this make sense?
                safe_address,
            )
            .await;
        results.push(TransactionListItem::Transaction {
            transaction: tx_summary,
            conflict_type: ConflictType::None,
        });
    }

    Ok(results)
}

pub fn build_cursor(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    page_meta: &PageMetadata,
    backend_page_url: Option<&String>,
    filters: &TransferFilters,
    direction: i64,
) -> Option<String> {
    backend_page_url.as_ref().map(|_| {
        build_absolute_uri(
            context,
            uri!(crate::routes::transactions::routes::get_incoming_transfers(
                chain_id = chain_id,
                safe_address = safe_address,
                cursor = Some(offset_page_meta(
                    page_meta,
                    direction * (page_meta.limit as i64)
                )),
                filters = (
                    filters.to.unwrap_or_default(),
                    filters.date.unwrap_or_default(),
                    filters.token_address.unwrap_or_default(),
                    filters.value.unwrap_or_default()
                )
            )),
        )
    })
}
