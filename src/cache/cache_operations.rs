use crate::cache::cache_op_executors::{cache_response, invalidate, request_cached};
use crate::cache::Cache;
use crate::config::{
    default_request_timeout, request_cache_duration, request_error_cache_duration,
};
use crate::utils::errors::ApiResult;
use rocket::futures::future::BoxFuture;
use rocket::futures::FutureExt;
use rocket::response::content;
use serde::Serialize;
use std::future::Future;

pub enum Database {
    Info = 1,
    Default = 2,
}

pub struct Invalidate {
    pattern: InvalidationPattern,
    database: Database,
}

pub enum InvalidationPattern {
    FlushAll,
    RequestsResponses(String),
}

impl Invalidate {
    pub fn new(pattern: InvalidationPattern) -> Self {
        Invalidate {
            pattern,
            database: Database::Default,
        }
    }

    fn database(&mut self, database: Database) -> &mut Self {
        self.database = database;
        self
    }

    pub fn execute(&self, cache: &impl Cache) {
        invalidate(cache, &self.pattern)
    }
}

pub struct CacheResponse<'a, R>
where
    R: Serialize,
{
    database: Database,
    pub key: String,
    pub duration: usize,
    pub resp_generator: Option<Box<dyn Fn() -> BoxFuture<'a, ApiResult<R>> + Send + Sync + 'a>>,
}

impl<'a, R> CacheResponse<'a, R>
where
    R: Serialize,
{
    pub fn new(key: String) -> Self {
        CacheResponse {
            key,
            database: Database::Default,
            duration: request_cache_duration(),
            resp_generator: None,
        }
    }

    pub fn database(&mut self, database: Database) -> &mut Self {
        self.database = database;
        self
    }

    pub fn duration(&mut self, duration: usize) -> &mut Self {
        self.duration = duration;
        self
    }

    pub fn resp_generator<F, Fut>(&mut self, resp_generator: F) -> &mut Self
    where
        F: Fn() -> Fut + Send + Sync + 'a,
        Fut: Future<Output = ApiResult<R>> + Send + 'a,
    {
        self.resp_generator = Some(Box::new(|| resp_generator().boxed()));
        self
    }

    pub async fn generate(&self) -> ApiResult<R> {
        (self.resp_generator.unwrap())().await
    }

    pub async fn execute(&self, cache: &impl Cache) -> ApiResult<content::Json<String>> {
        cache_response(cache, self)
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
    pub fn new(url: String) -> Self {
        RequestCached {
            database: Database::Default,
            url,
            request_timeout: default_request_timeout(),
            cache_duration: request_cache_duration(),
            error_cache_duration: request_error_cache_duration(),
            cache_all_errors: false,
        }
    }

    pub fn database(&mut self, database: Database) -> &mut Self {
        self.database = database;
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

    pub fn cache_all_errors(&mut self) -> &mut Self {
        self.cache_all_errors = true;
        self
    }

    pub async fn execute(&self, client: &reqwest::Client, cache: &dyn Cache) -> ApiResult<String> {
        assert!(self.request_timeout > 0);
        request_cached(cache, &client, self).await
    }
}
