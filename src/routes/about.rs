use crate::cache::cache::{Cache, CacheExt};
use crate::cache::cache_operations::CacheResponse;
use crate::config::base_transaction_service_url;
use crate::config::{about_cache_duration, webhook_token};
use crate::services::about;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::response::content;

#[get("/about")]
pub fn info(context: Context) -> ApiResult<content::Json<String>> {
    context
        .cache()
        .cache_resp(&context.uri(), about_cache_duration(), about::get_about)
}

#[get("/about/backbone")]
pub fn backbone(context: Context) -> ApiResult<content::Json<String>> {
    let url = format!("{}/v1/about/", base_transaction_service_url());
    Ok(content::Json(context.client().get(&url).send()?.text()?))
}

#[get("/about/redis/<token>")]
pub fn redis(context: Context, token: String) -> ApiResult<String> {
    if token != webhook_token() {
        bail!("Invalid token");
    }
    Ok(context.cache().info().unwrap_or(String::new()))
}
