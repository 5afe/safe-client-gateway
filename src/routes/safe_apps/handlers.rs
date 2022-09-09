use crate::cache::cache_operations::RequestCached;
use crate::cache::manager::ChainCache;
use crate::common::models::backend::safe_apps::SafeApp as BackendSafeApp;
use crate::config::safe_apps_cache_duration;
use crate::routes::safe_apps::models::SafeApp;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;

pub async fn safe_apps(
    context: &RequestContext,
    chain_id: &String,
    client_url: &Option<String>,
    url: &Option<String>,
) -> ApiResult<Vec<SafeApp>> {
    let url = config_uri!(
        "/v1/safe-apps/?chainId={}&clientUrl={}&url={}",
        chain_id,
        client_url.as_deref().unwrap_or(""),
        url.as_deref().unwrap_or("")
    );
    let data = RequestCached::new_from_context(url, &context, ChainCache::from(chain_id.as_str()))
        .cache_duration(safe_apps_cache_duration())
        .execute()
        .await?;

    Ok(serde_json::from_str::<Vec<BackendSafeApp>>(&data)?
        .into_iter()
        .map(|backend_safe_app| backend_safe_app.into())
        .collect::<Vec<SafeApp>>())
}
