use crate::cache::cache_operations::{CacheResponse, InvalidationPattern, RequestCached};
use crate::cache::inner_cache::CachedWithCode;
use crate::cache::Cache;
use crate::utils::errors::{ApiError, ApiResult};
use rocket::response::content;
use serde::Serialize;
use std::time::Duration;

const CACHE_REQS_PREFIX: &'static str = "c_reqs";
const CACHE_RESP_PREFIX: &'static str = "c_resp";
const CACHE_REQS_RESP_PREFIX: &'static str = "c_re";

pub(super) fn invalidate(cache: &impl Cache, pattern: &InvalidationPattern) {
    let pattern_str = match pattern {
        InvalidationPattern::FlushAll => String::from("*"),
        InvalidationPattern::RequestsResponses(value) => {
            format!("{}*{}*", CACHE_REQS_RESP_PREFIX, &value)
        }
    };

    cache.invalidate_pattern(pattern_str.as_str());
}

pub(super) fn cache_response<S>(
    cache: &impl Cache,
    cache_response: &CacheResponse<S>,
) -> ApiResult<content::Json<String>>
where
    S: Serialize,
{
    let cache_key = format!("{}_{}", CACHE_RESP_PREFIX, cache_response.key);
    let cached = cache.fetch(&cache_key);
    match cached {
        Some(value) => Ok(content::Json(value)),
        None => {
            let resp_string = serde_json::to_string(&cache_response.generate()?)?;
            cache.create(&cache_key, &resp_string, cache_response.duration);
            Ok(content::Json(resp_string))
        }
    }
}

pub(super) async fn request_cached(
    cache: &dyn Cache,
    client: &reqwest::Client,
    operation: &RequestCached,
) -> ApiResult<String> {
    let cache_key = format!("{}_{}", CACHE_REQS_PREFIX, &operation.url);
    match cache.fetch(&cache_key) {
        Some(cached) => CachedWithCode::split(&cached).to_result(),
        None => {
            let request = client
                .get(&operation.url)
                .timeout(Duration::from_millis(operation.request_timeout));

            let response = (request.send().await).map_err(|err| {
                if operation.cache_all_errors {
                    cache.create(
                        &cache_key,
                        &CachedWithCode::join(500, &format!("{:?}", &err)),
                        operation.error_cache_duration,
                    );
                }
                err
            })?;
            let status_code = response.status().as_u16();

            // Early return and no caching if the error is a 500 or greater
            let is_server_error = response.status().is_server_error();
            if !operation.cache_all_errors && is_server_error {
                return Err(ApiError::from_backend_error(
                    42,
                    &format!("Got server error for {}", response.text().await?),
                ));
            }

            let is_client_error = response.status().is_client_error();
            let raw_data = response.text().await?;

            if is_client_error || is_server_error {
                cache.create(
                    &cache_key,
                    &CachedWithCode::join(status_code, &raw_data),
                    operation.error_cache_duration,
                );
                Err(ApiError::from_backend_error(status_code, &raw_data))
            } else {
                cache.create(
                    &cache_key,
                    &CachedWithCode::join(status_code, &raw_data),
                    operation.cache_duration,
                );
                Ok(raw_data.to_string())
            }
        }
    }
}
