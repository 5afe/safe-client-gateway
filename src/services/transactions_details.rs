extern crate reqwest;

use crate::config::{
    base_transaction_service_url, request_cache_duration, request_error_cache_timeout,
};
use crate::models::backend::transactions::{ModuleTransaction, MultisigTransaction};
use crate::models::backend::transfers::Transfer;
use crate::models::commons::Page;
use crate::models::service::transactions::details::TransactionDetails;
use crate::models::service::transactions::{
    TransactionIdParts, ID_PREFIX_CREATION_TX, ID_PREFIX_ETHEREUM_TX, ID_PREFIX_MODULE_TX,
    ID_PREFIX_MULTISIG_TX, ID_SEPARATOR,
};
use crate::providers::info::DefaultInfoProvider;
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use crate::utils::errors::{ApiError, ApiResult};
use crate::utils::hex_hash;
use log::debug;

pub(super) fn get_multisig_transaction_details(
    context: &Context,
    safe_tx_hash: &str,
) -> ApiResult<TransactionDetails> {
    let mut info_provider = DefaultInfoProvider::new(context);
    let url = format!(
        "{}/v1/multisig-transactions/{}/",
        base_transaction_service_url(),
        safe_tx_hash
    );
    let body = context.cache().request_cached(
        &context.client(),
        &url,
        request_cache_duration(),
        request_error_cache_timeout(),
    )?;
    let multisig_tx: MultisigTransaction = serde_json::from_str(&body)?;

    let conflicting_txs = get_conflicting_txs(&context, &multisig_tx).unwrap_or(vec![]);

    let details = multisig_tx.to_transaction_details(conflicting_txs, &mut info_provider)?;

    Ok(details)
}

fn get_conflicting_txs(
    context: &Context,
    multisig_tx: &MultisigTransaction,
) -> ApiResult<Vec<MultisigTransaction>> {
    let url = format!(
        "{}/v1/safes/{}/multisig-transactions/?nonce={}",
        base_transaction_service_url(),
        &multisig_tx.safe,
        &multisig_tx.nonce
    );
    debug!("{:#?}", &url);

    let body = context.cache().request_cached(
        &context.client(),
        &url,
        request_cache_duration(),
        request_error_cache_timeout(),
    )?;
    let backend_transactions: Page<MultisigTransaction> = serde_json::from_str(&body)?;
    Ok(backend_transactions.results)
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
    let body = context.cache().request_cached(
        &context.client(),
        &url,
        request_cache_duration(),
        request_error_cache_timeout(),
    )?;
    let transfers: Page<Transfer> = serde_json::from_str(&body)?;
    let transfer = transfers
        .results
        .into_iter()
        .find(|transfer| {
            debug!("expected: {}", detail_hash);
            debug!("actual: {}", hex_hash(transfer));
            hex_hash(transfer) == detail_hash
        })
        .ok_or(api_error!("No transfer found"))?;
    let details = transfer.to_transaction_details(&mut info_provider, &safe.to_owned())?;

    Ok(details)
}

fn get_module_transaction_details(
    context: &Context,
    safe: &str,
    tx_hash: &str,
    detail_hash: &str,
) -> ApiResult<TransactionDetails> {
    let mut info_provider = DefaultInfoProvider::new(context);

    let url = format!(
        "{}/v1/safes/{}/module-transactions/?transaction_hash={}&limit=1000",
        base_transaction_service_url(),
        safe,
        tx_hash
    );
    debug!("url: {}", url);
    let body = context.cache().request_cached(
        &context.client(),
        &url,
        request_cache_duration(),
        request_error_cache_timeout(),
    )?;
    let transactions: Page<ModuleTransaction> = serde_json::from_str(&body)?;
    let transaction = transactions
        .results
        .into_iter()
        .find(|tx| hex_hash(tx) == detail_hash)
        .ok_or(api_error!("No transfer found"))?;
    let details = transaction.to_transaction_details(&mut info_provider)?;

    Ok(details)
}

pub fn get_transactions_details(
    context: &Context,
    details_id: &String,
) -> ApiResult<TransactionDetails> {
    let id_parts = parse_id(details_id)?;

    match id_parts {
        TransactionIdParts::Ethereum {
            safe_address,
            transaction_hash,
            details_hash,
        } => get_ethereum_transaction_details(
            context,
            &safe_address,
            &transaction_hash,
            &details_hash,
        ),
        TransactionIdParts::Module {
            safe_address,
            transaction_hash,
            details_hash,
        } => {
            get_module_transaction_details(context, &safe_address, &transaction_hash, &details_hash)
        }
        TransactionIdParts::Multisig { safe_tx_hash, .. } => {
            get_multisig_transaction_details(context, &safe_tx_hash)
        }
        TransactionIdParts::TransactionHash(safe_tx_hash) => {
            get_multisig_transaction_details(context, &safe_tx_hash)
        }
        _ => Err(ApiError::new_from_message_with_code(
            422,
            String::from("Bad transaction id"),
        )),
    }
}

pub(super) fn parse_id(details_id: &str) -> ApiResult<TransactionIdParts> {
    let id_parts: Vec<&str> = details_id.split(ID_SEPARATOR).collect();
    let tx_type = id_parts.get(0).ok_or(api_error!("Invalid id"))?;

    Ok(match tx_type.to_owned() {
        ID_PREFIX_MULTISIG_TX => TransactionIdParts::Multisig {
            safe_address: id_parts
                .get(1)
                .ok_or(api_error!("No safe address provided"))?
                .to_string(),
            safe_tx_hash: id_parts
                .get(2)
                .ok_or(api_error!("No safe tx hash provided"))?
                .to_string(),
        },
        ID_PREFIX_ETHEREUM_TX => TransactionIdParts::Ethereum {
            safe_address: id_parts
                .get(1)
                .ok_or(api_error!("No safe address"))?
                .to_string(),
            transaction_hash: id_parts
                .get(2)
                .ok_or(api_error!("No ethereum tx hash"))?
                .to_string(),
            details_hash: id_parts
                .get(3)
                .ok_or(api_error!("No ethereum tx details hash"))?
                .to_string(),
        },
        ID_PREFIX_MODULE_TX => TransactionIdParts::Module {
            safe_address: id_parts
                .get(1)
                .ok_or(api_error!("No safe address"))?
                .to_string(),
            transaction_hash: id_parts
                .get(2)
                .ok_or(api_error!("No module tx hash"))?
                .to_string(),
            details_hash: id_parts
                .get(3)
                .ok_or(api_error!("No module tx details hash"))?
                .to_string(),
        },
        ID_PREFIX_CREATION_TX => TransactionIdParts::Creation(
            id_parts
                .get(1)
                .ok_or(api_error!("No safe address provided"))?
                .to_string(),
        ),
        &_ => TransactionIdParts::TransactionHash(tx_type.to_string()),
    })
}
