use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::models::backend::transactions::Transaction;
use crate::models::commons::Page;
use crate::models::service::transactions::summary::TransactionListItem;
use crate::providers::info::DefaultInfoProvider;
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use log::debug;

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
