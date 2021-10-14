use crate::cache::cache_operations::RequestCached;
use crate::config::{balances_cache_duration, balances_request_timeout};
use crate::models::backend::balances::Balance as BalanceDto;
use crate::models::backend::chains::NativeCurrency;
use crate::models::backend::tokens::TokenPriceCore;
use crate::models::service::balances::{Balance, Balances};
use crate::models::service::tokens::TokenPrice;
use crate::providers::fiat::FiatInfoProvider;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use bigdecimal::BigDecimal;
use rocket::futures::{stream, StreamExt};

const N_CONCURRENT_TOKEN_REQUESTS: usize = 5;

pub async fn balances(
    context: &Context<'_>,
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

    let body = RequestCached::new(url)
        .cache_duration(balances_cache_duration())
        .request_timeout(balances_request_timeout())
        .execute(context.client(), context.cache())
        .await?;
    let backend_balances: Vec<BalanceDto> = serde_json::from_str(&body)?;

    let usd_to_fiat = fiat_info_provider
        .exchange_usd_to(fiat)
        .await
        .unwrap_or(BigDecimal::from(0));

    let native_currency: NativeCurrency = info_provider.chain_info().await?.native_currency;

    let mut total_fiat = 0.0;

    let token_prices: Vec<TokenPrice> =
        get_token_prices(&context, &info_provider, &backend_balances).await;

    let service_balances: Vec<Balance> = backend_balances
        .iter()
        .map(|it| {
            let token_address: String = it
                .token_address
                .to_owned()
                .unwrap_or("0x0000000000000000000000000000000000000000".to_string());
            let token_price: Option<&TokenPrice> = token_prices
                .iter()
                .find(|&token_price| token_price.address == token_address);
            let token_to_usd: BigDecimal = token_price
                .and_then(|t| Some(t.fiat_price.to_owned()))
                .unwrap_or(BigDecimal::from(0));

            let balance = it.to_balance(&token_to_usd, &usd_to_fiat, &native_currency);
            total_fiat += balance.fiat_balance.parse::<f64>().unwrap_or(0.0);
            balance
        })
        .collect();

    Ok(Balances {
        fiat_total: total_fiat.to_string(),
        items: service_balances,
    })
}

async fn get_token_prices(
    context: &Context<'_>,
    info_provider: &impl InfoProvider,
    backend_balances: &Vec<BalanceDto>,
) -> Vec<TokenPrice> {
    let token_addresses: Vec<String> = backend_balances
        .iter()
        .map(|balance| {
            balance
                .token_address
                .to_owned()
                .unwrap_or("0x0000000000000000000000000000000000000000".to_string())
        })
        .collect();

    // We collect the TokenPrice which were successful – unsuccessful ones are ignored
    return stream::iter(token_addresses)
        .map(|token_address| get_token_usd_rate(context, token_address, info_provider))
        .buffer_unordered(N_CONCURRENT_TOKEN_REQUESTS)
        .filter_map(|t| async move {
            match t {
                Ok(token_price) => Some(token_price),
                Err(_) => None,
            }
        })
        .collect()
        .await;
}

/// Gets the [TokenPrice] of the token with address [token_address] for the chain [chain_id]
/// To retrieve the Native Currency fiat price of the chain (eg.: Ether), 0x0000000000000000000000000000000000000000 should be used
///
/// # Arguments
///
/// * `context`: The context where the request will be executed
/// * `chain_id`: The chain id on which this request should be executed
/// * `token_address`: The token address (0x0000000000000000000000000000000000000000 for native currency)
///
/// returns: Result<TokenPrice, ApiError>
///
async fn get_token_usd_rate(
    context: &Context<'_>,
    token_address: String,
    info_provider: &impl InfoProvider,
) -> ApiResult<TokenPrice> {
    let endpoint: String = core_uri!(info_provider, "/v1/tokens/{}/prices/usd/", token_address)?;

    let body = RequestCached::new(endpoint.to_owned())
        // TODO – cache duration and timeout
        .execute(context.client(), context.cache())
        .await?;
    let response: TokenPriceCore = serde_json::from_str(&body)?;

    return Ok(TokenPrice {
        address: token_address.to_string(),
        fiat_code: response.fiat_code,
        fiat_price: response.fiat_price,
        timestamp: response.timestamp,
    });
}

pub async fn fiat_codes(context: &Context<'_>) -> ApiResult<Vec<String>> {
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
