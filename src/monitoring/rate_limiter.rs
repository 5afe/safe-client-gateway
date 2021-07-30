use governor::state::keyed::DefaultKeyedStateStore;
use governor::{clock, Quota, RateLimiter};
use nonzero_ext::*;
use rocket::http::Status;
use rocket::request::FromRequest;
use rocket::{request, Request};

pub struct RateLimiterConfig {
    pub rate_limiter: RateLimiter<String, DefaultKeyedStateStore<String>, clock::DefaultClock>,
}
const QUOTA: u32 = 20;

#[derive(Debug)]
pub enum RateLimitError {
    LimitReached,
}

impl RateLimiterConfig {
    pub fn new() -> Self {
        RateLimiterConfig {
            rate_limiter: RateLimiter::keyed(Quota::per_second(nonzero!(QUOTA))),
        }
    }
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for RateLimiterConfig {
    type Error = RateLimitError;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let rate_limiter_config = request.rocket().state::<RateLimiterConfig>().unwrap();
        let rate_limiter = &rate_limiter_config.rate_limiter;

        match rate_limiter.check_key(&request.uri().to_string()) {
            Ok(_) => request::Outcome::Forward(()),
            Err(_) => request::Outcome::Failure((
                Status::from_code(429u16).unwrap(),
                RateLimitError::LimitReached,
            )),
        }
    }
}
