extern crate dotenv;

use crate::cache::redis::create_service_cache;
use crate::cache::Cache;
use crate::config::{
    chain_info_request_timeout, safe_info_request_timeout, transaction_request_timeout,
};
use crate::utils::http_client::{HttpClient, MockHttpClient, Request, Response};
use core::time::Duration;
use dotenv::dotenv;
use mockall::predicate::eq;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use rocket::{Build, Rocket};
use serde_json::json;
use std::sync::Arc;

fn setup_rocket(mock_http_client: MockHttpClient) -> Rocket<Build> {
    dotenv().ok();

    rocket::build()
        .mount("/", routes![super::super::routes::post_confirmation])
        .manage(Arc::new(create_service_cache()) as Arc<dyn Cache>)
        .manage(Arc::new(mock_http_client) as Arc<dyn HttpClient>)
}

#[rocket::async_test]
async fn post_confirmation_success() {
    let mock_http_client = {
        let mut mock_http_client = MockHttpClient::new();

        let mut chain_request = Request::new(config_uri!("/v1/chains/{}/", 4));
        chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));

        //TODO Check why this is called twice on it's not hitting the in-memory cache
        mock_http_client
            .expect_get()
            .times(2)
            .with(eq(chain_request))
            .returning(move |_| {
                Ok(Response {
                    status_code: 200,
                    body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
                })
            });

        // CONFIRMATION REQUEST
        let mut backend_request = Request::new(
                "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/multisig-transactions/0x728e6dec56dc61523b56dc440e34c1c4c39c66895df8e5d3499ed1f7d4fcfe80/confirmations/"
                .to_string(),
        );
        backend_request.body(Some(json!({"signature": "bd42f5c205b544cc6397c8c2e592ca4ade02b8681673cc8c555ff1777b002ee959c3cca243a77a2de1bbe1b61413342ac7d6416a31ec0ff31bb1029e921202ee1c"}).to_string()));

        mock_http_client
            .expect_post()
            .times(1)
            .with(eq(backend_request))
            .return_once(move |_| {
                Ok(Response {
                    status_code: 201,
                    body: String::new(),
                })
            });

        // SAFE REQUEST
        let mut safe_request = Request::new(String::from("https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/safes/0xBc79855178842FDBA0c353494895DEEf509E26bB/"));
        safe_request.timeout(Duration::from_millis(safe_info_request_timeout()));

        mock_http_client
            .expect_get()
            .times(2)
            .with(eq(safe_request))
            .returning(move |_| {
                Ok(Response {
                    body: String::from(crate::tests::json::SAFE_TX_DETAILS_TESTS),
                    status_code: 200,
                })
            });

        // GAS TOKEN INFO REQUEST
        let mut gas_token_request = Request::new(String::fr)

        // TX DETAILS
        let mut details_request =
            Request::new(String::from("https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/multisig-transactions/0x728e6dec56dc61523b56dc440e34c1c4c39c66895df8e5d3499ed1f7d4fcfe80/"));
        details_request.timeout(Duration::from_millis(transaction_request_timeout()));

        mock_http_client
            .expect_get()
            .times(1)
            .with(eq(details_request))
            .return_once(move |_| {
                Ok(Response {
                    status_code: 200,
                    body: String::from(crate::tests::json::BACKEND_TX_DETAILS_WITH_ORIGIN),
                })
            });
        mock_http_client
    };

    let client = Client::tracked(setup_rocket(mock_http_client))
        .await
        .expect("Valid rocket instance");

    let request =  client.post("/v1/chains/4/transactions/0x728e6dec56dc61523b56dc440e34c1c4c39c66895df8e5d3499ed1f7d4fcfe80/confirmations")
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON)
        .body(&json!({"signedSafeTxHash":"bd42f5c205b544cc6397c8c2e592ca4ade02b8681673cc8c555ff1777b002ee959c3cca243a77a2de1bbe1b61413342ac7d6416a31ec0ff31bb1029e921202ee1c"}).to_string());
    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::Created);
    // assert_eq!(response.into_string().await.unwrap(), "");
}

#[rocket::async_test]
async fn post_confirmation_error_already_executed_tx() {}
