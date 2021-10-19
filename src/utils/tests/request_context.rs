use crate::cache::MockCache;
use crate::utils::context::RequestContext;
use crate::utils::http_client::{MockHttpClient, Request, Response};
use core::time::Duration;

#[rocket::async_test]
async fn testing_mocked_http_client() {
    let response_json = "{\"valid\":\"json\"}";

    let mut mock_http_client = MockHttpClient::new();
    mock_http_client
        .expect_get()
        .times(1)
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: Some(String::from(response_json)),
            })
        });

    let mock_cache = MockCache::new();

    let request_context =
        RequestContext::mock("request_id".to_string(), mock_http_client, mock_cache);
    let request = Request {
        url: "https://example.com".to_string(),
        body: None,
        timeout: Duration::from_millis(0),
    };
    let actual = request_context
        .http_client
        .get(&request)
        .await
        .expect("response error");
    assert_eq!(response_json, actual.body.unwrap());
}

#[rocket::async_test]
async fn testing_mocked_cache() {}
