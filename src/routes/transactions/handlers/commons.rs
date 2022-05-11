use crate::cache::cache_operations::RequestCached;
use crate::cache::manager::ChainCache;
use crate::common::models::page::{Page, PageMetadata};
use crate::routes::transactions::filters::QueryParam;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use rocket::serde::DeserializeOwned;

pub async fn get_backend_page<D>(
    context: &RequestContext,
    chain_id: &str,
    url: &str,
    request_timeout: u64,
    page_meta: &Option<PageMetadata>,
    filters: &(impl QueryParam + std::fmt::Debug),
) -> ApiResult<Page<D>>
where
    D: DeserializeOwned,
{
    let mut full_url = String::from(url);
    full_url.push_str("?");
    page_meta.as_ref().map(|page_meta| {
        full_url.push_str("&");
        full_url.push_str(&page_meta.to_url_string());
    });

    let filters = filters.as_query_param();
    if !filters.is_empty() {
        full_url.push_str("&");
        full_url.push_str(&filters);
    }
    let body = RequestCached::new_from_context(full_url, context, ChainCache::from(chain_id))
        .request_timeout(request_timeout)
        .execute()
        .await?;
    let object = serde_json::from_str::<Page<D>>(&body)?;
    Ok(object)
}
