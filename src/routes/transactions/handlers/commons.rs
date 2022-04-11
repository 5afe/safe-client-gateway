use crate::cache::cache_operations::RequestCached;
use crate::common::models::page::{Page, PageMetadata};
use crate::routes::transactions::filters::QueryParam;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::serde::DeserializeOwned;

pub async fn get_backend_page<D>(
    context: &RequestContext,
    url: &str,
    request_timeout: u64,
    page_meta: &PageMetadata,
    filters: &(impl QueryParam + std::fmt::Debug),
) -> ApiResult<Page<D>>
where
    D: DeserializeOwned,
{
    let other_filters = filters.as_query_param();

    let url = format!("{}?{}&{}", url, page_meta.to_url_string(), other_filters);
    log::debug!("request URL: {}", &url);
    log::debug!("page_metadata: {:#?}", &page_meta);
    log::debug!("filters: {:#?}", &filters);
    let body = RequestCached::new_from_context(url, context)
        .request_timeout(request_timeout)
        .execute()
        .await?;
    let object = serde_json::from_str::<Page<D>>(&body)?;
    Ok(object)
}
