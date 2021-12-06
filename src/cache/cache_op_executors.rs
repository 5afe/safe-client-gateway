use crate::cache::cache_operations::{CacheResponse, InvalidationPattern, RequestCached};
use crate::cache::inner_cache::CachedWithCode;
use crate::cache::{Cache, CACHE_REQS_PREFIX, CACHE_RESP_PREFIX};
use crate::utils::errors::ApiResult;
use crate::utils::http_client::Request;
use rocket::response::content;
use serde::Serialize;
use std::sync::Arc;
use std::time::Duration;

pub(super) fn invalidate(cache: Arc<dyn Cache>, pattern: &InvalidationPattern) {
    cache.invalidate_pattern(pattern.to_pattern_string().as_str());
}

pub(super) async fn cache_response<S>(
    cache_response: &CacheResponse<'_, S>,
) -> ApiResult<content::Json<String>>
where
    S: Serialize,
{
    let cache = cache_response.cache.clone();
    let cache_key = format!("{}_{}", CACHE_RESP_PREFIX, cache_response.key);
    let cached = cache.fetch(&cache_key);
    match cached {
        Some(value) => Ok(content::Json(value)),
        None => {
            let resp_string = serde_json::to_string(&cache_response.generate().await?)?;
            cache.create(&cache_key, &resp_string, cache_response.duration);
            Ok(content::Json(resp_string))
        }
    }
}

pub(super) async fn request_cached(operation: &RequestCached) -> ApiResult<String> {
    let cache = operation.cache.clone();
    let client = operation.client.clone();
    let cache_key = format!("{}_{}", CACHE_REQS_PREFIX, &operation.url);
    match cache.fetch(&cache_key) {
        Some(cached) => CachedWithCode::split(&cached).to_result(),
        None => {
            let http_request = {
                let mut request = Request::new(String::from(&operation.url));
                request.timeout(Duration::from_millis(operation.request_timeout));
                request
            };
            let response = client.get(http_request).await;

            match response {
                Err(error) => {
                    let default_message: String = String::from("Unknown error");
                    let response_body: &String =
                        error.details.message.as_ref().unwrap_or(&default_message);
                    // TODO extract http error range check (client error vs server error)
                    let is_client_error = error.status >= 400 && error.status < 500;

                    // If cache_all_errors is enabled we cache both client and server errors
                    // else we just cache client errors
                    if is_client_error || operation.cache_all_errors {
                        cache.create(
                            &cache_key,
                            &CachedWithCode::join(error.status, &response_body),
                            operation.error_cache_duration,
                        );
                    }

                    Err(error)
                }
                Ok(response) => {
                    let status_code = response.status_code;
                    let response_body = response.body;

                    cache.create(
                        &cache_key,
                        &CachedWithCode::join(status_code, &response_body),
                        operation.cache_duration,
                    );
                    Ok(response_body.to_string())
                }
            }
        }
    }
}
