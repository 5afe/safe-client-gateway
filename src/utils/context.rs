use crate::cache::Cache;
use crate::config::scheme;
use crate::utils::http_client::HttpClient;
use rocket::request::{self, FromRequest, Request};
use std::sync::Arc;

pub struct RequestContext {
    pub request_id: String,
    pub host: String,
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

    #[cfg(test)]
    pub fn new(
        request_id: String,
        host: String,
        http_client: &Arc<dyn HttpClient>,
        cache: &Arc<dyn Cache>,
    ) -> Self {
        RequestContext {
            request_id,
            host,
            http_client: http_client.clone(),
            cache: cache.clone(),
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
        let host = format!("{}://{}", scheme(), host.to_string());

        return request::Outcome::Success(RequestContext {
            request_id: uri,
            host,
            cache,
            http_client,
        });
    }
}
