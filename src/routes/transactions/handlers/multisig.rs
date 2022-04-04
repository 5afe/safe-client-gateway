use crate::{
    common::models::page::Page,
    routes::transactions::models::{filters::MultisigFilters, summary::TransactionListItem},
    utils::{context::RequestContext, errors::ApiResult},
};

pub async fn get_multisig_transactions(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    cursor: &Option<String>,
    filters: &MultisigFilters,
) -> ApiResult<Page<TransactionListItem>> {
    return Ok(Page {
        next: None,
        previous: None,
        results: vec![],
    });
}
