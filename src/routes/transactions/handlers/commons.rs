use rocket::serde::DeserializeOwned;
use serde::de::DeserializeSeed;

use crate::{
    cache::cache_operations::RequestCached,
    common::models::page::{Page, PageMetadata},
    config::transaction_request_timeout,
    routes::transactions::models::filters::QueryParam,
    utils::{context::RequestContext, errors::ApiResult, urls::build_absolute_uri},
};

pub async fn get_backend_page<D>(
    context: &RequestContext,
    url: &str,
    page_meta: &PageMetadata,
    filters: &impl QueryParam,
) -> ApiResult<Page<D>>
where
    D: DeserializeOwned,
{
    let other_filters = filters.as_query_param();

    let url = format!("{}?{}&{}", url, page_meta.to_url_string(), other_filters);
    log::debug!("request URL: {}", &url);
    log::debug!("page_metadata: {:#?}", &page_meta);
    let body = RequestCached::new_from_context(url, context)
        .request_timeout(transaction_request_timeout())
        .execute()
        .await?;
    let object = serde_json::from_str::<Page<D>>(&body)?;
    Ok(object)
}
