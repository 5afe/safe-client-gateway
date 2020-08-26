use crate::config::{about_cache_duration};
use crate::utils::context::Context;
use crate::services::about;
use rocket::response::content;
use anyhow::Result;
use crate::config::{base_transaction_service_url};
use crate::utils::cache::CacheExt;

#[get("/about")]
pub fn info(context: Context) -> Result<content::Json<String>> {
    context.cache().cache_resp(&context.uri(), about_cache_duration(), about::get_about)
}

#[get("/about/backbone")]
pub fn backbone(context: Context) -> Result<content::Json<String>> {
    let url = format!(
        "{}/about",
        base_transaction_service_url()
    );
    Ok(content::Json(context.client().get(&url).send()?.text()?))
}