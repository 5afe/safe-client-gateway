extern crate dotenv;

use crate::cache::redis::create_service_cache;
use crate::cache::Cache;
use crate::config::{
    chain_info_request_timeout, contract_info_request_timeout, safe_info_request_timeout,
    transaction_request_timeout,
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

    let cache = create_service_cache();
    cache.invalidate("*");
    rocket::build()
        .mount("/", routes![super::super::routes::post_confirmation])
        .manage(Arc::new(cache) as Arc<dyn Cache>)
        .manage(Arc::new(mock_http_client) as Arc<dyn HttpClient>)
}

#[rocket::async_test]
async fn post_confirmation_success() {
    let mock_http_client = {
        let mut mock_http_client = MockHttpClient::new();

        let mut chain_request = Request::new(config_uri!("/v1/chains/{}/", 4));
        chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));
        mock_http_client
            .expect_get()
            .times(1)
            .with(eq(chain_request))
            .returning(move |_| {
                Ok(Response {
                    status_code: 200,
                    body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
                })
            });

        // CONFIRMATION REQUEST
        let mut backend_request = Request::new(
                "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/multisig-transactions/0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa/confirmations/"
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

        // // TX DETAILS
        let mut details_request =
            Request::new(String::from("https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/multisig-transactions/0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa/"));
        details_request.timeout(Duration::from_millis(transaction_request_timeout()));

        mock_http_client
            .expect_get()
            .times(1)
            .with(eq(details_request))
            .return_once(move |_| {
                Ok(Response {
                    status_code: 200,
                    body: String::from(crate::tests::json::BACKEND_MULTISIG_TRANSFER_TX),
                })
            });

        // safe info fetch for cancellations
        let mut safe_request = Request::new(String::from("https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/safes/0x1230B3d59858296A31053C1b8562Ecf89A2f888b/"));
        safe_request.timeout(Duration::from_millis(safe_info_request_timeout()));

        mock_http_client
            .expect_get()
            .times(1) // From FETCHING CANCELLATION AND FROM ENRICHING TX DETAILS
            .with(eq(safe_request))
            .returning(move |_| {
                Ok(Response {
                    body: String::from(crate::tests::json::SAFE_WITH_MODULES),
                    status_code: 200,
                })
            });

        // Cancellation tx
        let mut cancellation_tx_request = Request::new(String::from("https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/multisig-transactions/0x43e0a39de2a62b8a79ac429cce6e0e9316907beef2e390fb2bebcbcf6412f4cf/"));
        cancellation_tx_request.timeout(Duration::from_millis(transaction_request_timeout()));
        mock_http_client
            .expect_get()
            .times(1)
            .with(eq(cancellation_tx_request))
            .return_once(move |_| {
                Ok(Response {
                    status_code: 404,
                    body: String::new(),
                })
            });

        // KNOWN ADDRESSES
        let mut known_address_request = Request::new(String::from("https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/contracts/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46/"));
        known_address_request.timeout(Duration::from_millis(contract_info_request_timeout()));
        mock_http_client
            .expect_get()
            .times(1)
            .with(eq(known_address_request))
            .return_once(move |_| {
                Ok(Response {
                    status_code: 404,
                    body: String::new(),
                })
            });

        let mut known_address_request = Request::new(String::from("https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/contracts/0xF353eBBa77e5E71c210599236686D51cA1F88b84/"));

        known_address_request.timeout(Duration::from_millis(contract_info_request_timeout()));
        mock_http_client
            .expect_get()
            .times(1)
            .with(eq(known_address_request))
            .return_once(move |_| {
                Ok(Response {
                    status_code: 404,
                    body: String::new(),
                })
            });

        mock_http_client
    };

    let client = Client::tracked(setup_rocket(mock_http_client))
        .await
        .expect("Valid rocket instance");

    let request =  client.post("/v1/chains/4/transactions/0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa/confirmations")
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON)
        .body(&json!({"signedSafeTxHash":"bd42f5c205b544cc6397c8c2e592ca4ade02b8681673cc8c555ff1777b002ee959c3cca243a77a2de1bbe1b61413342ac7d6416a31ec0ff31bb1029e921202ee1c"}).to_string());
    let response = request.dispatch().await;

    assert_eq!(response.status(), Status::Ok);
    // assert_eq!(response.into_string().await.unwrap(), "");
}

#[rocket::async_test]
async fn post_confirmation_error_already_executed_tx() {}
