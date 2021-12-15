use std::sync::Arc;

use crate::cache::cache_operations::RequestCached;
use crate::cache::redis::create_service_cache;
use crate::cache::Cache;
use crate::utils::context::RequestContext;
use crate::utils::errors::{ApiError, ErrorDetails};
use crate::utils::http_client::{HttpClient, MockHttpClient, Response};
use serde_json::json;

#[rocket::async_test]
async fn error_from_backend_deserialization() {
    let request_uri = "some.url";
    let error_json = json!({
        "code": 1,
        "message": "Checksum address validation failed",
        "arguments": [
            "0xD6f5Bef6bb4acD235CF85c0ce196316d10785d67"
        ]
    });

    let mut mock_http_client = MockHttpClient::new();
    mock_http_client.expect_get().times(1).returning(move |_| {
        Err(ApiError::from_http_response(&Response {
            body: error_json.to_string(),
            status_code: 422,
        }))
    });
    let cache = Arc::new(create_service_cache()) as Arc<dyn Cache>;

    let context = RequestContext::setup_for_test(
        String::from(request_uri),
        "host".to_string(),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
    );
    let expected = Err(ApiError::new(
        422,
        serde_json::from_value::<ErrorDetails>(json!({
            "code": 1,
            "message": "Checksum address validation failed",
            "arguments": [
                "0xD6f5Bef6bb4acD235CF85c0ce196316d10785d67"
            ]
        }))
        .unwrap(),
    ));

    let request = RequestCached::new_from_context(String::from(request_uri), &context);
    let actual = request.execute().await;

    assert_eq!(expected, actual);
}
