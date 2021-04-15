use crate::cache::cache_operations::CacheResponse;
use crate::cache::Cache;
use crate::config::base_transaction_service_url;
use crate::config::{about_cache_duration, webhook_token};
use crate::services::about;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/about")]
pub fn info(context: Context<'_>) -> ApiResult<content::Json<String>> {
    CacheResponse::new(context.uri())
        .duration(about_cache_duration())
        .resp_generator(about::get_about)
        .execute(context.cache())
}

#[get("/about/backbone")]
pub async fn backbone(context: Context<'_>) -> ApiResult<content::Json<String>> {
    let url = format!("{}/v1/about/", base_transaction_service_url());
    Ok(content::Json(
        context.client().get(&url).send().await?.text().await?,
    ))
}

#[get("/about/redis/<token>")]
pub fn redis(context: Context<'_>, token: String) -> ApiResult<String> {
    if token != webhook_token() {
        bail!("Invalid token");
    }
    Ok(context.cache().info().unwrap_or(String::new()))
}
