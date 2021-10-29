use crate::config::default_request_timeout;
use crate::utils::errors::ApiResult;
use core::time::Duration;
use mockall::automock;
use reqwest::header::CONTENT_TYPE;

#[derive(PartialEq, Debug)]
pub struct Request {
    url: String,
    body: Option<String>,
    timeout: Duration,
}

impl Request {
    pub fn new(url: String) -> Self {
        Request {
            url,
            body: None,
            timeout: Duration::from_millis(default_request_timeout()),
        }
    }

    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    pub fn body(&mut self, body: Option<String>) -> &mut Self {
        self.body = body;
        self
    }
}

#[derive(PartialEq, Debug)]
pub struct Response {
    pub body: String,
    pub status_code: u16,
}

impl Response {
    pub fn is_server_error(&self) -> bool {
        500 <= self.status_code && self.status_code < 600
    }
    pub fn is_client_error(&self) -> bool {
        400 <= self.status_code && self.status_code < 500
    }
    pub fn is_success(&self) -> bool {
        200 <= self.status_code && self.status_code < 300
    }
}

#[automock]
#[rocket::async_trait]
pub trait HttpClient: Send + Sync {
    async fn get(&self, request: Request) -> ApiResult<Response>;
    async fn post(&self, request: Request) -> ApiResult<Response>;
    async fn delete(&self, request: Request) -> ApiResult<Response>;
}

#[rocket::async_trait]
impl HttpClient for reqwest::Client {
    async fn get(&self, request: Request) -> ApiResult<Response> {
        let response = self
            .get(&request.url)
            .timeout(request.timeout)
            .send()
            .await?;
        let status_code = response.status().as_u16();
        let body = response.text().await?;
        Ok(Response { body, status_code })
    }

    async fn post(&self, request: Request) -> ApiResult<Response> {
        let body = request.body.unwrap_or(String::from(""));
        let response = self
            .post(&request.url)
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .timeout(request.timeout)
            .send()
            .await?;
        let status_code = response.status().as_u16();
        let body = response.text().await?;
        Ok(Response { body, status_code })
    }

    async fn delete(&self, request: Request) -> ApiResult<Response> {
        let body = request.body.unwrap_or(String::from(""));
        let response = self
            .delete(&request.url)
            .header(CONTENT_TYPE, "application/json")
            .body(body)
            .timeout(request.timeout)
            .send()
            .await?;
        let status_code = response.status().as_u16();
        let body = response.text().await?;
        Ok(Response { body, status_code })
    }
}
