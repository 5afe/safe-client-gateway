use crate::{
    common::models::page::Page,
    routes::transactions::models::{filters::ModuleFilters, summary::TransactionListItem},
    utils::{context::RequestContext, errors::ApiResult},
};

pub async fn get_module_transactions(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    cursor: &Option<String>,
    filters: &ModuleFilters,
) -> ApiResult<Page<TransactionListItem>> {
    return Ok(Page {
        next: None,
        previous: None,
        results: vec![],
    });
}
