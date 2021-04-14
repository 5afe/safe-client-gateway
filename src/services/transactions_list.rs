extern crate reqwest;

use crate::cache::cache_operations::RequestCached;
use crate::config::{base_transaction_service_url, transaction_request_timeout};
use crate::models::backend::transactions::{CreationTransaction, Transaction};
use crate::models::commons::Page;
use crate::models::service::transactions::summary::TransactionSummary;
use crate::providers::info::DefaultInfoProvider;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use crate::utils::extract_query_string;
use log::debug;

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
    debug!("request URL: {}", &url);
    debug!("page_url: {:#?}", page_url);
    let body = RequestCached::new(url)
        .request_timeout(transaction_request_timeout())
        .execute(context.client(), context.cache())?;
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
                context.build_absolute_url(uri!(
                    crate::routes::transactions::all: safe_address,
                    Some(link)
                ))
            }),
        previous: backend_transactions
            .previous
            .as_ref()
            .and_then(|link| extract_query_string(link))
            .map(|link| {
                context.build_absolute_url(uri!(
                    crate::routes::transactions::all: safe_address,
                    Some(link)
                ))
            }),
        results: service_transactions,
    })
}

pub(super) fn get_creation_transaction_summary(
    context: &Context,
    safe: &String,
) -> ApiResult<TransactionSummary> {
    let url = format!(
        "{}/v1/safes/{}/creation/",
        base_transaction_service_url(),
        safe
    );
    debug!("{}", &url);
    let body = RequestCached::new(url)
        .request_timeout(transaction_request_timeout())
        .execute(context.client(), context.cache())?;

    let mut info_provider = DefaultInfoProvider::new(context);

    let creation_transaction_dto: CreationTransaction = serde_json::from_str(&body)?;
    let transaction_summary =
        creation_transaction_dto.to_transaction_summary(safe, &mut info_provider);
    Ok(transaction_summary)
}
