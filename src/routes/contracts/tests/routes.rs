use crate::config::chain_info_request_timeout;
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

    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::post_data_decoder],
    ))
    .await
    .expect("valid rocket instance");
    let request  = client.post("/v1/chains/4/data-decoder")
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON)
        .body(&json!({"data": "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000"}).to_string());

    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().await.unwrap(),
        crate::tests::json::DATA_DECODED_APPROVE
    );
}

#[rocket::async_test]
async fn data_decoded_error() {}
