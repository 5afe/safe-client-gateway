use crate::common::models::page::Page;
use crate::config::{chain_info_request_timeout, transaction_request_timeout};
use crate::routes::transactions::models::summary::TransactionListItem;
use crate::tests::main::setup_rocket;
use crate::utils::http_client::{MockHttpClient, Request, Response};
use core::time::Duration;
use mockall::predicate::eq;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;

#[rocket::async_test]
pub async fn get_incoming_transfers_no_filters() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let expected = Page {
        next: None,
        previous: None,
        results: vec![],
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

    let mut transfer_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/safes/{}/incoming-transfers/?limit=20&offset=0",
        &safe_address
    ));
    transfer_request.timeout(Duration::from_millis(transaction_request_timeout()));
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(transfer_request))
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: serde_json::to_string(&Page::<TransactionListItem> {
                    next: None,
                    previous: None,
                    results: vec![],
                })
                .unwrap(),
            })
        });

    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![crate::routes::transactions::routes::get_incoming_transfers],
        )
        .await,
    )
    .await
    .expect("valid rocket instance");

    let request = client
        .get(format!(
            "/v1/chains/4/safes/{}/incoming-transfers",
            &safe_address
        ))
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON);

    let response = request.dispatch().await;

    let actual_status = response.status();
    let value = &response.into_string().await.unwrap();
    let actual = serde_json::from_str::<Page<TransactionListItem>>(&value).unwrap();

    assert_eq!(actual_status, Status::Ok);
    assert_eq!(actual, expected);
}
