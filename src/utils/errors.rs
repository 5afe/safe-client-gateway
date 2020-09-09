use std::fmt;
use thiserror::Error;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::{ContentType, Status};
use serde_json;
use serde::{Serialize, Deserialize};
use std::io::Cursor;
use anyhow::Result;

pub type ApiResult<T, E = ApiError> = Result<T, E>;

#[derive(Error, Debug)]
pub struct ApiError {
    pub status: u16,
    pub details: ErrorDetails,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct ErrorDetails {
    pub code: u64,
    pub message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<String>>,
}

impl ApiError {
    pub fn from_backend_error(status_code: u16, raw_error: &str) -> Self {
        let error_details = match serde_json::from_str::<ErrorDetails>(&raw_error) {
            Ok(backend_error) => backend_error,
            Err(_) => ErrorDetails {
                code: 42,
                message: Some(raw_error.to_owned()),
                arguments: None,
            },
        };
        Self::new(status_code, error_details)
    }

    fn new(status_code: u16, message: ErrorDetails) -> Self {
        Self { status: status_code, details: message }
    }

    fn new_internal(message: String) -> Self {
        Self::new(500, ErrorDetails {
            code: 1337,
            message: Some(message),
            arguments: None,
        })
    }
}

impl fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ApiErrorMessage: code:{:?}; message:{:?}; arguments:{:?}",
               &self.code,
               &self.message,
               &self.arguments
        )
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiError({}: {})", self.status, self.details)
    }
}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, _: &Request) -> response::Result<'r> {
        Response::build()
            .sized_body(Cursor::new(serde_json::to_string(&self.details).expect("Couldn't serialize error")))
            .header(ContentType::JSON)
            .status(Status::from_code(self.status).expect("Unknown status code"))
            .ok()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        Self::new_internal(format!("{:?}", err))
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        Self::new_internal(format!("{:?}", err))
    }
}

impl From<serde_json::error::Error> for ApiError {
    fn from(err: serde_json::error::Error) -> Self {
        Self::new_internal(format!("{:?}", err))
    }
}