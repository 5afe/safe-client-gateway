use std::borrow::Borrow;
use std::iter::Map;

use reqwest::Url;
use rocket::serde::DeserializeOwned;

use crate::cache::cache_operations::RequestCached;
use crate::common::models::page::{Page, PageMetadata};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;

pub async fn get_backend_page<D, I, K, V>(
    context: &RequestContext,
    url: &str,
    request_timeout: u64,
    query_params: &I,
) -> ApiResult<Page<D>>
where
    D: DeserializeOwned,
    I: IntoIterator,
    I::Item: Borrow<(K, V)>,
{
    let url = Url::parse_with_params(url, query_params).expect("TODO: panic message");
    let body = RequestCached::new_from_context(url.to_string(), context)
        .request_timeout(request_timeout)
        .execute()
        .await?;
    let object = serde_json::from_str::<Page<D>>(&body)?;
    Ok(object)
}
