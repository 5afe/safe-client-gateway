use std::collections::HashMap;

use crate::common::models::backend::transfers::Transfer;
use crate::common::models::page::{Page, PageMetadata};
use crate::config::transaction_request_timeout;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::transactions::handlers::offset_page_meta;
use crate::routes::transactions::models::summary::{
    ConflictType, TransactionListItem, TransactionSummary,
};
use crate::routes::transactions::models::TransactionStatus;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::urls::build_absolute_uri;

use super::commons::get_backend_page;

pub async fn get_incoming_transfers(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    cursor: Option<String>,
    filters: HashMap<String, String>,
) -> ApiResult<Page<TransactionListItem>> {
    let info_provider = DefaultInfoProvider::new(chain_id, context);
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/incoming-transfers/",
        safe_address
    )?;

    let page_metadata_params: HashMap<String, String> = cursor
        .map(|c| PageMetadata::from_cursor(&c).into())
        .unwrap_or(HashMap::new());

    // Merge filter query with cursor query
    let query_params: HashMap<String, String> =
        page_metadata_params.into_iter().chain(filters).collect();

    let backend_txs =
        get_backend_page(&context, &url, transaction_request_timeout(), &query_params).await?;

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
            page_meta.as_ref(),
            backend_txs.next,
            &filters,
            1,
        ),
        previous: build_cursor(
            context,
            chain_id,
            safe_address,
            page_meta.as_ref(),
            backend_txs.previous,
            &filters,
            -1,
        ),
        results: service_txs,
    });
}

async fn backend_txs_to_summary_txs(
    transfers: &mut impl Iterator<Item = Transfer>,
    info_provider: &(impl InfoProvider + Sync),
    safe_address: &str,
) -> ApiResult<Vec<TransactionListItem>> {
    let mut results = vec![];
    for transfer in transfers {
        let tx_summary = transfer
            .to_transaction_summary(
                info_provider,
                transfer.get_execution_time().unwrap_or(0), // TODO does this make sense?
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

fn build_cursor(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    page_meta: Option<&PageMetadata>,
    backend_page_url: Option<String>,
    filters: &HashMap<String, String>,
    direction: i64,
) -> Option<String> {
    backend_page_url.as_ref().map(|_| {
        let cursor = page_meta
            .map(|page_meta| offset_page_meta(page_meta, direction * (page_meta.limit as i64)));

        build_absolute_uri(
            context,
            uri!(crate::routes::transactions::routes::get_incoming_transfers(
                chain_id = chain_id,
                safe_address = safe_address,
                cursor = cursor,
                filters = filters.to_owned(),
            )),
        )
    })
}
