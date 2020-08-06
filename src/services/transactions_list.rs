extern crate reqwest;

use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::models::backend::transactions::{Transaction, CreationTransaction};
use crate::models::service::transactions::{ID_PREFIX_CREATION_TX, TransactionStatus, TransactionInfo, Creation};
use crate::models::service::transactions::summary::TransactionSummary;
use crate::models::commons::Page;
use crate::utils::context::Context;
use crate::utils::extract_query_string;
use crate::providers::info::DefaultInfoProvider;
use anyhow::Result;
use log::debug;

pub fn get_all_transactions(context: &Context, safe_address: &String, next: &Option<String>) -> Result<Page<TransactionSummary>> {
    let mut info_provider = DefaultInfoProvider::new(context);
    let url = format!(
        "{}/safes/{}/all-transactions/?{}",
        base_transaction_service_url(),
        safe_address,
        next.as_ref().unwrap_or(&String::new())
    );
    let body = context.cache().request_cached(&context.client(), &url, request_cache_duration())?;
    debug!("request URL: {}", &url);
    debug!("next: {:#?}", next);
    debug!("{:#?}", body);
    let backend_transactions: Page<Transaction> = serde_json::from_str(&body)?;
    let mut service_transactions: Vec<TransactionSummary> = backend_transactions.results.into_iter()
        .flat_map(|transaction| transaction.to_transaction_summary(&mut info_provider, safe_address).unwrap_or(vec!()))
        .collect();
    if backend_transactions.next.is_none() {
        let creation_transaction = get_creation_transaction_summary(context, safe_address)?;
        service_transactions.push(creation_transaction);
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
    safe: &str,
) -> Result<TransactionSummary> {
    let url = format!(
        "{}/safes/{}/creation",
        base_transaction_service_url(),
        safe
    );
    debug!("{}", &url);
    let body = context
        .cache()
        .request_cached(&context.client(), &url, request_cache_duration())?;

    let creation_transaction_dto: CreationTransaction = serde_json::from_str(&body)?;
    let transaction_summary = TransactionSummary {
        id: create_id!(ID_PREFIX_CREATION_TX, safe),
        timestamp: creation_transaction_dto.created.timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Creation(
            Creation {
                creator: creation_transaction_dto.creator,
                transaction_hash: creation_transaction_dto.transaction_hash,
                master_copy: creation_transaction_dto.master_copy,
                factory: creation_transaction_dto.factory_address,
            }
        ),
        execution_info: None,
    };
    Ok(transaction_summary)
}