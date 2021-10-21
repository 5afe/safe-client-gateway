use crate::cache::MockCache;
use crate::utils::context::RequestContext;
use crate::utils::http_client::{MockHttpClient, Request, Response};

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
                body: String::from(response_json),
            })
        });

    let mock_cache = MockCache::new();

    let request_context = RequestContext::mock(
        "request_id".to_string(),
        "host".to_string(),
        mock_http_client,
        mock_cache,
    );
    let request = Request::new("https://example.com".to_string());

    let actual = request_context
        .http_client()
        .get(request)
        .await
        .expect("response error");
    assert_eq!(response_json, actual.body);
}

#[rocket::async_test]
async fn testing_mocked_cache() {}
