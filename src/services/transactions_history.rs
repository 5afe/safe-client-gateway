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

        let tx_list_items = service_txs_to_tx_list_items(service_txs)?;

        Ok(Page {
            next: prepare_page_url(context, backend_paged_txs.next.as_ref(), safe_address),
            previous: None,
            results: tx_list_items,
        })
    } else {
        Ok(Page {
            next: None,
            previous: None,
            results: vec![],
        })
    }
    // let mut backend_transactions: Page<Transaction> = serde_json::from_str(&body)?;
    //
    // if is_first_page {
    // } else {
    // }
    // let previous_timestamp_label = remove_first_and_peek_timestamp(
    //     &mut backend_transactions.results,
    //     &mut info_provider,
    //     &safe_address,
    // );
    //
    // log::error!("RETURNED COUNT: {}", &backend_transactions.results.len()); //should always be 21
    //
    // let mut service_transactions: Vec<TransactionSummary> = backend_transactions
    //     .results
    //     .into_iter()
    //     .flat_map(|transaction| {
    //         transaction
    //             .to_transaction_summary(&mut info_provider, safe_address)
    //             .unwrap_or(vec![])
    //     })
    //     .collect();
    //
    // if backend_transactions.next.is_none() {
    //     if let Ok(creation_transaction) = get_creation_transaction_summary(context, safe_address) {
    //         service_transactions.push(creation_transaction)
    //     }
    // }
    //
    // let mut service_transactions_with_dates = Vec::new();
    // for (date_timestamp, transaction_group) in &service_transactions
    //     .into_iter()
    //     .group_by(|transaction| get_day_timestamp_millis(transaction.timestamp / 1000))
    // {
    //     service_transactions_with_dates.push(TransactionListItem::DateLabel {
    //         timestamp: date_timestamp,
    //     });
    //     transaction_group.for_each(|tx| {
    //         service_transactions_with_dates.push(TransactionListItem::Transaction {
    //             transaction_summary: tx,
    //             conflict_type: ConflictType::None,
    //         })
    //     });
    // }
    // Ok(Page {
    //     next: backend_transactions
    //         .next
    //         .as_ref()
    //         .and_then(|link| extract_query_string(link))
    //         .map(|link| {
    //             log::error!("NEXT LINK: {}", &link);
    //             context.build_absolute_url(uri!(
    //                 crate::routes::transactions::history_transactions: safe_address,
    //                 restore_page_size(link, is_first_page),
    //                 "" // timezone_offset
    //             ))
    //         }),
    //     previous: backend_transactions
    //         .previous
    //         .as_ref()
    //         .and_then(|link| extract_query_string(link))
    //         .map(|link| {
    //             context.build_absolute_url(uri!(
    //                 crate::routes::transactions::history_transactions: safe_address,
    //                 restore_page_size(link, is_first_page),
    //                 "" // timezone_offset
    //             ))
    //         }),
    //     results: service_transactions_with_dates,
    // })
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
    log::error!("request URL: {}", &url);
    log::error!("page_url: {:#?}", page_url);
    log::error!("page_metadata: {:#?}", page_metadata);
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

//TODO include guard for last page timestamp and is last page flag for creation tx
fn service_txs_to_tx_list_items(txs: Vec<TransactionSummary>) -> Result<Vec<TransactionListItem>> {
    let mut tx_list_items = Vec::new();
    for (date_timestamp, transaction_group) in &txs
        .into_iter()
        .group_by(|transaction| get_day_timestamp_millis(transaction.timestamp / 1000))
    {
        tx_list_items.push(TransactionListItem::DateLabel {
            timestamp: date_timestamp,
        });
        transaction_group.for_each(|tx| {
            tx_list_items.push(TransactionListItem::Transaction {
                transaction_summary: tx,
                conflict_type: ConflictType::None,
            })
        });
    }
    Ok(tx_list_items)
}

fn build_page_metadata_extended_size(page_url: &Option<String>) -> PageMetadata {
    if let Some(page_url_str) = page_url {
        let page_metadata = PageMetadata::from_url_string(page_url_str);
        PageMetadata {
            limit: page_metadata.limit + 1,
            offset: page_metadata.offset - 1,
        }
    } else {
        // first page
        PageMetadata {
            limit: 21,
            offset: 0,
        }
    }
}

// TODO too side-effect-y
// borrows list, removes first item, converts gets first of everything and evaluates timestamp
fn remove_first_and_peek_timestamp(
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

fn restore_page_size(link: String, is_first_page: bool) -> String {
    let page_metadata = PageMetadata::from_url_string(&link);
    let page_metadata = PageMetadata {
        offset: if is_first_page {
            20
        } else {
            page_metadata.offset
        },
        limit: 20,
    };
    page_metadata.to_url_string()
}

fn get_day_timestamp_millis(timestamp_in_secs: i64) -> i64 {
    let date_time =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp_in_secs, 0), Utc);
    let date =
        NaiveDate::from_ymd_opt(date_time.year(), date_time.month(), date_time.day()).unwrap();
    date.and_hms_milli(0, 0, 0, 0).timestamp() * 1000
}
