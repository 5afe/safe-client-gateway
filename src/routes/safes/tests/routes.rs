use crate::config::{
    chain_info_request_timeout, safe_info_request_timeout, transaction_request_timeout,
};
use crate::routes::safes::models::SafeState;
use crate::tests::main::setup_rocket;
use crate::utils::errors::ApiError;
// use crate::utils::errors::{ApiError, ErrorDetails};
use crate::utils::http_client::{MockHttpClient, Request, Response};
use core::time::Duration;
use mockall::predicate::eq;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
// use serde_json::json;

#[rocket::async_test]
async fn get_safe_info() {
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

    let mut safe_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/safes/{}/",
        &safe_address
    ));
    safe_request.timeout(Duration::from_millis(safe_info_request_timeout()));
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(safe_request))
        .returning(move |_| {
            Ok(Response {
                body: String::from(crate::tests::json::SAFE_WITH_GUARD_SAFE_V130_L2),
                status_code: 200,
            })
        });

    let master_copies_request = Request::new(String::from(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/about/master-copies/",
    ));
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(master_copies_request))
        .returning(move |_| {
            Ok(Response {
                body: String::from(crate::tests::json::POLYGON_MASTER_COPIES),
                status_code: 200,
            })
        });

    let mut request_last_collectible = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/safes/{}\
        /transfers/\
        ?&erc721=true\
        &limit=1",
        safe_address
    ));

    request_last_collectible.timeout(Duration::from_millis(transaction_request_timeout()));
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(request_last_collectible))
        .returning(move |_| {
            Ok(Response {
                body: String::from(super::LAST_COLLECTIBLE_TRANSFER),
                status_code: 200,
            })
        });

    let mut request_last_queued_tx = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/safes/{}/\
        multisig-transactions/?\
        &ordering=-modified\
        &executed=false\
        &trusted=true\
        &limit=1",
        safe_address,
    ));
    request_last_queued_tx.timeout(Duration::from_millis(transaction_request_timeout()));
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(request_last_queued_tx))
        .returning(move |_| {
            Ok(Response {
                body: String::from(super::LAST_QUEUED_TX),
                status_code: 200,
            })
        });

    let mut request_last_history_tx = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/safes/{}/\
        all-transactions/?\
        &ordering=executionDate
        &queued=false\
        &executed=true",
        safe_address
    ));
    request_last_history_tx.timeout(Duration::from_millis(transaction_request_timeout()));
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(request_last_history_tx))
        .returning(move |_| {
            Ok(Response {
                body: String::from(super::LAST_HISTORY_TX),
                status_code: 200,
            })
        });

    mock_http_client.expect_get().returning(move |_| {
        Err(ApiError::from_http_response(&Response {
            body: String::new(),
            status_code: 404,
        }))
    });

    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::get_safe_info],
    ))
    .await
    .expect("valid rocket instance");
    let expected = serde_json::from_str::<SafeState>(super::SAFE_STATE).unwrap();

    let request = client
        .get("/v1/chains/4/safes/0x4cb09344de5bCCD45F045c5Defa0E0452869FF0f")
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON);

    let response = request.dispatch().await;

    let actual_status = response.status();
    let actual_json_body = response.into_string().await.unwrap();
    let actual = serde_json::from_str::<SafeState>(&actual_json_body).unwrap();

    assert_eq!(actual_status, Status::Ok);
    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn get_safe_info_not_found() {}
