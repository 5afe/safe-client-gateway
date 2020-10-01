use crate::models::service::balances::{Balance, Fiat};
use crate::utils::errors::ApiResult;
use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::utils::context::Context;
use crate::utils::cache::CacheExt;
use crate::models::backend::balances::Balance as BalanceDto;
use log::debug;

pub fn balances(context: &Context, safe_address: &str, fiat: &Fiat) -> ApiResult<Vec<Balance>> {
    match fiat {
        Fiat::Usd(_) => balances_usd(context, safe_address),
        Fiat::Eur(conversion_rate) => balances_eur(context, safe_address, conversion_rate),
        _ => Ok(vec![])
    }
}

pub fn balances_eur(context: &Context, safe_address: &str, conversion_rate: &f64) -> ApiResult<Vec<Balance>> {
    let url = format!(
        "{}/v1/safes/{}/balances/usd/",
        base_transaction_service_url(),
        safe_address,
    );

    let body = context.cache().request_cached(&context.client(), &url, request_cache_duration())?;
    let backend_balances: Vec<BalanceDto> = serde_json::from_str(&body)?;
    let service_balances: Vec<Balance> = backend_balances.into_iter().map(|it| it.to_balance(conversion_rate)).collect();

    Ok(service_balances)
}

pub fn balances_usd(context: &Context, safe_address: &str) -> ApiResult<Vec<Balance>> {
    let url = format!(
        "{}/v1/safes/{}/balances/usd/",
        base_transaction_service_url(),
        safe_address,
    );

    let body = context.cache().request_cached(&context.client(), &url, request_cache_duration())?;
    debug!("request URL: {}", &url);
    debug!("safe_address: {:#?}", &safe_address);
    debug!("{:#?}", body);

    let backend_balances: Vec<BalanceDto> = serde_json::from_str(&body)?;
    let service_balances: Vec<Balance> = backend_balances.into_iter().map(|it| it.to_balance(&1.0)).collect();

    Ok(service_balances)
}