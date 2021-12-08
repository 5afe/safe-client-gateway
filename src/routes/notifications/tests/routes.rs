use crate::config::chain_info_request_timeout;
use crate::routes::notifications::handlers::build_backend_request;
use crate::routes::notifications::models::{
    DeviceData, DeviceType, NotificationRegistrationRequest, SafeRegistration,
};
use crate::tests::main::setup_rocket;
use crate::utils::errors::{ApiError, ErrorDetails};
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
async fn post_notification_success() {
    let safe_address = "0x4cb09344de5bCCD45F045c5Defa0E0452869FF0f";

    let request = NotificationRegistrationRequest {
        device_data: DeviceData {
            uuid: None,
            cloud_messaging_token: "cloud_messaging_token".to_string(),
            build_number: "build_number".to_string(),
            bundle: "bundle".to_string(),
            device_type: DeviceType::Android,
            version: "version".to_string(),
            timestamp: None,
        },
        safe_registrations: vec![
            SafeRegistration {
                chain_id: "4".to_string(),
                safes: vec![safe_address.to_string()],
                signatures: vec!["signature".to_string()],
            },
            SafeRegistration {
                chain_id: "137".to_string(),
                safes: vec![safe_address.to_string()],
                signatures: vec!["signature".to_string()],
            },
        ],
    };

    let backend_request =
        build_backend_request(&request.device_data, &request.safe_registrations[0]); // chain_id is ignored by this method

    let mut mock_http_client = MockHttpClient::new();

    let mut rinkeby_chain_request = Request::new(config_uri!("/v1/chains/{}/", 4));
    rinkeby_chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));

    mock_http_client
        .expect_get()
        .with(eq(rinkeby_chain_request))
        .times(1)
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
            })
        });

    let mut polygon_chain_request = Request::new(config_uri!("/v1/chains/{}/", 137));
    polygon_chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));

    mock_http_client
        .expect_get()
        .with(eq(polygon_chain_request))
        .times(1)
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::CHAIN_INFO_POLYGON),
            })
        });

    let mut post_request_rinkeby = Request::new(String::from(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/notifications/devices/",
    ));
    post_request_rinkeby.body(Some(serde_json::to_string(&backend_request).unwrap()));
    mock_http_client
        .expect_post()
        .times(1)
        .with(eq(post_request_rinkeby))
        .return_once(move |_| {
            Ok(Response {
                status_code: 204,
                body: String::new(),
            })
        });

    let mut post_request_polygon = Request::new(String::from(
        "https://safe-transaction-polygon.staging.gnosisdev.com/api/v1/notifications/devices/",
    ));
    post_request_polygon.body(Some(serde_json::to_string(&backend_request).unwrap()));
    mock_http_client
        .expect_post()
        .times(1)
        .with(eq(post_request_polygon))
        .return_once(move |_| {
            Ok(Response {
                status_code: 204,
                body: String::new(),
            })
        });

    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::post_notification_registration],
    ))
    .await
    .expect("valid rocket instance");

    let request = client
        .post("/v1/register/notifications")
        .body(&serde_json::to_string(&request).unwrap())
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON);

    let response = request.dispatch().await;
    let actual_status = response.status();

    assert_eq!(Status::Ok, actual_status);
}

#[rocket::async_test]
async fn post_notification_error() {
    let safe_address = "0x4cb09344de5bCCD45F045c5Defa0E0452869FF0f";

    let request = NotificationRegistrationRequest {
        device_data: DeviceData {
            uuid: None,
            cloud_messaging_token: "cloud_messaging_token".to_string(),
            build_number: "build_number".to_string(),
            bundle: "bundle".to_string(),
            device_type: DeviceType::Android,
            version: "version".to_string(),
            timestamp: None,
        },
        safe_registrations: vec![
            SafeRegistration {
                chain_id: "4".to_string(),
                safes: vec![safe_address.to_string()],
                signatures: vec!["signature".to_string()],
            },
            SafeRegistration {
                chain_id: "137".to_string(),
                safes: vec![safe_address.to_string()],
                signatures: vec!["signature".to_string()],
            },
        ],
    };

    let backend_request =
        build_backend_request(&request.device_data, &request.safe_registrations[0]); // chain_id is ignored by this method

    let mut mock_http_client = MockHttpClient::new();

    let mut rinkeby_chain_request = Request::new(config_uri!("/v1/chains/{}/", 4));
    rinkeby_chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));

    mock_http_client
        .expect_get()
        .with(eq(rinkeby_chain_request))
        .times(1)
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
            })
        });

    let mut polygon_chain_request = Request::new(config_uri!("/v1/chains/{}/", 137));
    polygon_chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));

    mock_http_client
        .expect_get()
        .with(eq(polygon_chain_request))
        .times(1)
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::CHAIN_INFO_POLYGON),
            })
        });

    let mut post_request_rinkeby = Request::new(String::from(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/notifications/devices/",
    ));
    post_request_rinkeby.body(Some(serde_json::to_string(&backend_request).unwrap()));
    mock_http_client
        .expect_post()
        .times(1)
        .with(eq(post_request_rinkeby))
        .return_once(move |_| {
            Err(ApiError::from_http_response(&Response {
                status_code: 422,
                body: serde_json::to_string(&ErrorDetails {
                    code: 500,
                    message: None,
                    arguments: None,
                    debug: None,
                })
                .unwrap(),
            }))
        });

    let mut post_request_polygon = Request::new(String::from(
        "https://safe-transaction-polygon.staging.gnosisdev.com/api/v1/notifications/devices/",
    ));
    post_request_polygon.body(Some(serde_json::to_string(&backend_request).unwrap()));
    mock_http_client
        .expect_post()
        .times(1)
        .with(eq(post_request_polygon))
        .return_once(move |_| {
            Err(ApiError::from_http_response(&Response {
                status_code: 404,
                body: String::from("Not found"),
            }))
        });

    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::post_notification_registration],
    ))
    .await
    .expect("valid rocket instance");

    let request = client
        .post("/v1/register/notifications")
        .body(&serde_json::to_string(&request).unwrap())
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON);

    let response = request.dispatch().await;
    let actual_status = response.status();
    let error_body = response.into_string().await.unwrap();

    assert_eq!(Status::InternalServerError, actual_status);
}
