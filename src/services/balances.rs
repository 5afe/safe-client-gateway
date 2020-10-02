use crate::models::service::balances::Balance;
use crate::utils::errors::ApiResult;
use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::utils::context::Context;
use crate::utils::cache::CacheExt;
use crate::models::backend::balances::Balance as BalanceDto;
use crate::providers::info::DefaultInfoProvider;
use log::debug;

pub fn balances(context: &Context, safe_address: &str, fiat: &str) -> ApiResult<Vec<Balance>> {
    let url = format!(
        "{}/v1/safes/{}/balances/usd/",
        base_transaction_service_url(),
        safe_address,
    );

    let body = context.cache().request_cached(&context.client(), &url, request_cache_duration())?;
    let backend_balances: Vec<BalanceDto> = serde_json::from_str(&body)?;

    let info_provider = DefaultInfoProvider::new(&context);
    let usd_to_fiat = info_provider.exchange_usd_to(fiat)?;

    debug!("usd_to_fiat: {}", usd_to_fiat);

    let service_balances: Vec<Balance> = backend_balances.into_iter().map(|it| it.to_balance(usd_to_fiat)).collect();

    Ok(service_balances)
}
