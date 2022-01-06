use crate::cache::cache_operations::RequestCached;
use crate::common::models::backend::safe_apps::SafeApp as BackendSafeApp;
use crate::config::safe_apps_cache_duration;
use crate::routes::safe_apps::models::SafeApp;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;

pub async fn safe_apps(
    context: &RequestContext,
    chain_id: &String,
    client_url: &Option<String>,
) -> ApiResult<Vec<SafeApp>> {
    let url = config_uri!(
        "/v1/safe-apps/?chainId={}&clientUrl={}",
        chain_id,
        client_url.clone().unwrap_or("".to_string())
    );
    let data = RequestCached::new_from_context(url, &context)
        .cache_duration(safe_apps_cache_duration())
        .execute()
        .await?;

    Ok(serde_json::from_str::<Vec<BackendSafeApp>>(&data)?
        .into_iter()
        .map(|backend_safe_app| backend_safe_app.into())
        .collect::<Vec<SafeApp>>())
}
