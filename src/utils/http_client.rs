use crate::utils::errors::ApiResult;
use chrono::Duration;
use mockall::automock;

pub struct Request {
    pub url: String,
    pub body: Option<String>,
    pub timeout: Duration,
}

pub struct Response {
    pub body: String,
    pub status_code: u16,
}

#[automock]
#[rocket::async_trait]
pub trait HttpClient: Send + Sync + 'static {
    async fn get(&self, request: &Request) -> ApiResult<Response>;
}

#[rocket::async_trait]
#[cfg(not(test))]
impl HttpClient for reqwest::Client {
    async fn get(&self, request: &Request) -> ApiResult<Response> {
        let response = self.get(&request.url).send().await?;
        let status_code = response.status().as_u16();
        let body = &response.text().await?;
        Ok(Response {
            body: body.to_string(),
            status_code: status_code.to_owned(),
        })
    }
}