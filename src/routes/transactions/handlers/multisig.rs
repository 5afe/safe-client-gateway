use super::commons::get_backend_page;
use crate::common::models::backend::transactions::MultisigTransaction;
use crate::common::models::page::{Page, PageMetadata};
use crate::config::transaction_request_timeout;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::transactions::filters::multisig::MultisigFilters;
use crate::routes::transactions::handlers::offset_page_meta;
use crate::routes::transactions::models::summary::{ConflictType, TransactionListItem};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::urls::build_absolute_uri;

pub async fn get_multisig_transactions(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    cursor: &Option<String>,
    filters: &MultisigFilters,
) -> ApiResult<Page<TransactionListItem>> {
    let info_provider = DefaultInfoProvider::new(chain_id, context);
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/multisig-transactions/",
        safe_address
    )?;
    let page_meta = PageMetadata::from_cursor(cursor.as_ref().unwrap_or(&"".to_string()));

    let backend_txs = get_backend_page(
        &context,
        &url,
        transaction_request_timeout(),
        &page_meta,
        filters,
    )
    .await?;
    let service_txs = backend_txs_to_summary_txs(
        &mut backend_txs.results.into_iter(),
        &info_provider,
        safe_address,
    )
    .await?;

    return Ok(Page {
        next: build_cursor(
            context,
            chain_id,
            safe_address,
            &page_meta,
            backend_txs.next,
            filters,
            1,
        ),
        previous: build_cursor(
            context,
            chain_id,
            safe_address,
            &page_meta,
            backend_txs.previous,
            filters,
            -1,
        ),
        results: service_txs,
    });
}

async fn backend_txs_to_summary_txs(
    transactions: &mut impl Iterator<Item = MultisigTransaction>,
    info_provider: &(impl InfoProvider + Sync),
    safe_address: &str,
) -> ApiResult<Vec<TransactionListItem>> {
    let mut results = vec![];
    for transaction in transactions {
        let tx_summary = transaction
            .to_transaction_summary(info_provider)
            .await?
            .into_iter()
            .for_each(|tx_summary| {
                results.push(TransactionListItem::Transaction {
                    transaction: tx_summary,
                    conflict_type: ConflictType::None,
                });
            });
    }

    Ok(results)
}

fn build_cursor(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    page_meta: &PageMetadata,
    backend_page_url: Option<String>,
    filters: &MultisigFilters,
    direction: i64,
) -> Option<String> {
    backend_page_url.as_ref().map(|_| {
        build_absolute_uri(
            context,
            uri!(
                crate::routes::transactions::routes::get_multisig_transactions(
                    chain_id = chain_id,
                    safe_address = safe_address,
                    cursor = Some(offset_page_meta(
                        page_meta,
                        direction * (page_meta.limit as i64)
                    )),
                    filters = (
                        filters.date.to_owned(),
                        filters.to.to_owned(),
                        filters.value.to_owned(),
                        filters.nonce.to_owned()
                    )
                )
            ),
        )
    })
}
