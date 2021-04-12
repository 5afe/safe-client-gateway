use crate::cache::cache_operations::RequestCached;
use crate::config::base_transaction_service_url;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/v1/safes/<safe_address>/collectibles?<trusted>&<exclude_spam>")]
pub fn list(
    context: Context,
    safe_address: String,
    trusted: Option<bool>,
    exclude_spam: Option<bool>,
) -> ApiResult<content::Json<String>> {
    let url = format!(
        "{}/v1/safes/{}/collectibles/?trusted={}&exclude_spam={}",
        base_transaction_service_url(),
        safe_address,
        trusted.unwrap_or(false),
        exclude_spam.unwrap_or(true)
    );

    Ok(content::Json(
        RequestCached::new()
            .url(url)
            .execute(context.client(), context.cache())?,
    ))
}
