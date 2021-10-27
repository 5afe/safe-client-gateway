use crate::cache::manager::CacheManager;
use crate::cache::Cache;
use crate::config::scheme;
use crate::utils::http_client::HttpClient;
use rocket::request::{self, FromRequest, Request};
use std::sync::Arc;

pub struct RequestContext {
    pub request_id: String,
    pub host: String,
    http_client: Arc<dyn HttpClient>,
    cache_manager: Arc<dyn CacheManager>,
}

impl RequestContext {
    pub fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    pub fn default_cache(&self) -> Arc<dyn Cache> {
        self.cache_manager.default_cache()
    }

    pub fn info_cache(&self) -> Arc<dyn Cache> {
        self.cache_manager.info_cache()
    }
}

#[cfg(test)]
impl RequestContext {
    pub fn mock(
        request_id: String,
        host: String,
        mock_http_client: crate::utils::http_client::MockHttpClient,
        mock_cache: crate::cache::MockCache,
        mock_cache_manager: crate::cache::manager::MockCacheManager,
    ) -> Self {
        RequestContext {
            request_id,
            host,
            http_client: Arc::new(mock_http_client),
            cache_manager: Arc::new(mock_cache_manager),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RequestContext {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let http_client = request
            .rocket()
            .state::<Arc<dyn HttpClient>>()
            .expect("HttpClient unavailable. Is it added to rocket instance?")
            .clone();
        let cache_manager = request
            .rocket()
            .state::<Arc<dyn CacheManager>>()
            .expect("CacheManager unavailable. Is it added to rocket instance?")
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
            http_client,
            cache_manager,
        });
    }
}
