use std::collections::HashMap;
use std::future::Future;
use std::sync::Arc;

use rocket::futures::future::BoxFuture;
use rocket::futures::FutureExt;
use rocket::response::content;
use serde::{Deserialize, Serialize};

use crate::cache::cache_op_executors::{cache_response, invalidate, request_cached};
use crate::cache::manager::ChainCache;
use crate::cache::{Cache, CACHE_REQS_PREFIX, CACHE_REQS_RESP_PREFIX, CACHE_RESP_PREFIX};
use crate::config::{
    base_config_service_uri, default_request_timeout, request_cache_duration,
    request_error_cache_duration,
};
use crate::providers::info::generate_token_key;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::HttpClient;

pub struct Invalidate {
    pub(super) cache: Arc<dyn Cache>,
    pattern: InvalidationPattern,
}

#[derive(Deserialize, Debug)]
pub enum InvalidationScope {
    Requests,
    Responses,
    Both,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "invalidate", content = "pattern_details")]
pub enum InvalidationPattern {
    Any(InvalidationScope, String),
    Transactions(InvalidationScope, String),
    Balances(InvalidationScope, String),
    Collectibles(InvalidationScope, String),
    Transfers(InvalidationScope, String),
    Chains,
    Contracts,
    Tokens { chain_id: String },
}

impl InvalidationPattern {
    pub(super) fn to_pattern_string(&self) -> String {
        match &self {
            InvalidationPattern::Any(scope, value) => {
                format!("{}*{}*", scope.invalidation_scope_string(), &value)
            }
            InvalidationPattern::Balances(scope, value) => {
                format!("{}*/{}/balances*", scope.invalidation_scope_string(), value)
            }
            InvalidationPattern::Collectibles(scope, value) => {
                format!(
                    "{}*/{}/collectibles*",
                    scope.invalidation_scope_string(),
                    value
                )
            }
            InvalidationPattern::Transfers(scope, value) => {
                format!(
                    "{}*/{}/*transfer*",
                    scope.invalidation_scope_string(),
                    value
                )
            }
            InvalidationPattern::Transactions(scope, value) => {
                format!(
                    "{}*/{}/*transactions/*",
                    scope.invalidation_scope_string(),
                    value
                )
            }
            InvalidationPattern::Contracts => String::from("*contract*"),
            InvalidationPattern::Tokens { chain_id } => generate_token_key(chain_id),
            InvalidationPattern::Chains => {
                format!("*{}*", base_config_service_uri())
            }
        }
    }
}

impl InvalidationScope {
    pub(super) fn invalidation_scope_string(&self) -> &str {
        match &self {
            InvalidationScope::Requests => CACHE_REQS_PREFIX,
            InvalidationScope::Responses => CACHE_RESP_PREFIX,
            InvalidationScope::Both => CACHE_REQS_RESP_PREFIX,
        }
    }
}

impl Invalidate {
    pub fn new(pattern: InvalidationPattern, cache: Arc<dyn Cache>) -> Self {
        Invalidate { cache, pattern }
    }

    pub async fn execute(&self) {
        invalidate(self.cache.clone(), &self.pattern).await
    }
}

pub struct CacheResponse<'a, R>
where
    R: Serialize,
{
    pub(super) cache: Arc<dyn Cache>,
    pub key: String,
    pub duration: usize,
    // "dyn" allows setting the type of the BoxFuture to different times in runtime
    pub resp_generator: Option<Box<dyn Fn() -> BoxFuture<'a, ApiResult<R>> + Send + Sync + 'a>>,
}

impl<'a, R> CacheResponse<'a, R>
where
    R: Serialize,
{
    pub fn new(context: &RequestContext, chain_cache: ChainCache) -> Self {
        CacheResponse {
            key: context.request_id.to_string(),
            cache: context.cache(chain_cache),
            duration: request_cache_duration(),
            resp_generator: None,
        }
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
        self.resp_generator = Some(Box::new(move || resp_generator().boxed()));
        self
    }

    pub async fn generate(&self) -> ApiResult<R> {
        (self.resp_generator.as_ref().unwrap())().await
    }

    pub async fn execute(&self) -> ApiResult<content::RawJson<String>> {
        cache_response(self).await
    }
}

pub struct RequestCached {
    pub(super) client: Arc<dyn HttpClient>,
    pub(super) cache: Arc<dyn Cache>,
    pub url: String,
    pub request_timeout: u64,
    pub cache_duration: usize,
    pub error_cache_duration: usize,
    pub cache_all_errors: bool,
    pub headers: HashMap<String, String>,
}

impl RequestCached {
    pub fn new(url: String, client: &Arc<dyn HttpClient>, cache: &Arc<dyn Cache>) -> Self {
        RequestCached {
            client: client.clone(),
            cache: cache.clone(),
            url,
            request_timeout: default_request_timeout(),
            cache_duration: request_cache_duration(),
            error_cache_duration: request_error_cache_duration(),
            cache_all_errors: false,
            headers: HashMap::default(),
        }
    }

    pub fn new_from_context(
        url: String,
        context: &RequestContext,
        chain_cache: ChainCache,
    ) -> Self {
        RequestCached {
            client: context.http_client(),
            cache: context.cache(chain_cache),
            url,
            request_timeout: default_request_timeout(),
            cache_duration: request_cache_duration(),
            error_cache_duration: request_error_cache_duration(),
            cache_all_errors: false,
            headers: HashMap::new(),
        }
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

    pub fn add_header(&mut self, header: (&str, &str)) -> &mut Self {
        self.headers
            .insert(String::from(header.0), String::from(header.1));
        self
    }

    pub async fn execute(&self) -> ApiResult<String> {
        assert!(self.request_timeout > 0);
        request_cached(self).await
    }
}
