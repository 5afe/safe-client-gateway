extern crate reqwest;

use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::models::backend::transactions::{ModuleTransaction, MultisigTransaction};
use crate::models::backend::transfers::Transfer;
use crate::models::commons::Page;
use crate::models::service::transactions::details::TransactionDetails;
use crate::models::service::transactions::{
    ID_PREFIX_ETHEREUM_TX, ID_PREFIX_MODULE_TX, ID_PREFIX_MULTISIG_TX, ID_SEPARATOR,
};
use crate::providers::info::DefaultInfoProvider;
use crate::utils::context::Context;
use crate::utils::hex_hash;
use crate::utils::cache::CacheExt;
use log::debug;
use crate::utils::errors::ApiResult;

fn get_multisig_transaction_details(
    context: &Context,
    safe_tx_hash: &str,
) -> ApiResult<TransactionDetails> {
    let mut info_provider = DefaultInfoProvider::new(context);
    let url = format!(
        "{}/v1/transactions/{}",
        base_transaction_service_url(),
        safe_tx_hash
    );
    let body = context
        .cache()
        .request_cached(&context.client(), &url, request_cache_duration())?;
    debug!("{:#?}", body);
    let multisig_tx: MultisigTransaction = serde_json::from_str(&body)?;
    let details = multisig_tx.to_transaction_details(&mut info_provider)?;

    Ok(details)
}

fn get_ethereum_transaction_details(
    context: &Context,
    safe: &str,
    tx_hash: &str,
    detail_hash: &str,
) -> ApiResult<TransactionDetails> {
    let mut info_provider = DefaultInfoProvider::new(context);
    let url = format!(
        "{}/v1/safes/{}/transfers/?transaction_hash={}&limit=1000",
        base_transaction_service_url(),
        safe,
        tx_hash
    );
    debug!("url: {}", url);
    let body = context
        .cache()
        .request_cached(&context.client(), &url, request_cache_duration())?;
    debug!("{:#?}", body);
    let transfers: Page<Transfer> = serde_json::from_str(&body)?;
    let transfer = transfers
        .results
        .into_iter()
        .find(|transfer| {
            debug!("expected: {}", detail_hash);
            debug!("actual: {}", hex_hash(transfer));
            hex_hash(transfer) == detail_hash
        })
        .ok_or(anyhow::anyhow!("No transfer found"))?;
    let details = transfer.to_transaction_details(&mut info_provider, &safe.to_owned())?;

    Ok(details)
}

fn get_module_transaction_details(
    context: &Context,
    safe: &str,
    tx_hash: &str,
    detail_hash: &str,
) -> ApiResult<TransactionDetails> {
    let url = format!(
        "{}/v1/safes/{}/module-transactions/?transaction_hash={}&limit=1000",
        base_transaction_service_url(),
        safe,
        tx_hash
    );
    debug!("url: {}", url);
    let body = context
        .cache()
        .request_cached(&context.client(), &url, request_cache_duration())?;
    let transactions: Page<ModuleTransaction> = serde_json::from_str(&body)?;
    let transaction = transactions
        .results
        .into_iter()
        .find(|tx| hex_hash(tx) == detail_hash)
        .ok_or(anyhow::anyhow!("No transfer found"))?;
    let details = transaction.to_transaction_details()?;

    Ok(details)
}

pub fn get_transactions_details(
    context: &Context,
    details_id: &String,
) -> ApiResult<TransactionDetails> {
    let id_parts: Vec<&str> = details_id.split(ID_SEPARATOR).collect();
    let tx_type = id_parts.get(0).ok_or(anyhow::anyhow!("Invalid id"))?;

    match tx_type.to_owned() {
        ID_PREFIX_MULTISIG_TX => get_multisig_transaction_details(
            context,
            id_parts
                .get(2)
                .ok_or(anyhow::anyhow!("No safe tx hash provided"))?,
        ),
        ID_PREFIX_ETHEREUM_TX => get_ethereum_transaction_details(
            context,
            id_parts.get(1).ok_or(anyhow::anyhow!("No safe address"))?,
            id_parts
                .get(2)
                .ok_or(anyhow::anyhow!("No module tx hash"))?,
            id_parts
                .get(3)
                .ok_or(anyhow::anyhow!("No module tx details hash"))?,
        ),
        ID_PREFIX_MODULE_TX => get_module_transaction_details(
            context,
            id_parts.get(1).ok_or(anyhow::anyhow!("No safe address"))?,
            id_parts
                .get(2)
                .ok_or(anyhow::anyhow!("No module tx hash"))?,
            id_parts
                .get(3)
                .ok_or(anyhow::anyhow!("No module tx details hash"))?,
        ),
        &_ => get_multisig_transaction_details(context, tx_type),
    }
}
