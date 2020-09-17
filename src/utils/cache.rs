use rocket::response::content;
use rocket_contrib::databases::redis::{self, pipe, Commands, Iter, PipelineCommands};
use serde::ser::Serialize;
use serde_json;
use mockall::automock;
use crate::utils::errors::{ApiResult, ApiError};

#[database("service_cache")]
pub struct ServiceCache(redis::Connection);

#[automock]
pub trait Cache {
    fn fetch(&self, id: &str) -> Option<String>;
    fn create(&self, id: &str, dest: &str, timeout: usize);
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

    fn invalidate_pattern(&self, pattern: &str) {
        pipeline_delete(self, self.scan_match(pattern).unwrap());
    }

    fn invalidate(&self, id: &str) {
        let _: () = self.del(id).unwrap();
    }
}

pub trait CacheExt: Cache {
    fn cache_resp<R>(
        &self,
        key: &str,
        timeout: usize,
        resp: impl Fn() -> ApiResult<R>,
    ) -> ApiResult<content::Json<String>>
        where R: Serialize {
        let cached = self.fetch(key);
        match cached {
            Some(value) => Ok(content::Json(value)),
            None => {
                let resp = resp()?;
                let resp_string = serde_json::to_string(&resp)?;
                self.create(key, &resp_string, timeout);
                Ok(content::Json(resp_string))
            }
        }
    }

    fn request_cached(
        &self,
        client: &reqwest::blocking::Client,
        url: &str,
        timeout: usize,
    ) -> ApiResult<String> {
        match self.fetch(&url) {
            Some(cached) => {
                CachedWithCode::split(&cached).to_result()
            }
            None => {
                let response = client.get(url).send()?;
                let status_code = response.status().as_u16();

                if response.status().is_server_error() {
                    return Err(ApiError::from_backend_error(42, format!("Got server error for {}", response.text()?).as_str()));
                }
                let is_client_error = response.status().is_client_error();
                let raw_data = response.text()?;
                self.create(&url, CachedWithCode::join(status_code, &raw_data).as_str(), timeout);
                return if is_client_error {
                    Err(ApiError::from_backend_error(status_code, &raw_data))
                } else {
                    Ok(raw_data)
                };
            }
        }
    }

    fn cache_string(
        &self,
        key: &str,
        timeout: usize,
    ) -> ApiResult<String> {
        let cached = self.fetch(key);
        match cached {
            Some(value) => Ok(value),
            None => {
                let to_cache = String::new();
                self.create(key, to_cache.as_str(), timeout);
                Ok(to_cache)
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
            code: cached_with_code.get(0).expect("Must have a status code").parse().expect("Not a valid Http code"),
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
