use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket::State;
use rocket::http::uri::Origin;

use crate::utils::cache::{ServiceCache, Cache};
use crate::config::scheme;

pub struct Context<'a, 'r> {
    request: &'a Request<'r>,
    cache: ServiceCache,
}

impl<'a, 'r> Context<'a, 'r> {
    fn get<T: FromRequest<'a, 'r>>(&self) -> T {
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
        self.request.headers().get_one("Host").map(|host| {
            format!("{}://{}", scheme(), host)
        })
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Context<'a, 'r> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let cache = request.guard().unwrap();
        return Outcome::Success(Context { request, cache });
    }
}