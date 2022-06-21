use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::okapi::openapi3::{
    Object, SecurityRequirement, SecurityScheme, SecuritySchemeData,
};
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use serde::Serialize;

use crate::config::webhook_token;

#[derive(Serialize, Debug, JsonSchema)]
pub struct AuthorizationToken {
    value: String,
}

#[derive(Debug)]
pub enum AuthorizationError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthorizationToken {
    type Error = AuthorizationError;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        match request.headers().get_one("Authorization") {
            // Require the header to be present
            None => Outcome::Failure((Status::BadRequest, AuthorizationError::Missing)),
            Some(key) if key == format!("Basic {}", webhook_token()) => {
                Outcome::Success(AuthorizationToken {
                    value: key.to_string(),
                })
            }
            // If the Authorization header didn't match with "Basic <token>" we consider it to be
            // an invalid token
            Some(_) => Outcome::Failure((Status::Unauthorized, AuthorizationError::Invalid)),
        }
    }
}
