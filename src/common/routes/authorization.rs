use rocket::http::Status;
use rocket::request::{FromRequest, Outcome};
use rocket::Request;
use rocket_okapi::okapi::openapi3::{Object, SecurityScheme, SecuritySchemeData, SecurityRequirement};
use serde::Serialize;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{
    gen::OpenApiGenerator,
    request::{OpenApiFromRequest, RequestHeaderInput},
};

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

impl<'a> OpenApiFromRequest<'a> for AuthorizationToken {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> rocket_okapi::Result<RequestHeaderInput> {
        // Setup global requirement for Security scheme
        let security_scheme = SecurityScheme {
            description: Some("Requires an API key to access, key is: `mykey`.".to_owned()),
            // Setup data requirements.
            data: SecuritySchemeData::ApiKey {
                name: "Authorization".to_owned(),
                location: "header".to_owned(),
            },
            extensions: Object::default(),
        };
        // Add the requirement for this route/endpoint
        let mut security_req = SecurityRequirement::new();
        // Each security requirement needs to be met before access is allowed.
        security_req.insert("ApiKeyAuth".to_owned(), Vec::new());
        // These vvvvvvv-----^^^^^^^^^^ values need to match exactly!
        Ok(RequestHeaderInput::Security(
            "ApiKeyAuth".to_owned(),
            security_scheme,
            security_req,
        ))
    }
}