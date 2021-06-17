use crate::cache::cache_operations::RequestCached;
use crate::config::collectibles_request_timeout;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket::response::content::Json;

pub async fn collectibles(
    context: &Context<'_>,
    chain_id: &str,
    safe_address: &str,
    trusted: Option<bool>,
    exclude_spam: Option<bool>,
) -> ApiResult<Json<String>> {
    let info_provider = DefaultInfoProvider::new(chain_id, &context);

    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/collectibles/?trusted={}&exclude_spam={}",
        safe_address,
        trusted.unwrap_or(false),
        exclude_spam.unwrap_or(true)
    )?;

    Ok(content::Json(
        RequestCached::new(url)
            .request_timeout(collectibles_request_timeout())
            .execute(context.client(), context.cache())
            .await?,
    ))
}
