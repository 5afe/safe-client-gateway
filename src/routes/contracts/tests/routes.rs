use crate::common::models::data_decoded::DataDecoded;
use crate::config::{chain_info_request_timeout, contract_info_request_timeout};
use crate::providers::address_info::ContractInfo;
use crate::tests::main::setup_rocket;
use crate::utils::errors::{ApiError, ErrorDetails};
use crate::utils::http_client::{MockHttpClient, Request, Response};
use core::time::Duration;
use mockall::predicate::eq;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;

#[rocket::async_test]
async fn data_decoded() {
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

    let mut collectibles_request = Request::new(String::from(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/data-decoder/",
    ));
    collectibles_request.body(Some(
        json!({
            "data": "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000"
        }).to_string()
    ));
    mock_http_client
        .expect_post()
        .times(1)
        .with(eq(collectibles_request))
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::DATA_DECODED_APPROVE),
            })
        });

    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::post_data_decoder],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");
    let request  = client.post("/v1/chains/4/data-decoder")
        .header(Header::new("Host", "test.safe.global"))
        .header(ContentType::JSON)
        .body(&json!({"data": "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000"}).to_string());

    let response = request.dispatch().await;

    let actual_status = response.status();
    let actual =
        serde_json::from_str::<DataDecoded>(&response.into_string().await.unwrap()).unwrap();
    let expected =
        serde_json::from_str::<DataDecoded>(crate::tests::json::DATA_DECODED_APPROVE).unwrap();
    assert_eq!(actual_status, Status::Ok);
    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn data_decoded_error() {
    let backend_error_json = json!({"details": "Not found"}).to_string();
    let error = ErrorDetails {
        code: 1337,
        message: Some(backend_error_json.clone()),
        arguments: None,
        debug: None,
    };

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

    let mut collectibles_request = Request::new(String::from(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/data-decoder/",
    ));
    collectibles_request.body(Some(
        json!({
            "data": "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000"
        }).to_string()
    ));
    mock_http_client
        .expect_post()
        .times(1)
        .with(eq(collectibles_request))
        .return_once(move |_| {
            Err(ApiError::from_http_response(&Response {
                status_code: 404,
                body: backend_error_json.clone(),
            }))
        });

    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::post_data_decoder],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");
    let request  = client.post("/v1/chains/4/data-decoder")
        .header(Header::new("Host", "test.safe.global"))
        .header(ContentType::JSON)
        .body(&json!({"data": "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000"}).to_string());

    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::NotFound);
    assert_eq!(
        response.into_string().await.unwrap(),
        serde_json::to_string(&error).unwrap()
    );
}

#[rocket::async_test]
async fn get_contract() {
    let bip_contract_address = "0x00000000000045166C45aF0FC6E4Cf31D9E14B9A";
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

    let mut contract_info_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/contracts/{}/",
        &bip_contract_address
    ));
    contract_info_request.timeout(Duration::from_millis(contract_info_request_timeout()));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(contract_info_request))
        .returning(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::CONTRACT_INFO_BID),
            })
        });

    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::get_contract],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");

    let request = client
        .get(format!("/v1/chains/4/contracts/{}", &bip_contract_address))
        .header(Header::new("Host", "test.safe.global"))
        .header(ContentType::JSON);

    let response = request.dispatch().await;
    let actual_status = response.status();
    let actual =
        serde_json::from_str::<ContractInfo>(&response.into_string().await.unwrap()).unwrap();
    let expected =
        serde_json::from_str::<ContractInfo>(crate::tests::json::CONTRACT_INFO_BID).unwrap();

    assert_eq!(Status::Ok, actual_status);
    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn get_contract_not_found() {
    let backend_error_json = json!({"details": "Not found"}).to_string();
    let error = ErrorDetails {
        code: 1337,
        message: Some(backend_error_json.clone()),
        arguments: None,
        debug: None,
    };
    let bip_contract_address = "0x00000000000045166C45aF0FC6E4Cf31D9E14B9A";
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

    let mut contract_info_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/contracts/{}/",
        &bip_contract_address
    ));
    contract_info_request.timeout(Duration::from_millis(contract_info_request_timeout()));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(contract_info_request))
        .returning(move |_| {
            Err(ApiError::from_http_response(&Response {
                status_code: 404,
                body: backend_error_json.clone(),
            }))
        });

    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::get_contract],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");

    let request = client
        .get(format!("/v1/chains/4/contracts/{}", &bip_contract_address))
        .header(Header::new("Host", "test.safe.global"))
        .header(ContentType::JSON);

    let response = request.dispatch().await;
    let actual_status = response.status();
    let actual = response.into_string().await.unwrap();
    let expected = serde_json::to_string(&error).unwrap();

    assert_eq!(Status::NotFound, actual_status);
    assert_eq!(expected, actual);
}
