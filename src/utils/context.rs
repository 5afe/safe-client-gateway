use rocket::http::uri::Origin;
use rocket::request::{self, FromRequest, Request};
use rocket::State;

use crate::cache::redis::ServiceCache;
use crate::cache::Cache;
use crate::config::scheme;

pub struct Context<'r> {
    uri: String,
    host: Option<String>,
    cache: ServiceCache<'r>,
    client: State<'r, reqwest::Client>,
}

impl<'r> Context<'r> {
    pub fn client(&self) -> &'r reqwest::Client {
        self.client.inner()
    }

    // TODO: we would want to return the Cache trait here
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
        let client = try_outcome!(request.guard::<State<reqwest::Client>>().await);
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
