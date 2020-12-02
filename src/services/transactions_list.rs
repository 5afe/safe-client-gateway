extern crate reqwest;

use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::models::backend::transactions::{CreationTransaction, Transaction};
use crate::models::commons::Page;
use crate::models::service::transactions::summary::{
    ConflictType, TransactionListItem, TransactionSummary,
};
use crate::providers::info::DefaultInfoProvider;
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use crate::utils::extract_query_string;
use chrono::{DateTime, Datelike, NaiveDate, NaiveDateTime, Utc};
use itertools::Itertools;
use log::debug;
use rocket::http::Cookie;

pub fn get_all_transactions(
    context: &Context,
    safe_address: &String,
    page_url: &Option<String>,
) -> ApiResult<Page<TransactionSummary>> {
    let mut info_provider = DefaultInfoProvider::new(context);
    let url = format!(
        "{}/v1/safes/{}/all-transactions/?{}",
        base_transaction_service_url(),
        safe_address,
        page_url.as_ref().unwrap_or(&String::new())
    );
    let body = context
        .cache()
        .request_cached(&context.client(), &url, request_cache_duration())?;
    debug!("request URL: {}", &url);
    debug!("page_url: {:#?}", page_url);
    debug!("{:#?}", body);
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
            service_transactions.push(creation_transaction);
        }
    }

    Ok(Page {
        next: backend_transactions
            .next
            .as_ref()
            .and_then(|link| extract_query_string(link))
            .map(|link| {
                context
                    .build_absolute_url(uri!(crate::routes::transactions::all: safe_address, link))
            }),
        previous: backend_transactions
            .previous
            .as_ref()
            .and_then(|link| extract_query_string(link))
            .map(|link| {
                context
                    .build_absolute_url(uri!(crate::routes::transactions::all: safe_address, link))
            }),
        results: service_transactions,
    })
}

fn get_creation_transaction_summary(
    context: &Context,
    safe: &String,
) -> ApiResult<TransactionSummary> {
    let url = format!(
        "{}/v1/safes/{}/creation/",
        base_transaction_service_url(),
        safe
    );
    debug!("{}", &url);
    let body = context
        .cache()
        .request_cached(&context.client(), &url, request_cache_duration())?;

    let creation_transaction_dto: CreationTransaction = serde_json::from_str(&body)?;
    let transaction_summary = creation_transaction_dto.to_transaction_summary(safe);
    Ok(transaction_summary)
}

pub fn get_history_transactions(
    context: &Context,
    safe_address: &String,
    page_url: &Option<String>,
    timezone_offset: &Option<String>,
    mut last_timestamp_cookie: Option<Cookie>,
) -> ApiResult<Page<TransactionListItem>> {
    let mut info_provider = DefaultInfoProvider::new(context);
    let url = format!(
        "{}/v1/safes/{}/all-transactions/?{}&queued=true",
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

    let last_timestamp = last_timestamp_cookie
        .as_ref()
        .map(|it| it.value().to_owned());
    let mut service_transactions_with_dates = Vec::new();
    for (date, transaction_group) in &service_transactions.into_iter().group_by(|transaction| {
        let date_time = DateTime::<Utc>::from_utc(
            NaiveDateTime::from_timestamp(transaction.timestamp / 1000, 0),
            Utc,
        );
        NaiveDate::from_ymd_opt(date_time.year(), date_time.month(), date_time.day()).unwrap()
    }) {
        let date_timestamp = date.and_hms_milli(0, 0, 0, 0).timestamp();
        if let Some(last_timestamp_str) = last_timestamp.as_ref() {
            log::info!("THE LAST TIMESTAMP: {:#?}", &last_timestamp_str);
            if last_timestamp_str.parse::<i64>().unwrap() < date_timestamp {
                service_transactions_with_dates.push(TransactionListItem::DateLabel {
                    timestamp: date_timestamp * 1000,
                });
                // last_timestamp_cookie
                //     .as_mut()
                //     .map(|cookie| cookie.set_value(date_timestamp.to_string()));
            }
        } else {
            service_transactions_with_dates.push(TransactionListItem::DateLabel {
                timestamp: date_timestamp * 1000,
            });
            // last_timestamp_cookie
            //     .as_mut()
            //     .map(|cookie| cookie.set_value(date_timestamp.to_string()));
        }

        log::error!("THE LAST TIMESTAMP: {:#?}", &last_timestamp_cookie);

        last_timestamp_cookie
            .as_mut()
            .map(|cookie| cookie.set_value(date_timestamp.to_string()));
        //naive approach, no pagination awareness
        // service_transactions_with_dates.push(TransactionListItem::DateLabel {
        //     timestamp: date_timestamp,
        // });
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

// use https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.peekable
pub fn get_queued_transactions(
    context: &Context,
    safe_address: &String,
    page_url: &Option<String>,
    timezone_offset: &Option<String>,
) -> ApiResult<Page<TransactionListItem>> {
    let mut info_provider = DefaultInfoProvider::new(context);
    let url = format!(
        "{}/v1/safes/{}/multisig-transactions/?{}&executed=false",
        base_transaction_service_url(),
        safe_address,
        page_url.as_ref().unwrap_or(&String::new())
    );
    let body = context
        .cache()
        .request_cached(&context.client(), &url, request_cache_duration())?;
    debug!("request URL: {}", &url);
    debug!("page_url: {:#?}", page_url);
    debug!("{:#?}", body);
    let _backend_transactions: Page<Transaction> = serde_json::from_str(&body)?;

    Ok(Page {
        next: None,
        previous: None,
        results: vec![
            TransactionListItem::StringLabel {
                label: String::from("touched queued endpoint"),
            },
            TransactionListItem::DateLabel { timestamp: 12378 },
        ],
    })
}
