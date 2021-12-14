use crate::common::models::page::Page;
use crate::providers::info::DefaultInfoProvider;
use crate::providers::info::InfoProvider;
use crate::routes::delegates::models::{
    Delegate, DelegateCreate, DelegateDelete, SafeDelegateDelete,
};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::Request;

pub async fn get_delegates(
    context: &RequestContext,
    chain_id: String,
    safe: Option<String>,
    delegate: Option<String>,
    delegator: Option<String>,
    label: Option<String>,
) -> ApiResult<Page<Delegate>> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let url = core_uri!(
        info_provider,
        "/v1/delegates/?safe={}&delegate={}&delegator={}&label={}",
        safe.unwrap_or(String::from("")),
        delegate.unwrap_or(String::from("")),
        delegator.unwrap_or(String::from("")),
        label.unwrap_or(String::from("")),
    )?;

    let request = Request::new(url);

    let response = context.http_client().get(request).await?;
    let safe_delegates = serde_json::from_str::<Page<Delegate>>(&response.body)?;

    return Ok(safe_delegates);
}

pub async fn post_delegate(
    context: &RequestContext,
    chain_id: String,
    safe_delegate_create: DelegateCreate,
) -> ApiResult<()> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let url = core_uri!(info_provider, "/v1/delegates/",)?;

    let request = {
        let mut request = Request::new(url);
        request.body(serde_json::to_string(&safe_delegate_create).ok());
        request
    };

    context.http_client().post(request).await?;
    Ok(())
}

pub async fn delete_delegate(
    context: &RequestContext,
    chain_id: String,
    delegate_address: String,
    delegate_delete: DelegateDelete,
) -> ApiResult<()> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let url = core_uri!(info_provider, "/v1/delegates/{}/", delegate_address)?;

    let request = {
        let mut request = Request::new(url);
        request.body(serde_json::to_string(&delegate_delete).ok());
        request
    };

    context.http_client().delete(request).await?;
    Ok(())
}

pub async fn delete_safe_delegate(
    context: &RequestContext,
    chain_id: String,
    safe_address: String,
    delegate_address: String,
    safe_delegate_delete: SafeDelegateDelete,
) -> ApiResult<()> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/delegates/{}/",
        safe_address,
        delegate_address
    )?;

    let request = {
        let mut request = Request::new(url);
        request.body(serde_json::to_string(&safe_delegate_delete).ok());
        request
    };

    context.http_client().delete(request).await?;
    Ok(())
}
