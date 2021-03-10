use crate::utils::errors::{ApiError, ApiResult};
use mockall::automock;
use rocket::response::content;
use rocket_contrib::databases::redis::{self, pipe, Commands, Iter, PipelineCommands};
use serde::ser::Serialize;
use serde_json;

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
    fn exists_in_hash(&self, hash: &str, id: &str) -> bool;
    fn expire_entity(&self, id: &str, timeout: usize);
    fn invalidate_pattern(&self, pattern: &str);
    fn invalidate(&self, id: &str);
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

    fn exists_in_hash(&self, hash: &str, id: &str) -> bool {
        let exists: Option<usize> = self.hexists(hash, id).ok();
        exists.map(|it| it != 0).unwrap_or(false)
    }

    fn expire_entity(&self, id: &str, timeout: usize) {
        let _: () = self.expire(id, timeout).unwrap();
    }

    fn invalidate_pattern(&self, pattern: &str) {
        pipeline_delete(self, self.scan_match(pattern).unwrap());
    }

    fn invalidate(&self, id: &str) {
        let _: () = self.del(id).unwrap();
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

    fn request_cached(
        &self,
        client: &reqwest::blocking::Client,
        url: &str,
        timeout: usize,
        error_timeout: usize,
    ) -> ApiResult<String> {
        let cache_key = format!("{}_{}", CACHE_REQS_PREFIX, &url);
        match self.fetch(&cache_key) {
            Some(cached) => CachedWithCode::split(&cached).to_result(),
            None => {
                let response = client.get(url).send()?;
                let status_code = response.status().as_u16();

                // Early return and no caching if the error is a 500 or greater
                if response.status().is_server_error() {
                    return Err(ApiError::from_backend_error(
                        42,
                        format!("Got server error for {}", response.text()?).as_str(),
                    ));
                }

                let is_client_error = response.status().is_client_error();
                let raw_data = response.text()?;

                if is_client_error {
                    self.create(
                        &cache_key,
                        CachedWithCode::join(status_code, &raw_data).as_str(),
                        error_timeout,
                    );
                    Err(ApiError::from_backend_error(status_code, &raw_data))
                } else {
                    self.create(
                        &cache_key,
                        CachedWithCode::join(status_code, &raw_data).as_str(),
                        timeout,
                    );
                    Ok(raw_data.to_string())
                }
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub(super) struct CachedWithCode {
    pub(super) code: u16,
    pub(super) data: String,
}

impl CachedWithCode {
    const SEPARATOR: &'static str = ";";

    pub(super) fn split(cached: &str) -> Self {
        let cached_with_code: Vec<&str> = cached.splitn(2, CachedWithCode::SEPARATOR).collect();
        CachedWithCode {
            code: cached_with_code
                .get(0)
                .expect("Must have a status code")
                .parse()
                .expect("Not a valid Http code"),
            data: cached_with_code.get(1).expect("Must have data").to_string(),
        }
    }

    pub(super) fn join(code: u16, data: &str) -> String {
        format!("{}{}{}", code, CachedWithCode::SEPARATOR, data)
    }

    pub(super) fn is_error(&self) -> bool {
        200 > self.code || self.code >= 400
    }

    pub(super) fn to_result(&self) -> Result<String, ApiError> {
        if self.is_error() {
            Err(ApiError::from_backend_error(self.code, &self.data))
        } else {
            Ok(String::from(&self.data))
        }
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
