use crate::cache::cache_operations::{Invalidate, InvalidationPattern};
use crate::common::models::backend::hooks::Payload;
use crate::config::webhook_token;
use crate::routes::hooks::handlers::invalidate_caches;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::serde::json::Json;

#[post("/v1/hook/update/<token>", format = "json", data = "<update>")]
pub fn update(context: RequestContext, token: String, update: Json<Payload>) -> ApiResult<()> {
    if token != webhook_token() {
        bail!("Invalid token");
    }
    invalidate_caches(context.cache(), &update)
}

#[post(
    "/v1/chains/<chain_id>/hook/update/<token>",
    format = "json",
    data = "<payload>"
)]
pub fn post_hook_update(
    context: RequestContext,
    chain_id: String,
    token: String,
    payload: Json<Payload>,
) -> ApiResult<()> {
    update(context, token, payload)
}

#[post("/v1/flush/<token>", format = "json", data = "<invalidation_pattern>")]
pub fn flush(
    context: RequestContext,
    token: String,
    invalidation_pattern: Json<InvalidationPattern>,
) -> ApiResult<()> {
    if token != webhook_token() {
        bail!("Invalid token");
    }
    Invalidate::new(invalidation_pattern.0, context.cache()).execute();
    Ok(())
}
