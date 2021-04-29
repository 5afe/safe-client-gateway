use crate::cache::cache_operations::RequestCached;
use crate::config::{base_transaction_service_url, transaction_request_timeout};
use crate::models::backend::transactions::{MultisigTransaction, Transaction};
use crate::models::backend::transfers::Transfer;
use crate::models::commons::Page;
use crate::models::service::safes::{SafeInfoEx, SafeLastChanges, SafeState};
use crate::models::service::transactions::summary::TransactionSummary;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use chrono::{DateTime, TimeZone, Utc};

pub fn get_safe_info_ex(context: &Context, safe_address: &String) -> ApiResult<SafeState> {
    let mut info_provider = DefaultInfoProvider::new(context);
    let safe_info = info_provider.safe_info(safe_address)?;

    let safe_info_ex = safe_info.to_safe_info_ex(&mut info_provider);

    let safe_state = SafeState {
        safe_config: safe_info_ex,
        safe_state: SafeLastChanges {
            collectibles: get_last_collectible(context, safe_address).unwrap_or(0),
            tx_queued: get_last_queued_tx(context, safe_address).unwrap_or(0),
            tx_history: get_last_history_tx(context, safe_address).unwrap_or(0),
        },
    };

    Ok(safe_state)
}

fn get_last_collectible(context: &Context, safe_address: &String) -> ApiResult<i64> {
    let url = format!(
        "{}/v1/safes/{}/transfers/?\
        &ordering=executionDate\
        &limit=1",
        base_transaction_service_url(),
        safe_address,
    );

    let body = RequestCached::new(url)
        .request_timeout(transaction_request_timeout())
        .execute(context.client(), context.cache())?;
    let transaction: Page<Transfer> = serde_json::from_str(&body)?;

    log::debug!("{:#?}", &transaction);
    transaction
        .results
        .get(0)
        .as_ref()
        .map(|transfer| match transfer {
            Transfer::Erc721(transfer) => transfer.execution_date.timestamp(),
            Transfer::Erc20(transfer) => transfer.execution_date.timestamp(),
            Transfer::Ether(transfer) => transfer.execution_date.timestamp(),
            Transfer::Unknown => 0,
        })
        .ok_or(api_error!("Couldn't get tx timestamps"))
}

fn get_last_queued_tx(context: &Context, safe_address: &String) -> ApiResult<i64> {
    let url = format!(
        "{}/v1/safes/{}/multisig-transactions/?\
        &ordering=nonce,submissionDate\
        &executed=false\
        &trusted=true\
        &limit=1",
        base_transaction_service_url(),
        safe_address,
    );

    let body = RequestCached::new(url)
        .request_timeout(transaction_request_timeout())
        .execute(context.client(), context.cache())?;
    let transaction: Page<MultisigTransaction> = serde_json::from_str(&body)?;

    log::debug!("{:#?}", &transaction);
    transaction
        .results
        .get(0)
        .as_ref()
        .map(|tx| tx.submission_date.timestamp())
        .ok_or(api_error!("Couldn't get tx timestamps"))
}

fn get_last_history_tx(context: &Context, safe_address: &String) -> ApiResult<i64> {
    let mut info_provider = DefaultInfoProvider::new(context);

    let url = format!(
        "{}/v1/safes/{}/all-transactions/?\
        &ordering=executionDate
        &queued=false\
        &executed=true",
        base_transaction_service_url(),
        safe_address
    );

    let body = RequestCached::new(url)
        .request_timeout(transaction_request_timeout())
        .execute(context.client(), context.cache())?;
    let transaction: Page<Transaction> = serde_json::from_str(&body)?;

    log::debug!("{:#?}", &transaction);
    transaction
        .results
        .get(0)
        .as_ref()
        .and_then(|tx| {
            tx.to_transaction_summary(&mut info_provider, safe_address)
                .ok()
                .as_ref()
                .map(|txs| txs.get(0))
                .flatten()
                .as_ref()
                .map(|tx| tx.timestamp / 1000)
        })
        .ok_or(api_error!("Couldn't get tx timestamps"))
}
