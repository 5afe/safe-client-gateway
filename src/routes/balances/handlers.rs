use crate::cache::cache_operations::RequestCached;
use crate::cache::manager::ChainCache;
use crate::common::models::backend::balances::Balance as BalanceDto;
use crate::common::models::backend::chains::NativeCurrency;
use crate::config::{balances_cache_duration, balances_request_timeout};
use crate::providers::fiat::FiatInfoProvider;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::balances::models::{Balance, Balances};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use bigdecimal::{BigDecimal, ToPrimitive};
use std::cmp::Ordering;

pub async fn balances(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    fiat: &str,
    trusted: bool,
    exclude_spam: bool,
) -> ApiResult<Balances> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);
    let fiat_info_provider = FiatInfoProvider::new(&context);
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/balances/usd/?trusted={}&exclude_spam={}",
        safe_address,
        trusted,
        exclude_spam
    )?;

    let body = RequestCached::new_from_context(url, context, ChainCache::from(chain_id))
        .cache_duration(balances_cache_duration())
        .request_timeout(balances_request_timeout())
        .execute()
        .await?;
    let backend_balances: Vec<BalanceDto> = serde_json::from_str(&body)?;

    let usd_to_fiat = fiat_info_provider
        .exchange_usd_to(fiat)
        .await
        .unwrap_or(BigDecimal::from(0))
        .to_f64()
        .unwrap_or(f64::from(0));

    let native_currency: NativeCurrency = info_provider.chain_info().await?.native_currency;

    let mut total_fiat = 0.0;

    let mut service_balances: Vec<Balance> = backend_balances
        .into_iter()
        .map(|it| {
            let balance = it.to_balance(usd_to_fiat, &native_currency);
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

pub async fn fiat_codes(context: &RequestContext) -> ApiResult<Vec<String>> {
    let info_provider = FiatInfoProvider::new(&context);
    let mut fiat_codes = info_provider.available_currency_codes().await?;

    let usd_index = fiat_codes.iter().position(|it| it.eq("USD")).unwrap();
    let eur_index = fiat_codes.iter().position(|it| it.eq("EUR")).unwrap();

    let usd_code = fiat_codes.swap_remove(usd_index);
    let eur_code = fiat_codes.swap_remove(eur_index);

    fiat_codes.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    let mut output = vec![usd_code, eur_code];
    output.append(&mut fiat_codes);

    Ok(output)
}
