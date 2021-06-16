extern crate reqwest;

use crate::cache::cache_operations::RequestCached;
use crate::config::transaction_request_timeout;
use crate::models::backend::transactions::{ModuleTransaction, MultisigTransaction};
use crate::models::backend::transfers::Transfer;
use crate::models::commons::Page;
use crate::models::service::transactions::details::TransactionDetails;
use crate::models::service::transactions::{
    TransactionIdParts, ID_PREFIX_CREATION_TX, ID_PREFIX_ETHEREUM_TX, ID_PREFIX_MODULE_TX,
    ID_PREFIX_MULTISIG_TX, ID_SEPARATOR,
};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use crate::utils::hex_hash;
use crate::utils::transactions::fetch_rejections;
use log::debug;

pub(super) async fn get_multisig_transaction_details(
    context: &Context<'_>,
    chain_id: &str,
    safe_tx_hash: &str,
) -> ApiResult<TransactionDetails> {
    let mut info_provider = DefaultInfoProvider::new(chain_id, context);
    let url = core_uri!(info_provider, "/v1/multisig-transactions/{}/", safe_tx_hash)?;
    let body = RequestCached::new(url)
        .request_timeout(transaction_request_timeout())
        .execute(context.client(), context.cache())
        .await?;
    let multisig_tx: MultisigTransaction = serde_json::from_str(&body)?;

    let rejections = fetch_rejections(
        context,
        chain_id,
        &multisig_tx.safe_transaction.safe,
        multisig_tx.nonce,
    )
    .await;

    let details = multisig_tx
        .to_transaction_details(rejections, &mut info_provider)
        .await?;

    Ok(details)
}

async fn get_ethereum_transaction_details(
    context: &Context<'_>,
    chain_id: &str,
    safe: &str,
    tx_hash: &str,
    detail_hash: &str,
) -> ApiResult<TransactionDetails> {
    let mut info_provider = DefaultInfoProvider::new(chain_id, context);
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/transfers/?transaction_hash={}&limit=1000",
        safe,
        tx_hash
    )?;
    debug!("url: {}", url);
    let body = RequestCached::new(url)
        .request_timeout(transaction_request_timeout())
        .execute(context.client(), context.cache())
        .await?;
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
    let details = transfer
        .to_transaction_details(&mut info_provider, &safe.to_owned())
        .await?;

    Ok(details)
}

async fn get_module_transaction_details(
    context: &Context<'_>,
    chain_id: &str,
    safe_address: &str,
    safe_tx_hash: &str,
    detail_hash: &str,
) -> ApiResult<TransactionDetails> {
    let mut info_provider = DefaultInfoProvider::new(chain_id, context);

    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/module-transactions/?transaction_hash={}&limit=1000",
        safe_address,
        safe_tx_hash
    )?;

    debug!("url: {}", url);
    let body = RequestCached::new(url)
        .request_timeout(transaction_request_timeout())
        .execute(context.client(), context.cache())
        .await?;
    let transactions: Page<ModuleTransaction> = serde_json::from_str(&body)?;
    let transaction = transactions
        .results
        .into_iter()
        .find(|tx| hex_hash(tx) == detail_hash)
        .ok_or(api_error!("No transfer found"))?;
    let details = transaction
        .to_transaction_details(&mut info_provider)
        .await?;

    Ok(details)
}

pub async fn get_transactions_details(
    context: &Context<'_>,
    chain_id: &str,
    details_id: &String,
) -> ApiResult<TransactionDetails> {
    let id_parts = parse_id(details_id)?;

    match id_parts {
        TransactionIdParts::Ethereum {
            safe_address,
            transaction_hash,
            details_hash,
        } => {
            get_ethereum_transaction_details(
                context,
                &chain_id,
                &safe_address,
                &transaction_hash,
                &details_hash,
            )
            .await
        }
        TransactionIdParts::Module {
            safe_address,
            transaction_hash,
            details_hash,
        } => {
            get_module_transaction_details(
                context,
                chain_id,
                &safe_address,
                &transaction_hash,
                &details_hash,
            )
            .await
        }
        TransactionIdParts::Multisig { safe_tx_hash, .. } => {
            get_multisig_transaction_details(context, chain_id, &safe_tx_hash).await
        }
        TransactionIdParts::TransactionHash(safe_tx_hash) => {
            get_multisig_transaction_details(context, chain_id, &safe_tx_hash).await
        }
        _ => Err(client_error!(422, "Bad transaction id")),
    }
}

pub(super) fn parse_id(details_id: &str) -> ApiResult<TransactionIdParts> {
    let id_parts: Vec<&str> = details_id.split(ID_SEPARATOR).collect();
    let tx_type = id_parts.get(0).ok_or(api_error!("Invalid id"))?;

    Ok(match tx_type.to_owned() {
        ID_PREFIX_MULTISIG_TX => TransactionIdParts::Multisig {
            safe_address: id_parts
                .get(1)
                .ok_or(client_error!(422, "No safe address provided"))?
                .to_string(),
            safe_tx_hash: id_parts
                .get(2)
                .ok_or(client_error!(422, "No safe tx hash provided"))?
                .to_string(),
        },
        ID_PREFIX_ETHEREUM_TX => TransactionIdParts::Ethereum {
            safe_address: id_parts
                .get(1)
                .ok_or(client_error!(422, "No safe address"))?
                .to_string(),
            transaction_hash: id_parts
                .get(2)
                .ok_or(client_error!(422, "No ethereum tx hash"))?
                .to_string(),
            details_hash: id_parts
                .get(3)
                .ok_or(client_error!(422, "No ethereum tx details hash"))?
                .to_string(),
        },
        ID_PREFIX_MODULE_TX => TransactionIdParts::Module {
            safe_address: id_parts
                .get(1)
                .ok_or(client_error!(422, "No safe address"))?
                .to_string(),
            transaction_hash: id_parts
                .get(2)
                .ok_or(client_error!(422, "No module tx hash"))?
                .to_string(),
            details_hash: id_parts
                .get(3)
                .ok_or(client_error!(422, "No module tx details hash"))?
                .to_string(),
        },
        ID_PREFIX_CREATION_TX => TransactionIdParts::Creation(
            id_parts
                .get(1)
                .ok_or(client_error!(422, "No safe address provided"))?
                .to_string(),
        ),
        &_ => TransactionIdParts::TransactionHash(tx_type.to_string()),
    })
}
