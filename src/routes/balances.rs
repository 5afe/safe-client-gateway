use crate::cache::cache_operations::CacheResponse;
use crate::config::{balances_cache_duration, feature_flag_balances_rate_implementation};
use crate::services::balances::fiat_codes;
use crate::services::{balances, balances_v2};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::futures::FutureExt;
use rocket::response::content;

/**
 * `/v1/chains/<chain_id>/safes/<safe_address>/balances/<fiat>?<trusted>&<exclude_spam>`<br/>
 * Returns [Balances](crate::models::service::balances::Balances)
 *
 * # Balances
 *
 * This endpoint returns the [crate::models::service::balances::Balances] with information (when available) of their converted balance into a designated fiat. The entries are sorted by their fiat balance value.
 *
 * The `fiat_code` can be selected from any of the values returned by the supported fiat endpoint.
 *
 * The total balance in the designated fiat is also part of the response.
 *
 * ## Path
 *
 * - `/v1/chains/<chain_id>/safes/<safe_address>/balances/<fiat>?<trusted>&<exclude_spam>` returns the balance for every supported ERC20 token for a `<safe_address>`, as well as the aggregated fiat total in the fiat currency requested with `<fiat>` . Sorted by fiat balance.
 *
 * ## Query parameters
 *
 * - `<trusted>` : A token is defined as trusted by our core service process when adding them. Default value is `false`
 * - `<exclude_spam>`: A token is defined as spam by our core service process when adding them. Default value is `true`
 */
#[get("/v1/chains/<chain_id>/safes/<safe_address>/balances/<fiat>?<trusted>&<exclude_spam>")]
pub async fn get_balances(
    context: Context<'_>,
    chain_id: String,
    safe_address: String,
    fiat: String,
    trusted: Option<bool>,
    exclude_spam: Option<bool>,
) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .duration(balances_cache_duration())
        .resp_generator(|| {
            if feature_flag_balances_rate_implementation() {
                balances_v2::balances(
                    &context,
                    chain_id.as_str(),
                    safe_address.as_str(),
                    fiat.as_str(),
                    trusted.unwrap_or(false),
                    exclude_spam.unwrap_or(true),
                )
                .left_future()
            } else {
                balances::balances(
                    &context,
                    chain_id.as_str(),
                    safe_address.as_str(),
                    fiat.as_str(),
                    trusted.unwrap_or(false),
                    exclude_spam.unwrap_or(true),
                )
                .right_future()
            }
        })
        .execute(context.cache())
        .await
}

/**
 * `/v1/balances/supported-fiat-codes` <br/>
 * Returns [Vec] of [String]
 *
 * Supported fiat codes for balances
 * `/v1/balances/supported-fiat-codes` : returns the supported fiat codes to be included int the `<fiat>` segment of the balance endpoint.
 * The entries are sorted alphabetically, with the exception of `USD` and `EUR` being placed in the top of the list in that order.
 */
#[get("/v1/balances/supported-fiat-codes")]
pub async fn get_supported_fiat(context: Context<'_>) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .resp_generator(|| fiat_codes(&context))
        .execute(context.cache())
        .await
}
