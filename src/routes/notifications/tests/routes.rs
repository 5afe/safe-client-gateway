use crate::config::chain_info_request_timeout;
use crate::tests::main::setup_rocket;
use crate::utils::errors::ApiError;
use crate::utils::http_client::{MockHttpClient, Request, Response};
use mockall::predicate::eq;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use std::time::Duration;

#[rocket::async_test]
async fn delete_notification_success() {
    let uuid = "some_uuid";
    let safe_address = "0x4cb09344de5bCCD45F045c5Defa0E0452869FF0f";

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

    let delete_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com\
    /api/v1/notifications/devices/{}/safes/{}/",
        uuid, safe_address
    ));
    mock_http_client
        .expect_delete()
        .times(1)
        .with(eq(delete_request))
        .return_once(move |_| {
            Ok(Response {
                status_code: 204,
                body: String::new(),
            })
        });

    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::delete_notification_registration],
    ))
    .await
    .expect("valid rocket instance");

    let request = client
        .delete(format!(
            "/v1/chains/{}/notifications/devices/{}/safes/{}",
            4, uuid, safe_address
        ))
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON);

    let response = request.dispatch().await;
    let actual_status = response.status();

    assert_eq!(Status::Ok, actual_status);
}

#[rocket::async_test]
async fn delete_notification_error() {
    let uuid = "some_uuid";
    let safe_address = "0x4cb09344de5bCCD45F045c5Defa0E0452869FF0f";

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

    let delete_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com\
    /api/v1/notifications/devices/{}/safes/{}/",
        uuid, safe_address
    ));
    mock_http_client
        .expect_delete()
        .times(1)
        .with(eq(delete_request))
        .return_once(move |_| {
            Err(ApiError::from_http_response(&Response {
                status_code: 422,
                body: String::new(),
            }))
        });

    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::delete_notification_registration],
    ))
    .await
    .expect("valid rocket instance");

    let request = client
        .delete(format!(
            "/v1/chains/{}/notifications/devices/{}/safes/{}",
            4, uuid, safe_address
        ))
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON);

    let response = request.dispatch().await;
    let actual_status = response.status();

    assert_eq!(Status::UnprocessableEntity, actual_status);
}

#[rocket::async_test]
async fn post_notification_success() {}

#[rocket::async_test]
async fn post_notification_error() {}
