use crate::config::{
    balances_cache_duration, base_transaction_service_url, request_error_cache_timeout,
};
use crate::models::backend::balances::Balance as BalanceDto;
use crate::models::service::balances::{Balance, Balances};
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
) -> ApiResult<Balances> {
    let url = format!(
        "{}/v1/safes/{}/balances/usd/?trusted={}&exclude_spam={}",
        base_transaction_service_url(),
        safe_address,
        trusted,
        exclude_spam
    );

    let body = context.cache().request_cached(
        &context.client(),
        &url,
        balances_cache_duration(),
        request_error_cache_timeout(),
    )?;
    let backend_balances: Vec<BalanceDto> = serde_json::from_str(&body)?;

    let info_provider = DefaultInfoProvider::new(&context);
    let usd_to_fiat = info_provider.exchange_usd_to(fiat)?;

    let mut total_fiat = 0.0;

    let service_balances: Vec<Balance> = backend_balances
        .into_iter()
        .map(|it| {
            let balance = it.to_balance(usd_to_fiat);
            total_fiat += balance.fiat_balance.parse::<f64>().unwrap_or(0.0);
            balance
        })
        .collect();

    Ok(Balances {
        fiat_total: total_fiat.to_string(),
        items: service_balances,
    })
}
