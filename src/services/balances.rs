use crate::cache::cache_operations::RequestCached;
use crate::config::{balances_cache_duration, base_transaction_service_url};
use crate::models::backend::balances::Balance as BalanceDto;
use crate::models::service::balances::{Balance, Balances};
use crate::providers::info::DefaultInfoProvider;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use std::cmp::Ordering;

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

    let body = RequestCached::new()
        .url(url)
        .cache_duration(balances_cache_duration())
        .execute(context.client(), context.cache())?;
    let backend_balances: Vec<BalanceDto> = serde_json::from_str(&body)?;

    let info_provider = DefaultInfoProvider::new(&context);
    let usd_to_fiat = info_provider.exchange_usd_to(fiat).unwrap_or(0.0);

    let mut total_fiat = 0.0;

    let mut service_balances: Vec<Balance> = backend_balances
        .into_iter()
        .map(|it| {
            let balance = it.to_balance(usd_to_fiat);
            total_fiat += balance.fiat_balance.parse::<f64>().unwrap_or(0.0);
            balance
        })
        .collect();

    service_balances.sort_by(|a, b| {
        b.fiat_balance
            .parse::<f64>()
            .unwrap_or(0.0)
            .partial_cmp(&a.fiat_balance.parse::<f64>().unwrap_or(0.0))
            .unwrap_or(Ordering::Equal)
    });
    Ok(Balances {
        fiat_total: total_fiat.to_string(),
        items: service_balances,
    })
}

pub fn fiat_codes(context: &Context) -> ApiResult<Vec<String>> {
    let info_provider = DefaultInfoProvider::new(&context);
    let mut fiat_codes = info_provider.available_currency_codes()?;

    let usd_index = fiat_codes.iter().position(|it| it.eq("USD")).unwrap();
    let eur_index = fiat_codes.iter().position(|it| it.eq("EUR")).unwrap();

    let usd_code = fiat_codes.swap_remove(usd_index);
    let eur_code = fiat_codes.swap_remove(eur_index);

    fiat_codes.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    let mut output = vec![usd_code, eur_code];
    output.append(&mut fiat_codes);

    Ok(output)
}
