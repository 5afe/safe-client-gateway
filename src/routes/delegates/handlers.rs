use crate::common::models::page::Page;
use crate::providers::info::DefaultInfoProvider;
use crate::providers::info::InfoProvider;
use crate::routes::delegates::models::{
    Delegate, DelegateCreate, DelegateDelete, SafeDelegateDelete,
};
use crate::utils::context::Context;
use crate::utils::errors::{ApiError, ApiResult};

pub async fn get_delegates(
    context: Context<'_>,
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

    let safe_delegates: Page<Delegate> = context
        .client()
        .get(&url)
        .send()
        .await?
        .json::<Page<Delegate>>()
        .await?;

    return Ok(safe_delegates);
}

pub async fn post_delegate(
    context: Context<'_>,
    chain_id: String,
    safe_delegate_create: DelegateCreate,
) -> ApiResult<()> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let url = core_uri!(info_provider, "/v1/delegates/",)?;

    let response = context
        .client()
        .post(&url)
        .json(&safe_delegate_create)
        .send()
        .await?;

    return if response.status().is_success() {
        Ok(())
    } else {
        Err(ApiError::from_http_response(
            response,
            String::from("Unexpected delegate creation error"),
        )
        .await)
    };
}

pub async fn delete_delegate(
    context: Context<'_>,
    chain_id: String,
    delegate_address: String,
    delegate_delete: DelegateDelete,
) -> ApiResult<()> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let url = core_uri!(info_provider, "/v1/delegates/{}", delegate_address)?;

    let response = context
        .client()
        .delete(&url)
        .json(&delegate_delete)
        .send()
        .await?;

    return if response.status().is_success() {
        Ok(())
    } else {
        Err(ApiError::from_http_response(
            response,
            String::from("Unexpected delegate deletion error"),
        )
        .await)
    };
}

pub async fn delete_safe_delegate(
    context: Context<'_>,
    chain_id: String,
    safe_address: String,
    delegate_address: String,
    safe_delegate_delete: SafeDelegateDelete,
) -> ApiResult<()> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let url = core_uri!(
        info_provider,
        "/v1/safes/{}/delegates/{}",
        safe_address,
        delegate_address
    )?;

    let response = context
        .client()
        .delete(&url)
        .json(&safe_delegate_delete)
        .send()
        .await?;

    return if response.status().is_success() {
        Ok(())
    } else {
        Err(ApiError::from_http_response(
            response,
            String::from("Unexpected delegate deletion error"),
        )
        .await)
    };
}
