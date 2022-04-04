use crate::{
    common::models::page::Page,
    routes::transactions::models::{filters::TransferFilters, summary::TransactionListItem},
    utils::{context::RequestContext, errors::ApiResult},
};

pub async fn get_incoming_transfers(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    cursor: &Option<String>,
    filters: &TransferFilters,
) -> ApiResult<Page<TransactionListItem>> {
    return Ok(Page {
        next: "",
        previous: None,
        results: vec![],
    });
}
