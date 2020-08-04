extern crate reqwest;

use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::models::backend::transactions::Transaction;
use crate::models::service::transactions::summary::TransactionSummary;
use crate::models::commons::Page;
use crate::utils::context::Context;
use crate::utils::extract_query_string;
use crate::providers::info::InfoProvider;
use anyhow::Result;

pub fn get_all_transactions(context: &Context, safe_address: &String, next: &Option<String>) -> Result<Page<TransactionSummary>> {
    let mut info_provider = InfoProvider::new(context);
    let url = format!(
        "{}/safes/{}/all-transactions/?{}",
        base_transaction_service_url(),
        safe_address,
        next.as_ref().unwrap_or(&String::new())
    );
    let body = context.cache().request_cached(&context.client(), &url, request_cache_duration())?;
    println!("request URL: {}", &url);
    println!("next: {:#?}", next);
    println!("{:#?}", body);
    let backend_transactions: Page<Transaction> = serde_json::from_str(&body)?;
    let service_transactions: Vec<TransactionSummary> = backend_transactions.results.into_iter()
        .flat_map(|transaction| transaction.to_transaction_summary(&mut info_provider, safe_address).unwrap_or(vec!()))
        .collect();

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
