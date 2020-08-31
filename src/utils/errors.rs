use std::fmt;
use thiserror::Error;
use rocket::request::Request;
use rocket::response::{self, Response, Responder};
use rocket::http::{ContentType, Status};
use serde_json;
use serde::Serialize;
use std::io::Cursor;
use log::debug;
use anyhow::Result;

pub type ApiResult<T, E = ApiError> = Result<T, E>;

#[derive(Error, Debug, Serialize)]
pub struct ApiError {
    pub status: u16,
    pub message: Option<String>,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ApiError({}: {})", self.status, self.message.as_ref().unwrap_or(&"Unknown error".to_owned()))
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
            message: Some(format!("{:?}", err)),
        }
    }
}

impl From<reqwest::Error> for ApiError {
    fn from(err: reqwest::Error) -> Self {
        Self {
            status: 500,
            message: Some(format!("{:?}", err)),
        }
    }
}

impl From<serde_json::Error> for ApiError {
    fn from(err: serde_json::Error) -> Self {
        Self {
            status: 500,
            message: Some(format!("{:?}", err)),
        }
    }
}