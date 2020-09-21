use crate::config::{about_cache_duration};
use crate::utils::context::Context;
use crate::services::about;
use rocket::response::content;
use crate::config::{base_transaction_service_url};
use crate::utils::cache::CacheExt;
use crate::utils::errors::ApiResult;

#[get("/about")]
pub fn info(context: Context) -> ApiResult<content::Json<String>> {
    context.cache().cache_resp(&context.uri(), about_cache_duration(), about::get_about)
}

#[get("/about/backbone")]
pub fn backbone(context: Context) -> ApiResult<content::Json<String>> {
    let url = format!(
        "{}/v1/about",
        base_transaction_service_url()
    );
    Ok(content::Json(context.client().get(&url).send()?.text()?))
}