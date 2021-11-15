extern crate dotenv;

use crate::common::models::page::Page;
use crate::config::{
    chain_info_request_timeout, contract_info_request_timeout, safe_info_request_timeout,
    token_info_request_timeout, transaction_request_timeout,
};
use crate::providers::info::TokenInfo;
use crate::routes::transactions::models::details::TransactionDetails;
use crate::routes::transactions::tests::{MULTISIG_TX_DETAILS, POST_CONFIRMATION_RESULT};
use crate::tests::main::setup_rocket;
use crate::utils::errors::{ApiError, ErrorDetails};
use crate::utils::http_client::{MockHttpClient, Request, Response};
use core::time::Duration;
use mockall::predicate::eq;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;

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

        // TX DETAILS
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
            .times(1)
            .with(eq(safe_request))
            .returning(move |_| {
                Ok(Response {
                    body: String::from(crate::tests::json::SAFE_WITH_MODULES),
                    status_code: 200,
                })
            });

        // Transfer TokenInfo
        let mut token_request = Request::new(String::from(
            "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/tokens/?limit=10000",
        ));
        token_request.timeout(Duration::from_millis(token_info_request_timeout()));
        let page_tokens: Page<TokenInfo> = Page {
            next: None,
            previous: None,
            results: vec![
                serde_json::from_str(crate::tests::json::TOKEN_BAT).expect("BAT token failure")
            ],
        };

        mock_http_client
            .expect_get()
            .times(1)
            .with(eq(token_request))
            .returning(move |_| {
                Ok(Response {
                    body: serde_json::to_string(&page_tokens).expect("Token page failure"),
                    status_code: 200,
                })
            });

        // Catch all calls not relevant to the test
        mock_http_client.expect_get().returning(move |_| {
            Ok(Response {
                status_code: 404,
                body: String::new(),
            })
        });

        mock_http_client
    };

    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::post_confirmation],
    ))
    .await
    .expect("Valid rocket instance");

    let request =  client.post("/v1/chains/4/transactions/0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa/confirmations")
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON)
        .body(&json!({"signedSafeTxHash":"bd42f5c205b544cc6397c8c2e592ca4ade02b8681673cc8c555ff1777b002ee959c3cca243a77a2de1bbe1b61413342ac7d6416a31ec0ff31bb1029e921202ee1c"}).to_string());
    let response = request.dispatch().await;

    let expected = serde_json::from_str::<TransactionDetails>(POST_CONFIRMATION_RESULT).unwrap();
    let actual_status = response.status();
    let actual =
        serde_json::from_str::<TransactionDetails>(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(actual_status, Status::Ok);
    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn post_confirmation_confirmation_error() {
    let backend_error_json = json!({"signature": ["Transaction with safe-tx-hash=0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa was already executed"]}).to_string();
    let error = ErrorDetails {
        code: 1337,
        message: Some(backend_error_json.clone()),
        arguments: None,
        debug: None,
    };

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
                Err(ApiError::from_http_response(&Response {
                    status_code: 400,
                    body: backend_error_json.clone(),
                }))
            });
        mock_http_client
    };
    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::post_confirmation],
    ))
    .await
    .expect("Valid rocket instance");

    let request =  client.post("/v1/chains/4/transactions/0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa/confirmations")
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON)
        .body(&json!({"signedSafeTxHash":"bd42f5c205b544cc6397c8c2e592ca4ade02b8681673cc8c555ff1777b002ee959c3cca243a77a2de1bbe1b61413342ac7d6416a31ec0ff31bb1029e921202ee1c"}).to_string());
    let response = request.dispatch().await;
    let status = response.status();
    let body = response.into_string().await.unwrap();

    assert_eq!(status, Status::BadRequest);
    assert_eq!(body, serde_json::to_string(&error).unwrap());
}

#[rocket::async_test]
async fn post_confirmation_confirmation_success_tx_details_error() {
    let backend_error_json = json!({"details": "Not found"}).to_string();
    let error = ErrorDetails {
        code: 1337,
        message: Some(backend_error_json.clone()),
        arguments: None,
        debug: None,
    };

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

        // TX DETAILS
        let mut details_request =
            Request::new(String::from("https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/multisig-transactions/0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa/"));
        details_request.timeout(Duration::from_millis(transaction_request_timeout()));

        mock_http_client
            .expect_get()
            .times(1)
            .with(eq(details_request))
            .return_once(move |_| {
                Err(ApiError::from_http_response(&Response {
                    status_code: 404,
                    body: backend_error_json.clone(),
                }))
            });

        mock_http_client
    };
    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::post_confirmation],
    ))
    .await
    .expect("Valid rocket instance");

    let request =  client.post("/v1/chains/4/transactions/0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa/confirmations")
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON)
        .body(&json!({"signedSafeTxHash":"bd42f5c205b544cc6397c8c2e592ca4ade02b8681673cc8c555ff1777b002ee959c3cca243a77a2de1bbe1b61413342ac7d6416a31ec0ff31bb1029e921202ee1c"}).to_string());
    let response = request.dispatch().await;
    let status = response.status();
    let body = response.into_string().await.unwrap();

    assert_eq!(status, Status::NotFound);
    assert_eq!(body, serde_json::to_string(&error).unwrap());
}

#[rocket::async_test]
async fn tx_details_multisig_tx_success() {
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

        // TransactionDetails
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

        // SafeInfo fetch for cancellations
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

        // Gas TokenInfo and Transfer token
        let mut token_request = Request::new(String::from(
            "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/tokens/?limit=10000",
        ));
        token_request.timeout(Duration::from_millis(token_info_request_timeout()));
        let page_tokens: Page<TokenInfo> = Page {
            next: None,
            previous: None,
            results: vec![
                serde_json::from_str(crate::tests::json::TOKEN_BAT).expect("BAT token failure")
            ],
        };

        mock_http_client
            .expect_get()
            .times(1)
            .with(eq(token_request))
            .returning(move |_| {
                Ok(Response {
                    body: serde_json::to_string(&page_tokens).expect("Token page failure"),
                    status_code: 200,
                })
            });

        // Known Addresses
        // the current safe does not get requested as a knownAddress by design
        // The Transfer target gets requested multiple times, but caching reduces it to once
        let mut known_address_request = Request::new(String::from("https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/contracts/0xF353eBBa77e5E71c210599236686D51cA1F88b84/"));

        known_address_request.timeout(Duration::from_millis(contract_info_request_timeout()));
        mock_http_client
            .expect_get()
            .times(1)
            .with(eq(known_address_request))
            .return_once(move |_| {
                Ok(Response {
                    status_code: 200,
                    body: String::from(
                        json! ({
                            "address": "0xF353eBBa77e5E71c210599236686D51cA1F88b84",
                            "name": "Transfer target",
                            "displayName": "Transfer target",
                        })
                        .to_string(),
                    ),
                })
            });

        // Catch all calls not relevant to the test
        mock_http_client.expect_get().returning(move |_| {
            Ok(Response {
                status_code: 404,
                body: String::new(),
            })
        });

        mock_http_client
    };

    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::get_transactions],
    ))
    .await
    .expect("Valid rocket instance");

    let request =  client.get("/v1/chains/4/transactions/0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa/")
        .header(Header::new("Host", "test.gnosis.io"));
    let response = request.dispatch().await;

    let expected = serde_json::from_str::<TransactionDetails>(MULTISIG_TX_DETAILS).unwrap();
    let actual_status = response.status();
    let actual =
        serde_json::from_str::<TransactionDetails>(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(actual_status, Status::Ok);
    assert_eq!(actual, expected);
}
