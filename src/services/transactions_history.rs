extern crate reqwest;

use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::models::backend::transactions::Transaction;
use crate::models::commons::{Page, PageMetadata};
use crate::models::service::transactions::summary::{
    ConflictType, TransactionListItem, TransactionSummary,
};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::services::transactions_list::get_creation_transaction_summary;
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use crate::utils::extract_query_string;
use anyhow::Result;
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Utc};
use itertools::Itertools;
use std::cmp::max;

pub fn get_history_transactions(
    context: &Context,
    safe_address: &str,
    page_url: &Option<String>,
    _timezone_offset: &Option<String>,
) -> ApiResult<Page<TransactionListItem>> {
    let is_first_page = page_url.is_none();
    let mut info_provider = DefaultInfoProvider::new(context);

    if is_first_page {
        let backend_paged_txs = fetch_backend_paged_txs(context, safe_address, page_url)?;
        let service_txs = backend_txs_to_summary_txs(
            backend_paged_txs.results,
            &mut info_provider,
            safe_address,
        )?;

        let tx_list_items = service_txs_to_tx_list_items(service_txs, -1)?; // we use an invalid timestamp

        Ok(Page {
            next: prepare_page_url(context, backend_paged_txs.next.as_ref(), safe_address),
            previous: None,
            results: tx_list_items,
        })
    } else {
        let page_url = page_url.as_ref().unwrap();
        let incoming_page_metadata = PageMetadata::from_url_string(&page_url);
        let page_metadata = PageMetadata {
            offset: max(0, incoming_page_metadata.offset - 1),
            limit: incoming_page_metadata.limit + 1,
        };
        let extended_page_url = Some(page_metadata.to_url_string());

        let mut backend_paged_txs =
            fetch_backend_paged_txs(context, safe_address, &extended_page_url)?;
        let prev_page_timestamp = peek_timestamp_and_remove_item(
            &mut backend_paged_txs.results,
            &mut info_provider,
            safe_address,
        )?;

        let service_txs = backend_txs_to_summary_txs(
            backend_paged_txs.results,
            &mut info_provider,
            safe_address,
        )?;

        let tx_list_items = service_txs_to_tx_list_items(service_txs, prev_page_timestamp)?;

        let backend_link_chunks: Vec<&str> = backend_paged_txs
            .previous
            .as_ref()
            .unwrap()
            .split("?")
            .collect();
        let backend_url_prefix = backend_link_chunks[0];

        let next = quick_page_metadata_restore_next(backend_url_prefix, &incoming_page_metadata);
        let prev = quick_page_metadata_restore_prev(backend_url_prefix, &incoming_page_metadata);

        Ok(Page {
            next: prepare_page_url(context, next.as_ref(), safe_address),
            previous: prepare_page_url(context, prev.as_ref(), safe_address),
            results: tx_list_items,
        })
    }
}

fn quick_page_metadata_restore_next(
    backend_url_prefix: &str,
    page_metadata: &PageMetadata,
) -> Option<String> {
    Some(format!(
        "{}?{}",
        backend_url_prefix,
        PageMetadata {
            offset: page_metadata.offset + 20,
            limit: 20,
        }
        .to_url_string()
    ))
}
fn quick_page_metadata_restore_prev(
    backend_url_prefix: &str,
    page_metadata: &PageMetadata,
) -> Option<String> {
    Some(format!(
        "{}?{}",
        backend_url_prefix,
        PageMetadata {
            offset: max(0, page_metadata.offset - 20),
            limit: 20,
        }
        .to_url_string()
    ))
}

fn fetch_backend_paged_txs(
    context: &Context,
    safe_address: &str,
    page_url: &Option<String>,
) -> Result<Page<Transaction>> {
    let page_metadata = PageMetadata::from_url_string(page_url.as_ref().unwrap_or(&"".to_string()));
    let url = format!(
        "{}/v1/safes/{}/all-transactions/?{}&queued=false",
        base_transaction_service_url(),
        safe_address,
        page_metadata.to_url_string()
    );
    let body = context
        .cache()
        .request_cached(&context.client(), &url, request_cache_duration())?;
    log::debug!("request URL: {}", &url);
    log::debug!("page_url: {:#?}", page_url);
    log::debug!("page_metadata: {:#?}", page_metadata);
    Ok(serde_json::from_str::<Page<Transaction>>(&body)?)
}

fn backend_txs_to_summary_txs(
    txs: Vec<Transaction>,
    info_provider: &mut dyn InfoProvider,
    safe_address: &str,
) -> Result<Vec<TransactionSummary>> {
    Ok(txs
        .into_iter()
        .flat_map(|transaction| {
            transaction
                .to_transaction_summary(info_provider, safe_address)
                .unwrap_or(vec![])
        })
        .collect())
}

//TODO include last page flag for creation tx
fn service_txs_to_tx_list_items(
    txs: Vec<TransactionSummary>,
    last_timestamp: i64,
) -> Result<Vec<TransactionListItem>> {
    let mut tx_list_items = Vec::new();
    for (date_timestamp, transaction_group) in &txs
        .into_iter()
        .group_by(|transaction| get_day_timestamp_millis(transaction.timestamp / 1000))
    {
        if last_timestamp < 0 || date_timestamp < last_timestamp {
            tx_list_items.push(TransactionListItem::DateLabel {
                timestamp: date_timestamp,
            });
        }
        transaction_group.for_each(|tx| {
            tx_list_items.push(TransactionListItem::Transaction {
                transaction_summary: tx,
                conflict_type: ConflictType::None,
            })
        });
    }
    Ok(tx_list_items)
}

// TODO too side-effect-y
// borrows list, removes first item, converts gets first of everything and evaluates timestamp
fn peek_timestamp_and_remove_item(
    transactions: &mut Vec<Transaction>,
    info_provider: &mut DefaultInfoProvider,
    safe_address: &str,
) -> Result<i64> {
    let timestamp = transactions
        .remove(0)
        .to_transaction_summary(info_provider, safe_address)?
        .get(0)
        .ok_or(anyhow::anyhow!("empty transactions"))?
        .timestamp;

    Ok(get_day_timestamp_millis(timestamp))
}

fn prepare_page_url(
    context: &Context,
    query_params: Option<&String>,
    safe_address: &str,
) -> Option<String> {
    query_params
        .and_then(|link| extract_query_string(link))
        .map(|link| {
            context.build_absolute_url(uri!(
                crate::routes::transactions::history_transactions: safe_address,
                link,
                "" // timezone_offset
            ))
        })
}

fn get_day_timestamp_millis(timestamp_in_secs: i64) -> i64 {
    let date_time =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp_in_secs, 0), Utc);
    let date =
        NaiveDate::from_ymd_opt(date_time.year(), date_time.month(), date_time.day()).unwrap();
    date.and_hms_milli(0, 0, 0, 0).timestamp() * 1000
}
