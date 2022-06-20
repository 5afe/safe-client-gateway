use std::sync::Arc;

use rocket::request::{self, FromRequest, Request};
use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
};

use crate::cache::manager::ChainCache;
use crate::cache::Cache;
use crate::config::scheme;
use crate::utils::http_client::HttpClient;
use crate::RedisCacheManager;

pub struct RequestContext {
    pub request_id: String,
    pub host: String,
    http_client: Arc<dyn HttpClient>,
    cache_manager: Arc<dyn RedisCacheManager>,
}

impl RequestContext {
    pub fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    pub fn cache(&self, chain_cache: ChainCache) -> Arc<dyn Cache> {
        self.cache_manager.cache_for_chain(chain_cache)
    }

    #[cfg(test)]
    pub async fn setup_for_test(
        request_id: String,
        host: String,
        http_client: &Arc<dyn HttpClient>,
        cache_manager: &Arc<dyn RedisCacheManager>,
    ) -> Self {
        cache_manager
            .cache_for_chain(ChainCache::Mainnet)
            .invalidate_pattern("*")
            .await;
        cache_manager
            .cache_for_chain(ChainCache::Other)
            .invalidate_pattern("*")
            .await;

        RequestContext {
            request_id,
            host,
            http_client: http_client.clone(),
            cache_manager: cache_manager.clone(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestContext {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cache_manager = request
            .rocket()
            .state::<Arc<dyn RedisCacheManager>>()
            .expect("RedisCacheManager unavailable. Is it added to rocket instance?")
            .clone();
        let http_client = request
            .rocket()
            .state::<Arc<dyn HttpClient>>()
            .expect("HttpClient unavailable. Is it added to rocket instance?")
            .clone();
        let host = request
            .headers()
            .get_one("Host")
            .expect("Request Host must be available");

        let uri = request.uri().to_string();
        let host = format!("{}://{}", scheme(), host.to_string());

        return request::Outcome::Success(RequestContext {
            request_id: uri,
            host,
            cache_manager,
            http_client,
        });
    }
}


impl<'a> OpenApiFromRequest<'a> for RequestContext {

    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        Ok(RequestHeaderInput::None)
    }
}
