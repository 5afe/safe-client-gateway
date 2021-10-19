use crate::utils::context::RequestContext;
use crate::utils::http_client::*;
use core::time::Duration;

#[rocket::async_test]
async fn testing_mocked_http_client() {
    let response_json = "{\"valid\":\"json\"}";

    let mut mock_info_provider = MockHttpClient::new();
    mock_info_provider
        .expect_get()
        .times(1)
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(response_json),
            })
        });

    let request_context = RequestContext::mock("request_id".to_string(), mock_info_provider);
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
    assert_eq!(response_json, actual.body);
}
