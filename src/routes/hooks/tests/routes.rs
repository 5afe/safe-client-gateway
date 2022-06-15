use std::env;

use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use rocket::serde::json::json;

use crate::tests::main::setup_rocket;
use crate::utils::http_client::MockHttpClient;

#[rocket::async_test]
async fn post_hooks_events_no_token_set() {
    let mock_http_client = MockHttpClient::new();
    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::post_hooks_events],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");

    let request = client
        .post("/v1/chains/1/hooks/events")
        .body(&json!({"address": "0x6810e776880C02933D47DB1b9fc05908e5386b96"}).to_string())
        .header(ContentType::JSON)
        .header(Header::new("Host", "test.gnosis.io"));
    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[rocket::async_test]
async fn post_hooks_events_invalid_token() {
    env::set_var("WEBHOOK_TOKEN", "test_webhook_token");
    let mock_http_client = MockHttpClient::new();
    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::post_hooks_events],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");

    let request = client
        .post("/v1/chains/1/hooks/events")
        .body(&json!({"address": "0x6810e776880C02933D47DB1b9fc05908e5386b96"}).to_string())
        .header(ContentType::JSON)
        .header(Header::new("Authorization", "Basic some_token"))
        .header(Header::new("Host", "test.gnosis.io"));
    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[rocket::async_test]
async fn post_hooks_events_valid_token() {
    env::set_var("WEBHOOK_TOKEN", "test_webhook_token");
    let mock_http_client = MockHttpClient::new();
    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::post_hooks_events],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");

    let request = client
        .post("/v1/chains/1/hooks/events")
        .body(
            &json!({"address": "0x6810e776880C02933D47DB1b9fc05908e5386b96", "chainId" : "1"})
                .to_string(),
        )
        .header(ContentType::JSON)
        .header(Header::new("Authorization", "Basic test_webhook_token"))
        .header(Header::new("Host", "test.gnosis.io"));
    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::Ok);
}

#[rocket::async_test]
async fn post_flush_events_no_token_set() {
    let mock_http_client = MockHttpClient::new();
    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::post_flush_events],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");

    let request = client
        .post("/v2/flush")
        .header(ContentType::JSON)
        .header(Header::new("Host", "test.gnosis.io"));
    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::BadRequest);
}

#[rocket::async_test]
async fn post_flush_events_invalid_token() {
    env::set_var("WEBHOOK_TOKEN", "test_webhook_token");
    let mock_http_client = MockHttpClient::new();
    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::post_flush_events],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");

    let request = client
        .post("/v2/flush")
        .header(ContentType::JSON)
        .header(Header::new("Host", "test.gnosis.io"))
        .header(Header::new("Authorization", "Basic some_token"));
    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::Unauthorized);
}

#[rocket::async_test]
async fn post_flush_events_valid_token() {
    env::set_var("WEBHOOK_TOKEN", "test_webhook_token");
    let mock_http_client = MockHttpClient::new();
    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::post_flush_events],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");

    let request = client
        .post("/v2/flush")
        .body(&json!({"invalidate": "Chains"}).to_string())
        .header(ContentType::JSON)
        .header(Header::new("Host", "test.gnosis.io"))
        .header(Header::new("Authorization", "Basic test_webhook_token"));
    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::Ok);
}
