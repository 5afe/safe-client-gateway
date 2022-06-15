use crate::routes::delegates::handlers;
use crate::routes::delegates::models::{DelegateCreate, DelegateDelete, SafeDelegateDelete};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::response::content;
use rocket::serde::json::Json;

#[get("/v1/chains/<chain_id>/delegates?<safe>&<delegate>&<delegator>&<label>")]
pub async fn get_delegates<'e>(
    context: RequestContext,
    chain_id: String,
    safe: Option<String>,
    delegate: Option<String>,
    delegator: Option<String>,
    label: Option<String>,
) -> ApiResult<content::RawJson<String>> {
    let json = serde_json::to_string(
        &handlers::get_delegates(&context, chain_id, safe, delegate, delegator, label).await?,
    )?;
    Ok(content::RawJson(json))
}

#[post(
    "/v1/chains/<chain_id>/delegates",
    format = "application/json",
    data = "<safe_delegate>"
)]
pub async fn post_delegate<'e>(
    context: RequestContext,
    chain_id: String,
    safe_delegate: Json<DelegateCreate>,
) -> ApiResult<()> {
    return handlers::post_delegate(&context, chain_id, safe_delegate.0).await;
}

#[delete(
    "/v1/chains/<chain_id>/delegates/<delegate_address>",
    format = "application/json",
    data = "<delegate_delete>"
)]
pub async fn delete_delegate<'e>(
    context: RequestContext,
    chain_id: String,
    delegate_address: String,
    delegate_delete: Json<DelegateDelete>,
) -> ApiResult<()> {
    return handlers::delete_delegate(&context, chain_id, delegate_address, delegate_delete.0)
        .await;
}

#[delete(
    "/v1/chains/<chain_id>/safes/<safe_address>/delegates/<delegate_address>",
    format = "application/json",
    data = "<delegate_delete>"
)]
pub async fn delete_safe_delegate<'e>(
    context: RequestContext,
    chain_id: String,
    safe_address: String,
    delegate_address: String,
    delegate_delete: Json<SafeDelegateDelete>,
) -> ApiResult<()> {
    return handlers::delete_safe_delegate(
        &context,
        chain_id,
        safe_address,
        delegate_address,
        delegate_delete.0,
    )
    .await;
}
