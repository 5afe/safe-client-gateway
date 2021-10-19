use crate::cache::redis::ServiceCache;
use crate::config::scheme;
use crate::utils::http_client::*;
use rocket::http::uri::Origin;
use rocket::request::{self, FromRequest, Request};
use std::sync::Arc;

pub struct Context<'r> {
    uri: String,
    host: Option<String>,
    cache: ServiceCache<'r>,
    client: &'r reqwest::Client,
}

pub struct RequestContext {
    pub request_id: String, // this will be host+uri , will be used for cache keys
    pub http_client: Arc<dyn HttpClient>,
    // pub cache : Arc<dyn Cache>
}

#[cfg(test)]
impl RequestContext {
    pub fn mock(request_id: String, mock_http_client: MockHttpClient) -> Self {
        RequestContext {
            request_id,
            http_client: Arc::new(mock_http_client),
        }
    }
}

impl<'r> Context<'r> {
    pub fn client(&self) -> &'r reqwest::Client {
        self.client
    }

    pub fn cache(&self) -> &ServiceCache<'r> {
        &self.cache
    }

    pub fn uri(&self) -> String {
        self.uri.clone()
    }

    pub fn build_absolute_url(&self, origin: Origin) -> String {
        format!("{}{}", self.host().unwrap(), origin)
    }

    fn host(&self) -> Option<String> {
        self.host
            .as_ref()
            .map(|host| format!("{}://{}", scheme(), host))
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Context<'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cache = request.guard().await.unwrap();
        let client = request.rocket().state::<reqwest::Client>().unwrap();
        // TODO: I couldn't get the request to be part of the context ... not sure if we want that for the future
        let host = request
            .headers()
            .get_one("Host")
            .map(|host| host.to_string());
        let uri = request.uri().to_string();
        return request::Outcome::Success(Context {
            host,
            uri,
            cache,
            client,
        });
    }
}
