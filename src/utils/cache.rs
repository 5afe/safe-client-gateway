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
        key: &String,
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
        url: &String,
        timeout: usize,
    ) -> ApiResult<String> {
        match self.fetch(&url) {
            Some(cached) => {
                let cached_with_code = CachedWithCode::split(&cached);
                if cached_with_code.is_error() {
                    Ok(String::from(cached_with_code.data))
                } else {
                    Err(ApiError { status: cached_with_code.code, message: Some(cached_with_code.data) })
                }
            }
            None => {
                let response = client.get(url).send()?;
                if response.status().is_server_error() {
                    // This should panic or be refactored to return an error
                    return Err(anyhow::anyhow!("Got server error for {}", url).into());
                }
                let status_code = response.status().as_u16();
                let is_client_error = response.status().is_client_error();
                let raw_data = response.text()?;
                // Add status code + data to cache ... currently it is only data
                self.create(&url, CachedWithCode::join(status_code, &raw_data).as_str(), timeout);
                if is_client_error {
                    return Err(ApiError { status: status_code, message: None });
                }
                Ok(raw_data)
            }
        }
    }
}

struct CachedWithCode {
    code: u16,
    data: String,
}

impl CachedWithCode {
    pub(super) fn split(cached: &str) -> Self {
        let cached_with_code: Vec<&str> = cached.split(";").collect();
        CachedWithCode {
            code: cached_with_code.get(0).expect("Must have a status code").parse().expect("Not a valid Http code"),
            data: cached_with_code.get(1).expect("Must have data").to_string(),
        }
    }

    pub(super) fn join(code: u16, data: &str) -> String {
        format!("{};{}", code, data)
    }

    pub(super) fn is_error(&self) -> bool {
        500 > self.code && self.code >= 400
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
