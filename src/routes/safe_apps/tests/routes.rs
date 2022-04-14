use crate::routes::safe_apps::models::SafeApp;
use crate::routes::safe_apps::tests::RESPONSE_SAFE_APPS_WITH_TAGS;
use crate::tests::main::setup_rocket;
use crate::utils::errors::{ApiError, ErrorDetails};
use crate::utils::http_client::{MockHttpClient, Request, Response};
use mockall::predicate::eq;
use rocket::http::{Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;
use std::env;

use super::RESPONSE_SAFE_APPS;

#[rocket::async_test]
async fn safe_apps() {
    env::set_var("SAFE_APPS_TAGS_FEATURE_ENABLED", "false");
    let chain_id = "137";
    let client_url = "https://gnosis-safe.io";

    let mut mock_http_client = MockHttpClient::new();

    let safe_apps_request = Request::new(config_uri!(
        "/v1/safe-apps/?chainId={}&clientUrl={}",
        chain_id,
        client_url
    ));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(safe_apps_request))
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::POLYGON_SAFE_APPS),
            })
        });

    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::get_safe_apps],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");
    let response = {
        let mut response = client.get("/v1/chains/137/safe-apps?client_url=https://gnosis-safe.io");
        response.add_header(Header::new("Host", "test.gnosis.io"));
        response.dispatch().await
    };
    let actual_status = response.status();
    let actual_body = response.into_string().await.unwrap();
    let actual: Vec<SafeApp> = serde_json::from_str(&actual_body).unwrap();
    let expected: Vec<SafeApp> = serde_json::from_str(RESPONSE_SAFE_APPS).unwrap();

    assert_eq!(actual_status, Status::Ok);
    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn safe_apps_not_found() {
    env::set_var("SAFE_APPS_TAGS_FEATURE_ENABLED", "false");
    let chain_id = "4";
    let backend_error_json = json!({"details": "Not found"}).to_string();
    let error = ErrorDetails {
        code: 1337,
        message: Some(backend_error_json.clone()),
        arguments: None,
        debug: None,
    };

    let mut mock_http_client = MockHttpClient::new();

    let safe_apps_request = Request::new(config_uri!(
        "/v1/safe-apps/?chainId={}&clientUrl=",
        chain_id
    ));
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(safe_apps_request))
        .return_once(move |_| {
            Err(ApiError::from_http_response(&Response {
                status_code: 404,
                body: backend_error_json.clone(),
            }))
        });

    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::get_safe_apps],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");
    let response = {
        let mut response = client.get("/v1/chains/4/safe-apps");
        response.add_header(Header::new("Host", "test.gnosis.io"));
        response.dispatch().await
    };

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
        response.into_string().await.unwrap(),
        serde_json::to_string(&error).unwrap()
    );
}

#[rocket::async_test]
async fn safe_apps_tags() {
    env::set_var("SAFE_APPS_TAGS_FEATURE_ENABLED", "true");
    let chain_id = "137";
    let client_url = "https://gnosis-safe.io";
    let mut mock_http_client = MockHttpClient::new();
    let safe_apps_request = Request::new(config_uri!(
        "/v1/safe-apps/?chainId={}&clientUrl={}",
        chain_id,
        client_url
    ));
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(safe_apps_request))
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::POLYGON_SAFE_APPS_WITH_TAGS),
            })
        });
    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::get_safe_apps],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");
    let response = {
        let mut response = client.get("/v1/chains/137/safe-apps?client_url=https://gnosis-safe.io");
        response.add_header(Header::new("Host", "test.gnosis.io"));
        response.dispatch().await
    };
    let actual_status = response.status();
    let actual_body = response.into_string().await.unwrap();
    let actual: Vec<SafeApp> = serde_json::from_str(&actual_body).unwrap();
    let expected: Vec<SafeApp> = serde_json::from_str(RESPONSE_SAFE_APPS_WITH_TAGS).unwrap();

    assert_eq!(actual_status, Status::Ok);
    assert_eq!(actual, expected);
}
