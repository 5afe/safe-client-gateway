use rocket::request::{self, FromRequest, Request};
use rocket::Outcome;
use rocket::State;

use crate::cache::{ServiceCache};
use std::borrow::Borrow;
use crate::config::scheme;

pub struct Context<'a, 'r> {
    request: &'a Request<'r>
}

impl<'a, 'r> Context<'a, 'r> {
    fn get<T: FromRequest<'a, 'r>>(&self) -> T {
        self.request.guard::<T>().unwrap()
    }

    pub fn client(&self) -> &'r reqwest::blocking::Client {
        self.get::<State<reqwest::blocking::Client>>().inner()
    }

    pub fn cache(&self) -> ServiceCache {
        self.get::<ServiceCache>()
    }

    pub fn host(&self) -> Option<String> {
        self.request.headers().get_one("Host").map(|host| {
            if host.starts_with("localhost:") {
                String::from(host)
            } else {
                format!("{}://{}", scheme(), host)
            }
        })
    }

    pub fn origin(&self) -> String {
        self.request.uri().to_string()
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Context<'a, 'r> {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        return Outcome::Success(Context { request });
    }
}