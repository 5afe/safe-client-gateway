use std::time::Duration;

use mockall::predicate::eq;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;

use crate::common::models::backend::chains::{
    BlockExplorerUriTemplate, ChainInfo, GasPrice, NativeCurrency, RpcAuthentication, RpcUri, Theme,
};
use crate::config::{chain_info_request_timeout, contract_info_request_timeout};
use crate::providers::info::*;
use crate::routes::contracts::models::DataDecoderRequest;
use crate::routes::transactions::handlers::preview::TransactionPreview;
use crate::routes::transactions::tests::{
    CONTRACT_INFO, PREVIEW_DATA_DECODED_ERROR_RESPONSE, PREVIEW_RESPONSE,
};
use crate::tests::main::setup_rocket;
use crate::utils::errors::{ApiError, ApiResult};
use crate::utils::http_client::{MockHttpClient, Request, Response};

#[rocket::async_test]
async fn post_preview_success() {
    std::env::set_var("FEATURE_FLAG_NESTED_DECODING", "false");
    std::env::set_var("CONFIG_SERVICE_URI", "https://config-url-example.com");

    let mock_http_client = {
        let mut mock_http_client = MockHttpClient::new();

        let contract_address = "0x37D94d4E230859f83c0868CebEd8CcB83A765cee";
        // Chain Request
        let mut chain_request =
            Request::new("https://config-url-example.com/api/v1/chains/1/".to_string());
        chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));
        mock_http_client
            .expect_get()
            .with(eq(chain_request))
            .times(1)
            .returning(move |_| {
                Ok(Response {
                    status_code: 200,
                    body: String::from(super::CHAIN_RESPONSE),
                })
            });

        // Known Address Request (to field)
        let mut contract_request = Request::new(format!(
            "https://safe-transaction.example.safe.global/api/v1/contracts/{}/",
            contract_address
        ));
        contract_request.timeout(Duration::from_millis(contract_info_request_timeout()));
        mock_http_client
            .expect_get()
            .with(eq(contract_request))
            .times(1)
            .returning(move |_| {
                Ok(Response {
                    status_code: 200,
                    body: String::from(CONTRACT_INFO),
                })
            });

        // Data Decoder Request
        let data = "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000";
        let mut data_decoder_request = Request::new(
            "https://safe-transaction.example.safe.global/api/v1/data-decoder/".to_string(),
        );
        data_decoder_request.body(Some(
            serde_json::to_string::<DataDecoderRequest>(&DataDecoderRequest {
                to: Some("0x37D94d4E230859f83c0868CebEd8CcB83A765cee".to_string()),
                data: data.to_string(),
            })
            .unwrap(),
        ));
        mock_http_client
            .expect_post()
            .with(eq(data_decoder_request))
            .times(1)
            .return_once(move |_| {
                Ok(Response {
                    status_code: 200,
                    body: String::from(crate::tests::json::DATA_DECODED_APPROVE),
                })
            });
        mock_http_client
    };

    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::post_preview_transaction],
        )
        .await,
    )
    .await
    .expect("Valid rocket instance");

    let request =  client.post("/v1/chains/1/transactions/0x37D94d4E230859f83c0868CebEd8CcB83A765cee/preview")
        .header(Header::new("Host", "test.safe.global"))
        .header(ContentType::JSON)
        .body(&json!({
            "to": "0x37D94d4E230859f83c0868CebEd8CcB83A765cee",
            "data": "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000",
            "value": "0",
            "operation": 0,
        }).to_string());
    let response = request.dispatch().await;

    let expected = serde_json::from_str::<TransactionPreview>(PREVIEW_RESPONSE).unwrap();
    let actual_status = response.status();
    let actual =
        serde_json::from_str::<TransactionPreview>(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(actual_status, Status::Ok);
    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn post_preview_data_decoder_error() {
    std::env::set_var("FEATURE_FLAG_NESTED_DECODING", "false");
    std::env::set_var("CONFIG_SERVICE_URI", "https://config-url-example.com");

    let mock_http_client = {
        let mut mock_http_client = MockHttpClient::new();

        let contract_address = "0x37D94d4E230859f83c0868CebEd8CcB83A765cee";
        // Chain Request
        let mut chain_request =
            Request::new("https://config-url-example.com/api/v1/chains/1/".to_string());
        chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));
        mock_http_client
            .expect_get()
            .with(eq(chain_request))
            .times(1)
            .returning(move |_| {
                Ok(Response {
                    status_code: 200,
                    body: String::from(super::CHAIN_RESPONSE),
                })
            });

        // Known Address Request (to field)
        let mut contract_request = Request::new(format!(
            "https://safe-transaction.example.safe.global/api/v1/contracts/{}/",
            contract_address
        ));
        contract_request.timeout(Duration::from_millis(contract_info_request_timeout()));
        mock_http_client
            .expect_get()
            .with(eq(contract_request))
            .times(1)
            .returning(move |_| {
                Err(ApiError::from_http_response(&Response {
                    body: "".to_string(),
                    status_code: 422,
                }))
            });

        // Data Decoder Request
        let data = "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000";
        let mut data_decoder_request = Request::new(
            "https://safe-transaction.example.safe.global/api/v1/data-decoder/".to_string(),
        );
        data_decoder_request.body(Some(
            serde_json::to_string::<DataDecoderRequest>(&DataDecoderRequest {
                to: Some("0x37D94d4E230859f83c0868CebEd8CcB83A765cee".to_string()),
                data: data.to_string(),
            })
            .unwrap(),
        ));
        mock_http_client
            .expect_post()
            .with(eq(data_decoder_request))
            .times(1)
            .return_once(move |_| {
                Err(ApiError::from_http_response(&Response {
                    body: "".to_string(),
                    status_code: 422,
                }))
            });
        mock_http_client
    };

    let client = Client::tracked(
        setup_rocket(
            mock_http_client,
            routes![super::super::routes::post_preview_transaction],
        )
        .await,
    )
    .await
    .expect("Valid rocket instance");

    let request =  client.post("/v1/chains/1/transactions/0x37D94d4E230859f83c0868CebEd8CcB83A765cee/preview")
        .header(Header::new("Host", "test.safe.global"))
        .header(ContentType::JSON)
        .body(&json!({
            "to": "0x37D94d4E230859f83c0868CebEd8CcB83A765cee",
            "data": "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000",
            "value": "0",
            "operation": 0,
        }).to_string());
    let response = request.dispatch().await;

    let expected =
        serde_json::from_str::<TransactionPreview>(PREVIEW_DATA_DECODED_ERROR_RESPONSE).unwrap();
    let actual_status = response.status();
    let actual =
        serde_json::from_str::<TransactionPreview>(&response.into_string().await.unwrap()).unwrap();

    assert_eq!(actual_status, Status::Ok);
    assert_eq!(actual, expected);
}
