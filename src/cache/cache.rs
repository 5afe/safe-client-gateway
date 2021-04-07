use crate::cache::inner_cache::CachedWithCode;
use crate::config::redis_scan_count;
use crate::utils::errors::{ApiError, ApiResult};
use mockall::automock;
use rocket::response::content;
use rocket_contrib::databases::redis::{
    self, pipe, Commands, FromRedisValue, Iter, PipelineCommands, ToRedisArgs,
};
use serde::ser::Serialize;
use serde_json;
use std::time::Duration;

pub const CACHE_RESP_PREFIX: &'static str = "c_resp";
pub const CACHE_REQS_PREFIX: &'static str = "c_reqs";

#[database("service_cache")]
pub struct ServiceCache(redis::Connection);

#[automock]
pub trait Cache {
    fn fetch(&self, id: &str) -> Option<String>;
    fn create(&self, id: &str, dest: &str, timeout: usize);
    fn insert_in_hash(&self, hash: &str, id: &str, dest: &str);
    fn get_from_hash(&self, hash: &str, id: &str) -> Option<String>;
    fn has_key(&self, id: &str) -> bool;
    fn expire_entity(&self, id: &str, timeout: usize);
    fn invalidate_pattern(&self, pattern: &str);
    fn invalidate(&self, id: &str);
    fn info(&self) -> Option<String>;
}

impl Cache for ServiceCache {
    fn fetch(&self, id: &str) -> Option<String> {
        match self.get(id) {
            Ok(value) => Some(value),
            _ => None,
        }
    }

    fn create(&self, id: &str, dest: &str, timeout: usize) {
        let _: () = self.set_ex(id, dest, timeout).unwrap();
    }

    fn insert_in_hash(&self, hash: &str, id: &str, dest: &str) {
        let _: () = self.hset(hash, id, dest).unwrap();
    }

    fn get_from_hash(&self, hash: &str, id: &str) -> Option<String> {
        self.hget(hash, id).ok()
    }

    fn has_key(&self, id: &str) -> bool {
        let result: Option<usize> = self.exists(id).ok();
        result.map(|it| it != 0).unwrap_or(false)
    }

    fn expire_entity(&self, id: &str, timeout: usize) {
        let _: () = self.expire(id, timeout).unwrap();
    }

    fn invalidate_pattern(&self, pattern: &str) {
        pipeline_delete(self, scan_match_count(self, pattern, redis_scan_count()));
    }

    fn invalidate(&self, id: &str) {
        let _: () = self.del(id).unwrap();
    }

    fn info(&self) -> Option<String> {
        info(self)
    }
}

pub trait CacheExt: Cache {
    fn invalidate_caches(&self, key: &str) {
        self.invalidate_pattern(&format!("c_re*{}*", &key));
    }

    fn cache_resp<R>(
        &self,
        key: &str,
        timeout: usize,
        resp: impl Fn() -> ApiResult<R>,
    ) -> ApiResult<content::Json<String>>
    where
        R: Serialize,
    {
        let cache_key = format!("{}_{}", CACHE_RESP_PREFIX, &key);
        let cached = self.fetch(&cache_key);
        match cached {
            Some(value) => Ok(content::Json(value)),
            None => {
                let resp = resp()?;
                let resp_string = serde_json::to_string(&resp)?;
                self.create(&cache_key, &resp_string, timeout);
                Ok(content::Json(resp_string))
            }
        }
    }

    fn request_cached_advanced(
        &self,
        client: &reqwest::blocking::Client,
        url: &str,
        cache_duration: usize,
        error_cache_duration: usize,
        cache_all_errors: bool,
        request_timeout: u64,
    ) -> ApiResult<String> {
        let cache_key = format!("{}_{}", CACHE_REQS_PREFIX, &url);
        match self.fetch(&cache_key) {
            Some(cached) => CachedWithCode::split(&cached).to_result(),
            None => {
                let mut request = client.get(url);
                if request_timeout > 0 {
                    request = request.timeout(Duration::from_millis(request_timeout));
                }
                let response = request.send().map_err(|err| {
                    if cache_all_errors {
                        self.create(
                            &cache_key,
                            &CachedWithCode::join(500, &format!("{:?}", &err)),
                            error_cache_duration,
                        );
                    }
                    err
                })?;
                let status_code = response.status().as_u16();

                // Early return and no caching if the error is a 500 or greater
                let is_server_error = response.status().is_server_error();
                if !cache_all_errors && is_server_error {
                    return Err(ApiError::from_backend_error(
                        42,
                        &format!("Got server error for {}", response.text()?),
                    ));
                }

                let is_client_error = response.status().is_client_error();
                let raw_data = response.text()?;

                if is_client_error || is_server_error {
                    self.create(
                        &cache_key,
                        &CachedWithCode::join(status_code, &raw_data),
                        error_cache_duration,
                    );
                    Err(ApiError::from_backend_error(status_code, &raw_data))
                } else {
                    self.create(
                        &cache_key,
                        &CachedWithCode::join(status_code, &raw_data),
                        cache_duration,
                    );
                    Ok(raw_data.to_string())
                }
            }
        }
    }

    fn request_cached(
        &self,
        client: &reqwest::blocking::Client,
        url: &str,
        cache_duration: usize,
        error_cache_duration: usize,
    ) -> ApiResult<String> {
        self.request_cached_advanced(client, url, cache_duration, error_cache_duration, false, 0)
    }
}

impl<T: Cache + ?Sized> CacheExt for T {}

fn pipeline_delete(con: &redis::Connection, keys: Iter<String>) {
    let pipeline = &mut pipe();
    for key in keys {
        pipeline.del(key);
    }
    pipeline.execute(con);
}

fn scan_match_count<P: ToRedisArgs, C: ToRedisArgs, RV: FromRedisValue>(
    con: &redis::Connection,
    pattern: P,
    count: C,
) -> redis::Iter<RV> {
    redis::cmd("SCAN")
        .cursor_arg(0)
        .arg("MATCH")
        .arg(pattern)
        .arg("COUNT")
        .arg(count)
        .iter(con)
        .unwrap()
}

fn info(con: &redis::Connection) -> Option<String> {
    redis::cmd("INFO").query(con).ok()
}
