use crate::cache::cache::{Cache, CacheExt};
use crate::config::{request_cache_duration, request_error_cache_timeout};
use crate::utils::errors::{ApiError, ApiResult};
use rocket::response::content;
use serde::Serialize;
use std::borrow::{Borrow, BorrowMut};

pub enum Database {
    Info = 1,
    Default = 2,
}

pub struct Invalidate {
    pattern: String,
    database: Database,
}

impl Invalidate {
    pub fn new() -> Self {
        Invalidate {
            pattern: String::new(),
            database: Database::Default,
        }
    }

    fn pattern(&mut self, pattern: &str) -> &mut Self {
        self.pattern = pattern.to_string();
        self
    }

    fn database(&mut self, database: Database) -> &mut Self {
        self.database = database;
        self
    }
}

pub struct CacheResponse<'a, R>
where
    R: Serialize,
{
    database: Database,
    pub key: String,
    pub duration: usize,
    pub resp_generator: Box<dyn Fn() -> ApiResult<R> + 'a>,
}

impl<'a, R> CacheResponse<'a, R>
where
    R: Serialize,
{
    pub fn new() -> Self {
        CacheResponse {
            database: Database::Default,
            key: String::new(),
            duration: request_cache_duration(),
            resp_generator: Box::new(|| bail!("Need to set a response callback")),
        }
    }

    pub fn database(&mut self, database: Database) -> &mut Self {
        self.database = database;
        self
    }

    pub fn key(&mut self, key: String) -> &mut Self {
        self.key = key;
        self
    }

    pub fn duration(&mut self, duration: usize) -> &mut Self {
        self.duration = duration;
        self
    }

    pub fn resp_generator(&mut self, resp_generator: impl Fn() -> ApiResult<R> + 'a) -> &mut Self {
        self.resp_generator = Box::new(resp_generator);
        self
    }

    pub fn generate(&self) -> ApiResult<R> {
        (self.resp_generator)()
    }

    pub fn execute(&self, cache: &impl Cache) -> ApiResult<content::Json<String>> {
        let cache_key = format!("{}_{}", "c_resp", self.key);
        let cached = cache.fetch(&cache_key);
        match cached {
            Some(value) => Ok(content::Json(value)),
            None => {
                let resp_string = serde_json::to_string(&self.generate()?)?;
                cache.create(&cache_key, &resp_string, self.duration);
                Ok(content::Json(resp_string))
            }
        }
    }
}

pub struct RequestCached {
    database: Database,
    pub url: String,
    pub request_timeout: u64,
    pub cache_duration: usize,
    pub error_cache_duration: usize,
    pub cache_all_errors: bool,
}

impl RequestCached {
    pub fn new() -> Self {
        RequestCached {
            database: Database::Default,
            url: String::new(),
            request_timeout: 0,
            cache_duration: request_cache_duration(),
            error_cache_duration: request_error_cache_timeout(),
            cache_all_errors: false,
        }
    }

    pub fn database(&mut self, database: Database) -> &mut Self {
        self.database = database;
        self
    }

    pub fn url(&mut self, url: String) -> &mut Self {
        self.url = url;
        self
    }

    pub fn request_timeout(&mut self, request_timeout: u64) -> &mut Self {
        self.request_timeout = request_timeout;
        self
    }

    pub fn cache_duration(&mut self, cache_duration: usize) -> &mut Self {
        self.cache_duration = cache_duration;
        self
    }

    pub fn error_cache_duration(&mut self, error_cache_duration: usize) -> &mut Self {
        self.error_cache_duration = error_cache_duration;
        self
    }

    pub fn cache_all_errors(&mut self, cache_all_errors: bool) -> &mut Self {
        self.cache_all_errors = cache_all_errors;
        self
    }

    pub fn execute(
        &self,
        client: &reqwest::blocking::Client,
        cache: &impl Cache,
    ) -> ApiResult<String> {
        cache.request_cached_op(&client, self)
    }
}
