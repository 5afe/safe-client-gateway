use crate::utils::errors::ApiResult;
use core::time::Duration;
use mockall::automock;

pub struct Request {
    pub url: String,
    pub body: Option<String>,
    pub timeout: Duration,
}

pub struct Response {
    pub body: Option<String>,
    pub status_code: u16,
}

#[automock]
#[rocket::async_trait]
pub trait HttpClient: Send + Sync {
    async fn get(&self, request: &Request) -> ApiResult<Response>;
}

#[rocket::async_trait]
#[cfg(not(test))]
impl HttpClient for reqwest::Client {
    async fn get(&self, request: &Request) -> ApiResult<Response> {
        let response = self
            .get(&request.url)
            .timeout(request.timeout)
            .send()
            .await?;
        let status_code = response.status().as_u16();
        let body = response.text().await?;
        Ok(Response { body, status_code })
    }
}
