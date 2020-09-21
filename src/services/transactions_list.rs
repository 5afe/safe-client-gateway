extern crate reqwest;

use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::models::backend::transactions::{Transaction, CreationTransaction};
use crate::models::service::transactions::summary::TransactionSummary;
use crate::models::commons::Page;
use crate::utils::context::Context;
use crate::utils::extract_query_string;
use crate::utils::cache::CacheExt;
use crate::providers::info::DefaultInfoProvider;
use log::debug;
use crate::utils::errors::ApiResult;

pub fn get_all_transactions(context: &Context, safe_address: &String, page_url: &Option<String>) -> ApiResult<Page<TransactionSummary>> {
    let mut info_provider = DefaultInfoProvider::new(context);
    let url = format!(
        "{}/v1/safes/{}/all-transactions/?{}",
        base_transaction_service_url(),
        safe_address,
        page_url.as_ref().unwrap_or(&String::new())
    );
    let body = context.cache().request_cached(&context.client(), &url, request_cache_duration())?;
    debug!("request URL: {}", &url);
    debug!("page_url: {:#?}", page_url);
    debug!("{:#?}", body);
    let backend_transactions: Page<Transaction> = serde_json::from_str(&body)?;
    let mut service_transactions: Vec<TransactionSummary> = backend_transactions.results.into_iter()
        .flat_map(|transaction| transaction.to_transaction_summary(&mut info_provider, safe_address).unwrap_or(vec!()))
        .collect();
    if backend_transactions.next.is_none() {
        if let Ok(creation_transaction) = get_creation_transaction_summary(context, safe_address) {
            service_transactions.push(creation_transaction);
        }
    }

    Ok(Page {
        next: backend_transactions.next.as_ref()
            .and_then(|link| extract_query_string(link))
            .map(|link|
                context.build_absolute_url(uri!(crate::routes::transactions::all: safe_address, link))
            ),
        previous: backend_transactions.previous.as_ref()
            .and_then(|link| extract_query_string(link))
            .map(|link|
                context.build_absolute_url(uri!(crate::routes::transactions::all: safe_address, link))
            ),
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