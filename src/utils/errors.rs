use crate::config::log_all_error_responses;
use reqwest::blocking::Response as ReqwestResponse;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket_contrib::json::JsonError;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt;
use std::io::Cursor;
use std::result::Result;
use thiserror::Error;

pub type ApiResult<T, E = ApiError> = Result<T, E>;

#[derive(Error, Debug, PartialEq)]
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

    pub fn from_http_response(response: ReqwestResponse, default_message: String) -> Self {
        Self::new_from_message_with_code(
            response.status().as_u16(),
            response.text().unwrap_or(default_message),
        )
    }

    pub fn new_from_message(message: impl Into<String>) -> Self {
        Self::new(
            500,
            ErrorDetails {
                code: 1337,
                message: Some(message.into()),
                arguments: None,
            },
        )
    }

    pub fn new_from_message_with_code(status_code: u16, message: String) -> Self {
        Self::new(
            status_code,
            ErrorDetails {
                code: 1337,
                message: Some(message),
                arguments: None,
            },
        )
    }

    fn new(status_code: u16, message: ErrorDetails) -> Self {
        Self {
            status: status_code,
            details: message,
        }
    }
}

impl fmt::Display for ErrorDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ApiErrorMessage: code:{:?}; message:{:?}; arguments:{:?}",
            &self.code, &self.message, &self.arguments
        )
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiError({}: {})", self.status, self.details)
    }
}

impl<'r> Responder<'r> for ApiError {
    fn respond_to(self, request: &Request) -> response::Result<'r> {
        if log_all_error_responses() || (self.status >= 500 && self.status < 600) {
            log::error!(
                "ERR::{}::{}::{}",
                self.status,
                request.uri().to_string(),
                self.details
            );
        }
        Response::build()
            .sized_body(Cursor::new(
                serde_json::to_string(&self.details).unwrap_or(String::from(
                    &self
                        .details
                        .message
                        .unwrap_or("No message error from backend".to_string()),
                )),
            ))
            .header(ContentType::JSON)
            .status(
                Status::from_code(self.status)
                    .unwrap_or(Status::new(self.status, "Unknown status code")),
            )
            .ok()
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        Self::new_from_message(format!("{:?}", err))
    }
}

impl From<serde_json::error::Error> for ApiError {
    fn from(err: serde_json::error::Error) -> Self {
        Self::new_from_message(format!("{:?}", err))
    }
}

impl From<rocket_contrib::json::JsonError<'_>> for ApiError {
    fn from(err: JsonError<'_>) -> Self {
        let message = match err {
            JsonError::Io(_) => String::from("Request deserialize IO error"),
            JsonError::Parse(_request_json, json_error) => json_error.to_string(),
        };
        Self::new_from_message_with_code(422, message)
    }
}
