// use rocket::http::{ContentType, Header};
// use rocket::local::asynchronous::Client;

// use crate::config::chain_info_request_timeout;
// use crate::routes::transactions::models::summary::TransactionListItem;
// use crate::tests::main::setup_rocket;
// use crate::utils::http_client::Response;

// #[rocket::async_test]
// pub async fn get_incoming_transfers_no_filters() {
//     let expected = vec![];

//     let mut chain_request = Request::new(config_uri!("/v1/chains/{}/", 4));
//     chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));
//     let mut mock_http_client = MockHttpClient::new();
//     mock_http_client
//         .expect_get()
//         .times(1)
//         .with(eq(chain_request))
//         .return_once(move |_| {
//             Ok(Response {
//                 status_code: 200,
//                 body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
//             })
//         });

//     let mut transfer_request = Request::new(format!(
//         "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/safes/{}/incoming-transfers/",
//         &safe_address
//     ));
//     mock_http_client
//         .expect_get()
//         .times(1)
//         .with(eq(transfer_request))
//         .return_once(move |_| {
//             Ok(Response {
//                 status_code: 200,
//                 body: String::from("[]"),
//             })
//         });

//     let client = Client::tracked(
//         setup_rocket(
//             mock_http_client,
//             routes![crate::routes::transactions::routes::get_incoming_transfers],
//         )
//         .await,
//     )
//     .await
//     .expect("valid rocket instance");

//     let request = client
//         .get("/v1/chains/4/safe/{safe-address}/incoming-transfers")
//         .header(Header::new("Host", "test.gnosis.io"))
//         .header(ContentType::JSON);

//     let response = request.dispatch().await;

//     let actual_status = response.status();
//     let actual =
//         serde_json::from_str::<Vec<TransactionListItem>>(&response.into_string().await.unwrap())
//             .unwrap();

//     assert_eq!(actual_status, Status::Ok);
//     assert_eq!(actual, expected);
// }
