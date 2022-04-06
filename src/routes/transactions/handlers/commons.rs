use crate::{
    cache::cache_operations::RequestCached,
    common::models::page::{Page, PageMetadata},
    routes::transactions::models::filters::QueryParam,
    utils::{context::RequestContext, errors::ApiResult},
};
use rocket::serde::DeserializeOwned;

pub async fn get_backend_page<D>(
    context: &RequestContext,
    url: &str,
    request_timeout: u64,
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
        .request_timeout(request_timeout)
        .execute()
        .await?;
    let object = serde_json::from_str::<Page<D>>(&body)?;
    Ok(object)
}
