use crate::cache::cache_operations::RequestCached;
use crate::cache::manager::ChainCache;
use crate::common::models::backend::transactions::{MultisigTransaction, Transaction};
use crate::common::models::backend::transfers::Transfer;
use crate::common::models::page::{Page, SafeList};
use crate::config::{
    is_messages_feature_enabled, owners_for_safes_cache_duration, transaction_request_timeout,
};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::messages::backend_models::Message;
use crate::routes::safes::models::{SafeLastChanges, SafeState};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::Request;
use chrono::Utc;
use rocket::futures::join;

// We use Utc::now().timestamp() as the fallback value so that we don't block clients from reloading
// as returning always 0, and the clients invalidating on value changes, would prevent reloading
pub async fn get_safe_info_ex(
    context: &RequestContext,
    chain_id: &String,
    safe_address: &String,
) -> ApiResult<SafeState> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);
    let safe_info = info_provider.safe_info(safe_address).await?;
    // We want to be able to return the rest of `SafeInfo` in case the `about/master-copies` endpoint is not available
    let supported_master_copies = info_provider.master_copies().await.unwrap_or(vec![]);
    let safe_info_ex = safe_info
        .to_safe_info_ex(&info_provider, supported_master_copies)
        .await;

    let (collectibles_tag, tx_queued_tag, tx_history_tag) = join!(
        get_last_collectible(&info_provider, safe_address),
        get_last_queued_tx(&info_provider, safe_address),
        get_last_history_tx(&info_provider, safe_address)
    );

    // This branch won't be required once the Messages feature is considered stable
    // SafeLastChanges.messages_tag can also be safely converted to a non-optional field
    let messages_tag: Option<String> = if is_messages_feature_enabled() {
        Some(
            get_last_modified_message(&info_provider, safe_address)
                .await
                .unwrap_or(Utc::now().timestamp())
                .to_string(),
        )
    } else {
        None
    };

    let safe_state = SafeState {
        safe_config: safe_info_ex,
        safe_state: SafeLastChanges {
            collectibles_tag: collectibles_tag
                .unwrap_or(Utc::now().timestamp())
                .to_string(),
            tx_queued_tag: tx_queued_tag.unwrap_or(Utc::now().timestamp()).to_string(),
            tx_history_tag: tx_history_tag.unwrap_or(Utc::now().timestamp()).to_string(),
            messages_tag,
        },
    };

    Ok(safe_state)
}

async fn get_last_modified_message(
    info_provider: &impl InfoProvider,
    safe_address: &String,
) -> ApiResult<i64> {
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/messages/?ordering=-modified&limit=1",
        safe_address
    )?;
    let http_request = Request::new(url);
    let body = info_provider.client().get(http_request).await?.body;
    let messages_page: Page<Message> = serde_json::from_str::<Page<Message>>(&body)?;

    return messages_page
        .results
        .get(0)
        .map(|message| message.modified.timestamp())
        .ok_or(api_error!("Couldn't get tx timestamps"));
}

async fn get_last_collectible(
    info_provider: &impl InfoProvider,
    safe_address: &String,
) -> ApiResult<i64> {
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/transfers/?\
        &erc721=true\
        &limit=1",
        safe_address,
    )?;

    let body = RequestCached::new(url, &info_provider.client(), &info_provider.cache())
        .request_timeout(transaction_request_timeout())
        .execute()
        .await?;
    let transaction: Page<Transfer> = serde_json::from_str(&body)?;

    transaction
        .results
        .get(0)
        .as_ref()
        .map(|transfer| match transfer {
            Transfer::Erc721(transfer) => transfer.execution_date.timestamp(),
            Transfer::Erc20(transfer) => transfer.execution_date.timestamp(),
            Transfer::Ether(transfer) => transfer.execution_date.timestamp(),
            Transfer::Unknown => Utc::now().timestamp(),
        })
        .ok_or(api_error!("Couldn't get tx timestamps"))
}

async fn get_last_queued_tx(
    info_provider: &impl InfoProvider,
    safe_address: &String,
) -> ApiResult<i64> {
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/multisig-transactions/?\
        &ordering=-modified\
        &executed=false\
        &trusted=true\
        &limit=1",
        safe_address,
    )?;

    let body = RequestCached::new(url, &info_provider.client(), &info_provider.cache())
        .request_timeout(transaction_request_timeout())
        .execute()
        .await?;
    let transaction: Page<MultisigTransaction> = serde_json::from_str(&body)?;

    transaction
        .results
        .get(0)
        .as_ref()
        .map(|tx| tx.modified.as_ref().map(|it| it.timestamp()))
        .flatten()
        .ok_or(api_error!("Couldn't get tx timestamps"))
}

async fn get_last_history_tx(
    info_provider: &impl InfoProvider,
    safe_address: &String,
) -> ApiResult<i64> {
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/all-transactions/?\
        &ordering=executionDate
        &queued=false\
        &executed=true",
        safe_address
    )?;

    let body = RequestCached::new(url, &info_provider.client(), &info_provider.cache())
        .request_timeout(transaction_request_timeout())
        .execute()
        .await?;
    let transaction: Page<Transaction> = serde_json::from_str(&body)?;

    transaction
        .results
        .get(0)
        .as_ref()
        .map(|tx| match tx {
            Transaction::Multisig(tx) => tx
                .modified
                .as_ref()
                .map(|it| it.timestamp())
                .unwrap_or(tx.submission_date.timestamp()),
            Transaction::Ethereum(tx) => tx.execution_date.timestamp(),
            Transaction::Module(tx) => tx.execution_date.timestamp(),
            Transaction::Unknown => Utc::now().timestamp(),
        })
        .ok_or(api_error!("Couldn't get tx timestamps"))
}

pub async fn get_owners_for_safe(
    context: &RequestContext,
    chain_id: &str,
    owner_address: &str,
) -> ApiResult<SafeList> {
    let info_provider = DefaultInfoProvider::new(&chain_id, context);

    let url = core_uri!(info_provider, "/v1/owners/{}/safes/", owner_address)?;
    let body = RequestCached::new_from_context(url, context, ChainCache::from(chain_id))
        .cache_duration(owners_for_safes_cache_duration())
        .execute()
        .await?;

    Ok(serde_json::from_str(&body)?)
}
