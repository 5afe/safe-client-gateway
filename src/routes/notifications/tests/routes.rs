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
async fn post_notification_client_error() {
    let safe_address = "0x4cb09344de5bCCD45F045c5Defa0E0452869FF0f";
    let expected_error = ErrorDetails {
        code: 1337,
        message: Some("Push notification registration failed for chain IDs: 4, 137".to_string()),
        arguments: None,
        debug: serde_json::from_str(
            "[{\"4\":{\"code\":500,\"message\":null}}\
        ,{\"137\":{\"safes\":{\"0\":[\"Address 0x0 is not valid\"]},\
        \"timestamp\":[\"Provided timestamp is not in a range within 5 minutes\"]}}]",
        )
        .ok(),
    };

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
                safes: vec!["0x0".to_string()],
                signatures: vec!["signature".to_string()],
            },
            SafeRegistration {
                chain_id: "137".to_string(),
                safes: vec![safe_address.to_string()],
                signatures: vec!["signature".to_string()],
            },
        ],
    };

    let rinkeby_backend_request =
        build_backend_request(&request.device_data, &request.safe_registrations[0]);
    let polygon_backend_request =
        build_backend_request(&request.device_data, &request.safe_registrations[1]);

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
    post_request_rinkeby.body(Some(
        serde_json::to_string(&rinkeby_backend_request).unwrap(),
    ));
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
    post_request_polygon.body(Some(
        serde_json::to_string(&polygon_backend_request).unwrap(),
    ));
    mock_http_client
        .expect_post()
        .times(1)
        .with(eq(post_request_polygon))
        .return_once(move |_| {
            Err(ApiError::new_from_message_with_code(
                400,
                "{\"safes\": {\
                            \"0\": [\
                                \"Address 0x0 is not valid\"\
                            ]\
                        },\
                        \"timestamp\": [\
                            \"Provided timestamp is not in a range within 5 minutes\"\
                        ]}"
                .to_string(),
            ))
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
    let actual = serde_json::from_str::<ErrorDetails>(&error_body).unwrap();

    assert_eq!(Status::BadRequest, actual_status);
    assert_eq!(expected_error, actual);
}

#[rocket::async_test]
async fn post_notification_server_and_client_errors() {
    let mut mock_http_client = MockHttpClient::new();
    let safe_address = "0x4cb09344de5bCCD45F045c5Defa0E0452869FF0f";
    // Mock /v1/chains/
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
    // Request Payload
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
                chain_id: "137".to_string(),
                safes: vec!["0x0".to_string()],
                signatures: vec!["signature".to_string()],
            },
            SafeRegistration {
                chain_id: "137".to_string(),
                safes: vec![safe_address.to_string()],
                signatures: vec!["signature".to_string()],
            },
        ],
    };
    // POST request with first payload – returns a 400
    let polygon_backend_request_0 =
        build_backend_request(&request.device_data, &request.safe_registrations[0]);
    let mut post_request_polygon_0 = Request::new(String::from(
        "https://safe-transaction-polygon.staging.gnosisdev.com/api/v1/notifications/devices/",
    ));
    post_request_polygon_0.body(Some(
        serde_json::to_string(&polygon_backend_request_0).unwrap(),
    ));
    mock_http_client
        .expect_post()
        .times(1)
        .with(eq(post_request_polygon_0))
        .return_once(move |_| {
            Err(ApiError::new_from_message_with_code(
                400,
                "{ \"test\" : \"Some client error\"}".to_string(),
            ))
        });
    // POST request with first payload – returns a 500
    let polygon_backend_request_1 =
        build_backend_request(&request.device_data, &request.safe_registrations[1]);
    let mut post_request_polygon_1 = Request::new(String::from(
        "https://safe-transaction-polygon.staging.gnosisdev.com/api/v1/notifications/devices/",
    ));
    post_request_polygon_1.body(Some(
        serde_json::to_string(&polygon_backend_request_1).unwrap(),
    ));
    mock_http_client
        .expect_post()
        .times(1)
        .with(eq(post_request_polygon_1))
        .return_once(move |_| {
            Err(ApiError::new_from_message_with_code(
                500,
                "{ \"test\" : \"Some server error\"}".to_string(),
            ))
        });

    // Test execution
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
    let actual = serde_json::from_str::<ErrorDetails>(&error_body).unwrap();

    let expected_error = ErrorDetails {
        code: 1337,
        message: Some("Push notification registration failed for chain IDs: 137, 137".to_string()),
        arguments: None,
        debug: serde_json::from_str("[ {\"137\" : { \"test\" : \"Some client error\"}}, {\"137\" : { \"test\" : \"Some server error\"}}]").ok(),
    };
    assert_eq!(Status::InternalServerError, actual_status);
    assert_eq!(expected_error, actual);
}
