use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;

use crate::config::webhook_token;

pub struct AuthorizationToken {
    value: String,
}

#[derive(Debug)]
pub enum AuthorizationError {
    Missing,
    Invalid,
    BadCount,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizationToken {
    type Error = AuthorizationError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let keys: Vec<&str> = request.headers().get("Authorization").collect();

        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, AuthorizationError::Missing)),
            1 if keys[0] != webhook_token() => Outcome::Success(AuthorizationToken {
                value: keys[0].to_string(),
            }),
            1 => Outcome::Failure((Status::Unauthorized, AuthorizationError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, AuthorizationError::BadCount)),
        }
    }
}
