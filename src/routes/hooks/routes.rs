use rocket::serde::json::Json;

use crate::cache::cache_operations::{Invalidate, InvalidationPattern};
use crate::cache::manager::ChainCache;
use crate::common::models::backend::hooks::Payload;
use crate::common::routes::authorization::AuthorizationToken;
use crate::config::webhook_token;
use crate::routes::hooks::handlers::invalidate_caches;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;

#[post("/v1/hook/update/<token>", format = "json", data = "<update>")]
pub async fn update(
    context: RequestContext,
    token: String,
    update: Json<Payload>,
) -> ApiResult<()> {
    if token != webhook_token() {
        bail!("Invalid token");
    }
    invalidate_caches(
        context.cache(ChainCache::from(update.chain_id.as_str())),
        &update,
    )
    .await
}

#[post(
    "/v1/chains/<chain_id>/hook/update/<token>",
    format = "json",
    data = "<payload>"
)]
pub async fn post_hook_update(
    context: RequestContext,
    chain_id: String,
    token: String,
    payload: Json<Payload>,
) -> ApiResult<()> {
    update(context, token, payload).await
}

#[post(
    "/v1/chains/<chain_id>/hooks/events",
    format = "json",
    data = "<payload>"
)]
pub async fn post_hooks_events(
    context: RequestContext,
    chain_id: String,
    _token: AuthorizationToken,
    payload: Json<Payload>,
) -> ApiResult<()> {
    invalidate_caches(context.cache(ChainCache::from(chain_id.as_str())), &payload).await
}

#[post("/v1/flush/<token>", format = "json", data = "<invalidation_pattern>")]
pub async fn flush(
    context: RequestContext,
    token: String,
    invalidation_pattern: Json<InvalidationPattern>,
) -> ApiResult<()> {
    if token != webhook_token() {
        bail!("Invalid token");
    }
    Invalidate::new(invalidation_pattern.0, context.cache(ChainCache::Other))
        .execute()
        .await;
    Ok(())
}

#[post("/v2/flush", format = "json", data = "<invalidation_pattern>")]
pub async fn post_flush_events(
    context: RequestContext,
    _token: AuthorizationToken,
    invalidation_pattern: Json<InvalidationPattern>,
) -> ApiResult<()> {
    Invalidate::new(
        invalidation_pattern.0.clone(),
        context.cache(ChainCache::Mainnet),
    )
    .execute()
    .await;
    Invalidate::new(invalidation_pattern.0, context.cache(ChainCache::Other))
        .execute()
        .await;
    Ok(())
}
