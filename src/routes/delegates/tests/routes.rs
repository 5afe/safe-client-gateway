use crate::common::models::page::Page;
use crate::routes::delegates::models::{
    Delegate, DelegateCreate, DelegateDelete, SafeDelegateDelete,
};
use crate::tests::main::setup_rocket;
use crate::utils::http_client::{MockHttpClient, Request, Response};
use core::time::Duration;
use mockall::predicate::eq;
use rocket::http::hyper::request;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;

use crate::config::{chain_info_request_timeout, default_request_timeout};

/// Test get delegate from safe
/// Check the CGW requests to the config-service and transaction-service
/// Check the response of get /v1/chains/{}/delegates?safe={}
#[rocket::async_test]
async fn get_delegates_from_safe() {
    let safe_address = "0xaE3c91c89153DEaC332Ab7BBd167164978638c30";
    // Mock config service
    let mut chain_request = Request::new(config_uri!("/v1/chains/{}/", 4));
    chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));

    let mut mock_http_client = MockHttpClient::new();
    let mut mock_http_client = MockHttpClient::new();
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(chain_request))
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
            })
        });
    // Mock delegates transaction service
    let mut delegates_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/delegates/?safe={}&delegate=&delegator=&label=",
        &safe_address));
    delegates_request.timeout(Duration::from_millis(default_request_timeout()));
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(delegates_request))
        .returning(move |_| {
            Ok(Response {
                body: String::from(super::BACKEND_LIST_DELEGATES_OF_SAFE),
                status_code: 200,
            })
        });
    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::get_delegates],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");
    let expected =
        serde_json::from_str::<Page<Delegate>>(super::EXPECTED_LIST_DELEGATES_OF_SAFE).unwrap();
    // Create request get delegates
    let request = client
        .get(format!("/v1/chains/{}/delegates?safe={}", 4, &safe_address))
        .header(Header::new("Host", "test.gnosis.io/api"))
        .header(ContentType::JSON);
    let response = request.dispatch().await;
    let actual_status = response.status();
    let actual_json_body = response.into_string().await.unwrap();
    let actual = serde_json::from_str::<Page<Delegate>>(&actual_json_body).unwrap();
    assert_eq!(actual_status, Status::Ok);
    assert_eq!(actual, expected);
}

/// Test add delegate to safe
/// Check the CGW requests to the config-service and transaction-service
/// Check the status response
#[rocket::async_test]
async fn add_delegate() {
    // Mock config service
    let mut chain_request = Request::new(config_uri!("/v1/chains/{}/", 4));
    chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));

    let mut mock_http_client = MockHttpClient::new();
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(chain_request))
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
            })
        });
    // Mock transaction service delegates
    let mut delegates_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/delegates/"
    ));
    let mut body_string = String::from(super::BACKEND_CREATE_DELEGATE);
    body_string = body_string.replace('\n', "");
    body_string = body_string.replace(' ', "");
    delegates_request.body(Some(body_string));
    delegates_request.timeout(Duration::from_millis(default_request_timeout()));
    mock_http_client
        .expect_post()
        .times(1)
        .with(eq(delegates_request))
        .returning(move |_| {
            Ok(Response {
                body: String::from(super::BACKEND_CREATE_DELEGATE),
                status_code: 201,
            })
        });

    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::post_delegate],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");
    // Create post request
    let request = client
        .post(format!("/v1/chains/4/delegates"))
        .body(String::from(super::BACKEND_CREATE_DELEGATE))
        .header(Header::new("Host", "test.gnosis.io/api"))
        .header(ContentType::JSON);
    println!("{:?}", request);
    let response = request.dispatch().await;
    let actual_status = response.status();

    assert_eq!(actual_status, Status::Ok);
}

/// Test delete delegate
/// Check the CGW requests to the config-service and transaction-service
/// Check the status response
#[rocket::async_test]
async fn delete_delegate() {
    // Mock the response of config service
    let mut chain_request = Request::new(config_uri!("/v1/chains/{}/", 4));
    chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));
    let mut mock_http_client = MockHttpClient::new();
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(chain_request))
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
            })
        });
    let delegate_address = "0x4CA998cE947Aed03c340141a5491Df539ff1Fd05";
    // Mock the response of transaction service delegates
    let mut delegates_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/delegates/{}/",
        delegate_address
    ));
    let mut body_string = String::from(super::BACKEND_DELETE_DELEGATE);
    body_string = body_string.replace('\n', "");
    body_string = body_string.replace(' ', "");
    delegates_request.body(Some(body_string));
    delegates_request.timeout(Duration::from_millis(default_request_timeout()));
    mock_http_client
        .expect_delete()
        .times(1)
        .with(eq(delegates_request))
        .returning(move |_| {
            Ok(Response {
                body: String::from(""),
                status_code: 204,
            })
        });
    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::delete_delegate],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");
    // Create request to delete a delegate
    let request = client
        .delete(format!("/v1/chains/4/delegates/{}", delegate_address))
        .body(String::from(super::BACKEND_DELETE_DELEGATE))
        .header(Header::new("Host", "test.gnosis.io/api"))
        .header(ContentType::JSON);
    let response = request.dispatch().await;
    let actual_status = response.status();
    // Should return OK
    assert_eq!(actual_status, Status::Ok);
}

/// Test delete delegate of a safe
/// Check the CGW requests to the config-service and transaction-service
/// Check the status response
#[rocket::async_test]
async fn delete_delegate_safe() {
    // Mock the response of config service
    let mut chain_request = Request::new(config_uri!("/v1/chains/{}/", 4));
    chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));
    let mut mock_http_client = MockHttpClient::new();
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(chain_request))
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
            })
        });
    let delegate_address = "0x4CA998cE947Aed03c340141a5491Df539ff1Fd05";
    let safe_address = "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02";
    // Mock the response of transaction service delegates
    let mut delegates_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/safes/{}/delegates/{}/",
        safe_address, delegate_address
    ));
    let mut body_string = String::from(super::BACKEND_DELETE_DELEGATE_SAFE);
    body_string = body_string.replace('\n', "");
    body_string = body_string.replace(' ', "");
    delegates_request.body(Some(body_string));
    delegates_request.timeout(Duration::from_millis(default_request_timeout()));
    mock_http_client
        .expect_delete()
        .times(1)
        .with(eq(delegates_request))
        .returning(move |_| {
            Ok(Response {
                body: String::from(""),
                status_code: 204,
            })
        });
    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::delete_safe_delegate],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");
    // Create request to delete a delegate
    let request = client
        .delete(format!(
            "/v1/chains/4/safes/{}/delegates/{}/",
            safe_address, delegate_address
        ))
        .body(String::from(super::BACKEND_DELETE_DELEGATE_SAFE))
        .header(Header::new("Host", "test.gnosis.io/api"))
        .header(ContentType::JSON);
    let response = request.dispatch().await;
    let actual_status = response.status();
    // Should return OK
    assert_eq!(actual_status, Status::Ok);
}
