use crate::cache::cache_operations::{Invalidate, InvalidationPattern};
use crate::common::models::backend::hooks::Payload;
use crate::config::webhook_token;
use crate::routes::hooks::handlers::invalidate_caches;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use rocket::serde::json::Json;

#[post("/v1/hook/update/<token>", format = "json", data = "<update>")]
pub fn update(context: Context<'_>, token: String, update: Json<Payload>) -> ApiResult<()> {
    if token != webhook_token() {
        bail!("Invalid token");
    }
    invalidate_caches(context.cache(), &update)
}

#[post("/v1/flush/<token>", format = "json", data = "<invalidation_pattern>")]
pub fn flush(
    context: Context,
    token: String,
    invalidation_pattern: Json<InvalidationPattern>,
) -> ApiResult<()> {
    if token != webhook_token() {
        bail!("Invalid token");
    }
    Invalidate::new(invalidation_pattern.0).execute(context.cache());
    Ok(())
}
