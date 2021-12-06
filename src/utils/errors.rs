use crate::config::log_all_error_responses;
use crate::utils::http_client::Response as HttpClientResponse;
use reqwest::StatusCode;
use rocket::http::{ContentType, Status};
use rocket::request::Request;
use rocket::response::{self, Responder, Response};
use rocket::serde::json::Error;
use serde::{Deserialize, Serialize};
use serde_json::{self, value::Value};
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub debug: Option<Value>,
}

impl ApiError {
    pub fn from_backend_error(status_code: u16, raw_error: &str) -> Self {
        let error_details = match serde_json::from_str::<ErrorDetails>(&raw_error) {
            Ok(backend_error) => backend_error,
            Err(_) => ErrorDetails {
                code: 42,
                message: Some(raw_error.to_owned()),
                arguments: None,
                debug: None,
            },
        };
        Self::new(status_code, error_details)
    }

    pub fn from_http_response(response: &HttpClientResponse) -> Self {
        Self::new_from_message_with_code(response.status_code, response.body.to_string())
    }

    pub fn new_from_message(message: impl Into<String>) -> Self {
        Self::new(
            500,
            ErrorDetails {
                code: 1337,
                message: Some(message.into()),
                arguments: None,
                debug: None,
            },
        )
    }

    pub fn new_from_message_with_debug(message: impl Into<String>, debug: Option<Value>) -> Self {
        Self::new(
            500,
            ErrorDetails {
                code: 1337,
                message: Some(message.into()),
                arguments: None,
                debug,
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
                debug: None,
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

impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, request: &'r Request<'_>) -> response::Result<'static> {
        if log_all_error_responses() || (self.status >= 500 && self.status < 600) {
            log::error!(
                "ERR::{}::{}::{}",
                self.status,
                request.uri().to_string(),
                self.details
            );
        }
        let resp = serde_json::to_string(&self.details).unwrap_or(String::from(
            &self
                .details
                .message
                .unwrap_or("No message error from backend".to_string()),
        ));
        Response::build()
            .sized_body(resp.len(), Cursor::new(resp))
            .header(ContentType::JSON)
            .status(Status::from_code(self.status).unwrap_or(Status::new(self.status)))
            .ok()
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        // We first check if err.is_timeout because in case of timeout the default error code is 500
        // However we want to map it to a GATEWAY_TIMEOUT (504)
        let status_code = if err.is_timeout() {
            StatusCode::GATEWAY_TIMEOUT
        } else {
            err.status().unwrap_or(StatusCode::INTERNAL_SERVER_ERROR)
        };
        Self::new_from_message_with_code(status_code.as_u16(), format!("{:?}", err))
    }
}

impl From<serde_json::error::Error> for ApiError {
    fn from(err: serde_json::error::Error) -> Self {
        Self::new_from_message(format!("{:?}", err))
    }
}

impl From<rocket::serde::json::Error<'_>> for ApiError {
    fn from(err: Error<'_>) -> Self {
        let message = match err {
            Error::Io(_) => String::from("Request deserialize IO error"),
            Error::Parse(_request_json, json_error) => json_error.to_string(),
        };
        Self::new_from_message_with_code(422, message)
    }
}
