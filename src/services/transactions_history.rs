extern crate reqwest;

use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::models::backend::transactions::Transaction;
use crate::models::commons::Page;
use crate::models::service::transactions::summary::{
    ConflictType, TransactionListItem, TransactionSummary,
};
use crate::providers::info::DefaultInfoProvider;
use crate::services::transactions_list::get_creation_transaction_summary;
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use crate::utils::extract_query_string;
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Utc};
use itertools::Itertools;
use log::debug;

pub fn get_history_transactions(
    context: &Context,
    safe_address: &String,
    page_url: &Option<String>,
    timezone_offset: &Option<String>,
) -> ApiResult<Page<TransactionListItem>> {
    let mut info_provider = DefaultInfoProvider::new(context);
    let url = format!(
        "{}/v1/safes/{}/all-transactions/?{}&queued=false",
        base_transaction_service_url(),
        safe_address,
        page_url.as_ref().unwrap_or(&String::new())
    );
    let body = context
        .cache()
        .request_cached(&context.client(), &url, request_cache_duration())?;
    debug!("request URL: {}", &url);
    debug!("page_url: {:#?}", page_url);
    let backend_transactions: Page<Transaction> = serde_json::from_str(&body)?;
    let mut service_transactions: Vec<TransactionSummary> = backend_transactions
        .results
        .into_iter()
        .flat_map(|transaction| {
            transaction
                .to_transaction_summary(&mut info_provider, safe_address)
                .unwrap_or(vec![])
        })
        .collect();

    if backend_transactions.next.is_none() {
        if let Ok(creation_transaction) = get_creation_transaction_summary(context, safe_address) {
            service_transactions.push(creation_transaction)
        }
    }

    let mut service_transactions_with_dates = Vec::new();
    for (date_timestamp, transaction_group) in &service_transactions
        .into_iter()
        .group_by(|transaction| get_day_timestamp_millis(transaction.timestamp / 1000))
    {
        service_transactions_with_dates.push(TransactionListItem::DateLabel {
            timestamp: date_timestamp,
        });
        transaction_group.for_each(|tx| {
            service_transactions_with_dates.push(TransactionListItem::Transaction {
                transaction_summary: tx,
                conflict_type: ConflictType::None,
            })
        });
    }
    Ok(Page {
        next: backend_transactions
            .next
            .as_ref()
            .and_then(|link| extract_query_string(link))
            .map(|link| {
                context.build_absolute_url(uri!(
                    crate::routes::transactions::history_transactions: safe_address,
                    link,
                    "" // timezone_offset
                ))
            }),
        previous: backend_transactions
            .previous
            .as_ref()
            .and_then(|link| extract_query_string(link))
            .map(|link| {
                context.build_absolute_url(uri!(
                    crate::routes::transactions::history_transactions: safe_address,
                    link,
                    "" // timezone_offset
                ))
            }),
        results: service_transactions_with_dates,
    })
}

fn get_day_timestamp_millis(timestamp_in_secs: i64) -> i64 {
    let date_time =
        DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(timestamp_in_secs, 0), Utc);
    let date =
        NaiveDate::from_ymd_opt(date_time.year(), date_time.month(), date_time.day()).unwrap();
    date.and_hms_milli(0, 0, 0, 0).timestamp() * 1000
}
