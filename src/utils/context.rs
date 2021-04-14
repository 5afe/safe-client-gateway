use rocket::http::uri::Origin;
use rocket::request::{self, FromRequest, Request};
use rocket::State;

use crate::cache::redis::ServiceCache;
use crate::cache::Cache;
use crate::config::scheme;

pub struct Context<'a, 'r> {
    request: &'a Request<'r>,
    cache: ServiceCache,
}

impl<'a, 'r> Context<'a, 'r> {
    fn get<T: FromRequest<'r>>(&self) -> T {
        self.request.guard::<T>().unwrap()
    }

    pub fn client(&self) -> &'r reqwest::blocking::Client {
        self.get::<State<reqwest::blocking::Client>>().inner()
    }

    pub fn cache(&self) -> &impl Cache {
        &self.cache
    }

    pub fn uri(&self) -> String {
        self.request.uri().to_string()
    }

    pub fn build_absolute_url(&self, origin: Origin) -> String {
        format!("{}{}", self.host().unwrap(), origin)
    }

    fn host(&self) -> Option<String> {
        self.request
            .headers()
            .get_one("Host")
            .map(|host| format!("{}://{}", scheme(), host))
    }
}

#[rocket::async_trait]
impl<'a, 'r> FromRequest<'r> for Context<'a, 'r> {
    type Error = ();

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let cache = request.guard().unwrap();
        return request::Outcome::Success(Context { request, cache });
    }
}
