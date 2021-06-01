use crate::cache::cache_operations::CacheResponse;
use crate::cache::Cache;
use crate::config::base_transaction_service_url;
use crate::config::{about_cache_duration, webhook_token};
use crate::services::about;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

///# About
///
/// The about endpoint provides information of the environmental variables set for the instance of `safe-client-gateway`. This would allow to identify on which commit and version the last deployment happened and to which safe transaction service backend environment the current instance of the gateway is pointing to.
///
/// ## Path
///
/// `/about`
///
/// ## Query parameters
///
/// There are no query parameters for this endpoint
#[get("/about")]
pub async fn info(context: Context<'_>) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .duration(about_cache_duration())
        .resp_generator(about::get_about)
        .execute(context.cache())
        .await
}

#[doc(hidden)]
#[get("/about/backbone")]
pub async fn backbone(context: Context<'_>) -> ApiResult<content::Json<String>> {
    let url = format!("{}/v1/about/", base_transaction_service_url());
    Ok(content::Json(
        context.client().get(&url).send().await?.text().await?,
    ))
}

#[doc(hidden)]
#[get("/about/redis/<token>")]
pub fn redis(context: Context<'_>, token: String) -> ApiResult<String> {
    if token != webhook_token() {
        bail!("Invalid token");
    }
    Ok(context.cache().info().unwrap_or(String::new()))
}
