use crate::utils::errors::ApiResult;
use crate::utils::context::Context;
use crate::config::request_cache_duration;
use rocket::response::content;
use crate::utils::cache::CacheExt;
use crate::services::balances::*;
use crate::models::service::balances::*;
use crate::providers::info::DefaultInfoProvider;

#[get("/v1/safes/<safe_address>/balances/<fiat>")]
pub fn get_balances(context: Context, safe_address: String, fiat: String) -> ApiResult<content::Json<String>> {
    let info_provider = DefaultInfoProvider::new(&context);
    let fiat: Fiat = Fiat::from_dasd(fiat.as_str(), &info_provider);

    context.cache().cache_resp(&context.uri(), request_cache_duration(), || {
        balances(&context, safe_address.as_str(), &fiat)
    })
}