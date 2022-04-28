use std::sync::Arc;

use rocket::request::{self, FromRequest, Request};

use crate::cache::Cache;
use crate::config::scheme;
use crate::utils::http_client::HttpClient;
use crate::ServiceCache;

pub(crate) const UNSPECIFIED_CHAIN: &'static str = "UNSPECIFIED";

pub struct RequestContext {
    pub request_id: String,
    pub host: String,
    http_client: Arc<dyn HttpClient>,
    cache: Arc<dyn Cache>,
    mainnet_cache: Arc<ServiceCache>,
}

impl RequestContext {
    pub fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    pub fn cache(&self, chain_id: &str) -> Arc<dyn Cache> {
        match chain_id {
            "0" => self.mainnet_cache.clone(),
            _ => self.cache.clone(),
        }
    }

    #[cfg(test)]
    pub async fn setup_for_test(
        request_id: String,
        host: String,
        http_client: &Arc<dyn HttpClient>,
        cache: &Arc<dyn Cache>,
        chain_cache: &Arc<ServiceCache>,
    ) -> Self {
        cache.invalidate_pattern("*").await;

        RequestContext {
            request_id,
            host,
            http_client: http_client.clone(),
            cache: cache.clone(),
            mainnet_cache: chain_cache.clone(),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestContext {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cache = request
            .rocket()
            .state::<Arc<dyn Cache>>()
            .expect("ServiceCache unavailable. Is it added to rocket instance?")
            .clone();
        let mainnet_cache = request
            .rocket()
            .state::<Arc<ServiceCache>>()
            .expect("Mainnet ServiceCache unavailable. Is it added to rocket instance?")
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
            cache,
            http_client,
            mainnet_cache,
        });
    }
}
