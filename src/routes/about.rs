use crate::config::{about_cache_duration};
use crate::utils::context::Context;
use crate::services::about;
use rocket::response::content;
use anyhow::Result;

#[get("/about")]
pub fn info(context: Context) -> Result<content::Json<String>> {
    context.cache().cache_resp(&context.uri(), about_cache_duration(), about::get_about)
}