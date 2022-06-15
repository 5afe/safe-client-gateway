use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveDateTime, Utc};
use itertools::Itertools;

use crate::cache::cache_operations::RequestCached;
use crate::cache::manager::ChainCache;
use crate::common::models::backend::transactions::{CreationTransaction, Transaction};
use crate::common::models::page::{Page, PageMetadata};
use crate::config::transaction_request_timeout;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::transactions::handlers::offset_page_meta;
use crate::routes::transactions::models::summary::{
    ConflictType, TransactionListItem, TransactionSummary,
};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::urls::build_absolute_uri;

pub async fn get_history_transactions(
    context: &RequestContext,
    chain_id: &String,
    safe_address: &String,
    cursor: &Option<String>,
    timezone_offset: &Option<String>,
) -> ApiResult<Page<TransactionListItem>> {
    let info_provider = DefaultInfoProvider::new(chain_id, context);
    let request_timezone_offset = timezone_offset
        .as_ref()
        .and_then(|it| it.parse::<i32>().ok())
        .unwrap_or(0)
        / 1000;

    let incoming_page_metadata =
        PageMetadata::from_cursor(cursor.as_ref().unwrap_or(&"".to_string()));

    let page_metadata = adjust_page_meta(&incoming_page_metadata);
    let extended_page_cursor = Some(page_metadata.to_url_string());

    let backend_paged_txs = fetch_backend_paged_txs(
        context,
        &info_provider,
        safe_address,
        &extended_page_cursor,
        chain_id,
    )
    .await?;
    let mut backend_txs_iter = backend_paged_txs.results.into_iter();
    let prev_page_timestamp = if page_metadata.offset != 0 {
        peek_timestamp_and_remove_item(
            &mut backend_txs_iter,
            &info_provider,
            safe_address,
            request_timezone_offset,
        )
        .await
        .unwrap_or(-1)
    } else {
        -1
    };

    let mut service_txs =
        backend_txs_to_summary_txs(&mut backend_txs_iter, &info_provider, safe_address).await?;
    if backend_paged_txs.next.is_none() {
        if let Ok(creation_tx) =
            get_creation_transaction_summary(context, &info_provider, safe_address, chain_id).await
        {
            service_txs.push(creation_tx);
        }
    }

    let tx_list_items =
        service_txs_to_tx_list_items(service_txs, prev_page_timestamp, request_timezone_offset)?;

    Ok(Page {
        next: build_cursor(
            context,
            chain_id,
            safe_address,
            &incoming_page_metadata,
            timezone_offset,
            backend_paged_txs.next,
            1, // Direction forward
        ),
        previous: build_cursor(
            context,
            chain_id,
            safe_address,
            &incoming_page_metadata,
            timezone_offset,
            backend_paged_txs.previous,
            -1, // Direction backwards
        ),
        results: tx_list_items,
    })
}

fn build_cursor(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    page_meta: &PageMetadata,
    timezone_offset: &Option<String>,
    url: Option<String>,
    direction: i64,
) -> Option<String> {
    url.as_ref().map(|_| {
        build_absolute_uri(
            context,
            uri!(
                crate::routes::transactions::routes::get_transactions_history(
                    chain_id,
                    safe_address,
                    Some(offset_page_meta(
                        page_meta,
                        direction * (page_meta.limit as i64)
                    )),
                    Some(timezone_offset.clone().unwrap_or("0".to_string()))
                )
            ),
        )
    })
}

pub(super) fn adjust_page_meta(meta: &PageMetadata) -> PageMetadata {
    if meta.offset == 0 {
        PageMetadata {
            offset: 0,
            limit: meta.limit,
        }
    } else {
        PageMetadata {
            offset: meta.offset - 1,
            limit: meta.limit + 1,
        }
    }
}

async fn fetch_backend_paged_txs(
    context: &RequestContext,
    info_provider: &impl InfoProvider,
    safe_address: &str,
    cursor: &Option<String>,
    chain_id: &str,
) -> ApiResult<Page<Transaction>> {
    let page_metadata = PageMetadata::from_cursor(cursor.as_ref().unwrap_or(&"".to_string()));
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/all-transactions/?{}&queued=false&executed=true",
        safe_address,
        page_metadata.to_url_string()
    )?;
    log::debug!("request URL: {}", &url);
    log::debug!("cursor: {:#?}", &cursor);
    log::debug!("page_metadata: {:#?}", &page_metadata);
    let body = RequestCached::new_from_context(url, context, ChainCache::from(chain_id))
        .request_timeout(transaction_request_timeout())
        .execute()
        .await?;
    Ok(serde_json::from_str::<Page<Transaction>>(&body)?)
}

pub(super) async fn backend_txs_to_summary_txs(
    txs: &mut impl Iterator<Item = Transaction>,
    info_provider: &(impl InfoProvider + Sync),
    safe_address: &str,
) -> ApiResult<Vec<TransactionSummary>> {
    let mut results = vec![];

    for transaction in txs {
        results.extend(
            transaction
                .to_transaction_summary(info_provider, safe_address)
                .await
                .unwrap_or_default(),
        );
    }

    Ok(results)
}

pub(super) fn service_txs_to_tx_list_items(
    txs: Vec<TransactionSummary>,
    last_timestamp: i64,
    timezone_offset: i32,
) -> ApiResult<Vec<TransactionListItem>> {
    let mut tx_list_items = Vec::new();
    for (date_timestamp, transaction_group) in &txs
        .into_iter()
        .group_by(|transaction| get_day_timestamp_millis(transaction.timestamp, timezone_offset))
    {
        if date_timestamp != last_timestamp {
            tx_list_items.push(TransactionListItem::DateLabel {
                timestamp: date_timestamp,
            });
        }
        transaction_group.for_each(|tx| {
            tx_list_items.push(TransactionListItem::Transaction {
                transaction: tx,
                conflict_type: ConflictType::None,
            })
        });
    }
    Ok(tx_list_items)
}

pub(super) async fn peek_timestamp_and_remove_item(
    transactions: &mut impl Iterator<Item = Transaction>,
    info_provider: &(impl InfoProvider + Sync),
    safe_address: &str,
    timezone_offset: i32,
) -> ApiResult<i64> {
    let timestamp = transactions
        .next()
        .ok_or(api_error!("empty transactions"))?
        .to_transaction_summary(info_provider, safe_address)
        .await?
        .last()
        .ok_or(api_error!("empty transactions"))?
        .timestamp;

    Ok(get_day_timestamp_millis(timestamp, timezone_offset))
}

pub(super) fn get_day_timestamp_millis(timestamp_in_millis: i64, timezone_offset: i32) -> i64 {
    log::debug!("Timezone offset: {:#?}", timezone_offset);
    let date_time = DateTime::<Utc>::from_utc(
        NaiveDateTime::from_timestamp(timestamp_in_millis / 1000, 0),
        Utc,
    )
    .with_timezone(&FixedOffset::east(timezone_offset));

    // we remove the client's timezone offset of the day timestamp so we return the day timestamp at 00:00:00.0000
    // this is particularly important for negative timezone offset.
    let date =
        NaiveDate::from_ymd_opt(date_time.year(), date_time.month(), date_time.day()).unwrap();
    (date.and_hms_milli(0, 0, 0, 0).timestamp() - timezone_offset as i64) * 1000
}

pub(super) async fn get_creation_transaction_summary(
    context: &RequestContext,
    info_provider: &(impl InfoProvider + Sync),
    safe: &String,
    chain_id: &str,
) -> ApiResult<TransactionSummary> {
    let url = core_uri!(info_provider, "/v1/safes/{}/creation/", safe)?;
    debug!("{}", &url);
    let body = RequestCached::new_from_context(url, context, ChainCache::from(chain_id))
        .request_timeout(transaction_request_timeout())
        .execute()
        .await?;

    let creation_transaction_dto: CreationTransaction = serde_json::from_str(&body)?;
    let transaction_summary = creation_transaction_dto
        .to_transaction_summary(safe, info_provider)
        .await;
    Ok(transaction_summary)
}
