use crate::cache::cache_operations::CacheResponse;
use crate::cache::Cache;
use crate::config::{about_cache_duration, webhook_token};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::services::about;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

/**
 * `/<chain_id>/about` <br />
 * Returns [About](crate::models::service::about::About)
 *
 * # About
 *
 * The about endpoint provides information of the environmental variables set for the instance of `safe-client-gateway`. This would allow to identify on which commit and version the last deployment happened and to which safe transaction service backend environment the current instance of the gateway is pointing to.
 *
 * ## Path
 *
 * `/<chain_id>/about`
 *
 * ## Query parameters
 *
 * There are no query parameters for this endpoint
 */
#[get("/<chain_id>/about")]
pub async fn get_about(context: Context<'_>, chain_id: String) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .duration(about_cache_duration())
        .resp_generator(|| about::about(&context, &chain_id))
        .execute(context.cache())
        .await
}

#[doc(hidden)]
#[get("/<chain_id>/about/backbone")]
pub async fn backbone(context: Context<'_>, chain_id: String) -> ApiResult<content::Json<String>> {
    let info_provider = DefaultInfoProvider::new(&context);
    let url = core_uri!(info_provider, &chain_id, "/v1/about/")?;
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
