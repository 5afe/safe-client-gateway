use std::fmt;
use thiserror::Error;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::{ContentType, Status};
use serde_json;
use serde::{Serialize, Deserialize};
use std::io::Cursor;
use log::debug;
use anyhow::Result;

pub type ApiResult<T, E = ApiError> = Result<T, E>;

#[derive(Error, Debug, Serialize)]
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
    pub fn from_backend_error(status_code: u16, raw_error: &str) -> ApiError {
        match serde_json::from_str::<ErrorDetails>(&raw_error) {
            Ok(backend_error) => {
                ApiError::new(status_code, backend_error)
            }
            Err(error) => {
                let error_details = ErrorDetails {
                    code: 1337,
                    message: Some(format!("Serde serialization error: {}", error.to_string())),
                    arguments: None,
                };
                ApiError::new(status_code, error_details)
            }
        }
    }

    fn new(status_code: u16, message: ErrorDetails) -> ApiError {
        ApiError { status: status_code, details: message }
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
        debug!("Handle ApiError");
        Response::build()
            .sized_body(Cursor::new(serde_json::to_string(&self).expect("Couldn't serialize error")))
            .header(ContentType::JSON)
            .status(Status::from_code(self.status).expect("Unknown status code"))
            .ok()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        Self {
            status: 500,
            details: ErrorDetails {
                code: 42,
                message: Some(format!("{:?}", err)),
                arguments: None,
            },
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        Self {
            status: 500,
            details: ErrorDetails {
                code: 42,
                message: Some(format!("{:?}", err)),
                arguments: None,
            },
        }
    }
}

impl From<serde_json::error::Error> for ApiError {
    fn from(err: serde_json::error::Error) -> Self {
        Self {
            status: 500,
            details: ErrorDetails {
                code: 1337,
                message: Some(format!("{:?}", err)),
                arguments: None,
            },
        }
    }
}