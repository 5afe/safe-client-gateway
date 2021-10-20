use crate::cache::redis::ServiceCache;
use crate::cache::Cache;
use crate::cache::MockCache;
use crate::config::scheme;
use crate::utils::http_client::{HttpClient, MockHttpClient};
use rocket::http::uri::Origin;
use rocket::request::{self, FromRequest, Request};
use std::sync::Arc;

pub struct RequestContext {
    pub request_id: String,
    pub absolute_uri: String,
    http_client: Arc<dyn HttpClient>,
    cache: Arc<dyn Cache>,
}

impl RequestContext {
    pub fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http_client.clone()
    }

    pub fn cache(&self) -> Arc<dyn Cache> {
        self.cache.clone()
    }
}

#[cfg(test)]
impl RequestContext {
    pub fn mock(
        request_id: String,
        absolute_uri: String,
        mock_http_client: MockHttpClient,
        mock_cache: MockCache,
    ) -> Self {
        RequestContext {
            request_id,
            absolute_uri,
            http_client: Arc::new(mock_http_client),
            cache: Arc::new(mock_cache),
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
        let absolute_uri = format!("{}://{}{}", scheme(), host, &uri);

        log::error!("{}", &absolute_uri);
        log::error!("{}", &uri);
        return request::Outcome::Success(RequestContext {
            request_id: uri,
            absolute_uri,
            cache,
            http_client,
        });
    }
}
