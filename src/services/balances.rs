use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::models::backend::balances::Balance as BalanceDto;
use crate::models::service::balances::Balance;
use crate::providers::info::DefaultInfoProvider;
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;

pub fn balances(
    context: &Context,
    safe_address: &str,
    fiat: &str,
    trusted: bool,
    exclude_spam: bool,
) -> ApiResult<Vec<Balance>> {
    let url = format!(
        "{}/v1/safes/{}/balances/usd/?trusted={}&exclude_spam={}",
        base_transaction_service_url(),
        safe_address,
        trusted,
        exclude_spam
    );

    let body = context
        .cache()
        .request_cached(&context.client(), &url, request_cache_duration())?;
    let backend_balances: Vec<BalanceDto> = serde_json::from_str(&body)?;

    let info_provider = DefaultInfoProvider::new(&context);
    let usd_to_fiat = info_provider.exchange_usd_to(fiat)?;

    let service_balances: Vec<Balance> = backend_balances
        .into_iter()
        .map(|it| it.to_balance(usd_to_fiat))
        .collect();

    Ok(service_balances)
}
