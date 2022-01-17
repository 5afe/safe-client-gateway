use crate::{
    common::models::backend::chains::ChainInfo as BackendChainInfo,
    common::models::page::Page,
    config::chain_info_request_timeout,
    routes::chains::models::ChainInfo,
    tests::main::setup_rocket,
    utils::http_client::{MockHttpClient, Request, Response},
};
use mockall::predicate::eq;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;

use std::time::Duration;

#[rocket::async_test]
async fn paginated_chain_infos() {
    let request_uri = config_uri!("/v1/chains/?limit=1&offset=1");

    let mut mock_http_client = MockHttpClient::new();
    let mut chain_request = Request::new(request_uri.clone());
    chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(chain_request))
        .returning(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(super::BACKEND_CHAINS_INFO_PAGE),
            })
        });

    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::get_chains],
    ))
    .await
    .expect("valid rocket instance");
    let expected =
        serde_json::from_str::<Page<ChainInfo>>(super::EXPECTED_CHAINS_INFO_PAGE).unwrap();

    let request = client
        .get("/v1/chains?cursor=limit%3D1%26offset%3D1")
        .header(Header::new("Host", "test.gnosis.io/api"))
        .header(ContentType::JSON);

    let response = request.dispatch().await;

    let actual_status = response.status();
    let actual_json_body = response.into_string().await.unwrap();
    let actual = serde_json::from_str::<Page<ChainInfo>>(&actual_json_body).unwrap();

    assert_eq!(actual_status, Status::Ok);
    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn single_chain_info() {
    let mock_http_client = {
        let mut mock_http_client = MockHttpClient::new();
        mock_http_client
            .expect_get()
            .times(1)
            .return_once(move |_| {
                Ok(Response {
                    status_code: 200,
                    body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
                })
            });
        mock_http_client
    };

    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::get_chain],
    ))
    .await
    .expect("valid rocket instance");
    let response = {
        let mut response = client.get("/v1/chains/4");
        response.add_header(Header::new("Host", "test.gnosis.io"));
        response.dispatch().await
    };
    let expected: ChainInfo =
        serde_json::from_str::<BackendChainInfo>(crate::tests::json::CHAIN_INFO_RINKEBY)
            .unwrap()
            .into();

    let actual_status = response.status();
    let actual_json_body = response.into_string().await.unwrap();
    let actual = serde_json::from_str::<ChainInfo>(&actual_json_body).unwrap();

    assert_eq!(actual_status, Status::Ok);
    assert_eq!(actual, expected);
}
