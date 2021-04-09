use crate::config::{request_cache_duration, request_error_cache_timeout};
use crate::utils::errors::{ApiError, ApiResult};
use serde::Serialize;

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
    pub timeout: usize,
    pub resp_generator: Box<dyn FnMut() -> ApiResult<R> + 'a>,
}

impl<'a, R> CacheResponse<'a, R>
where
    R: Serialize,
{
    pub fn new() -> Self {
        CacheResponse {
            database: Database::Default,
            key: String::new(),
            timeout: request_cache_duration(),
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

    pub fn timeout(&mut self, timeout: usize) -> &mut Self {
        self.timeout = timeout;
        self
    }

    pub fn resp_generator(
        &mut self,
        resp_generator: impl FnMut() -> ApiResult<R> + 'a,
    ) -> &mut Self {
        self.resp_generator = Box::new(resp_generator);
        self
    }

    pub fn generate(&mut self) -> ApiResult<R> {
        (self.resp_generator)()
    }
}

pub struct CacheRequest {
    database: Database,
    url: String,
    request_timeout: u64,
    cache_duration: usize,
    error_cache_duration: usize,
    cache_all_errors: bool,
}

impl CacheRequest {
    pub fn new() -> Self {
        CacheRequest {
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
}
