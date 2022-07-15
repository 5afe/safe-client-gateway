use std::time::Duration;

use mockall::predicate::eq;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;
use serde_json::json;

use crate::common::models::backend::chains::{
    BlockExplorerUriTemplate, ChainInfo, GasPrice, NativeCurrency, RpcAuthentication, RpcUri, Theme,
};
use crate::config::contract_info_request_timeout;
use crate::providers::info::*;
use crate::routes::contracts::models::DataDecoderRequest;
use crate::routes::transactions::handlers::preview::TransactionPreview;
use crate::routes::transactions::tests::PREVIEW_RESPONSE;
use crate::tests::main::setup_rocket;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::{MockHttpClient, Request, Response};

fn get_chain_info() -> ChainInfo {
    ChainInfo {
        recommended_master_copy_version: "1.1.1".to_string(),
        transaction_service: "https://safe-transaction.mainnet.gnosis.io".to_string(),
        vpc_transaction_service: "http://mainnet-safe-transaction-web.safe.svc.cluster.local"
            .to_string(),
        chain_id: "1".to_string(),
        chain_name: "".to_string(),
        short_name: "eth".to_string(),
        l2: false,
        description: "Random description".to_string(),
        rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "".to_string(),
        },
        safe_apps_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "".to_string(),
        },
        public_rpc_uri: RpcUri {
            authentication: RpcAuthentication::ApiKeyPath,
            value: "".to_string(),
        },
        block_explorer_uri_template: BlockExplorerUriTemplate {
            address: "".to_string(),
            tx_hash: "".to_string(),
            api: "".to_string(),
        },
        native_currency: NativeCurrency {
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0,
            logo_uri: "https://test.token.image.url".to_string(),
        },
        theme: Theme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
        ens_registry_address: None,
        gas_price: vec![GasPrice::Fixed {
            wei_value: "1000000".to_string(),
        }],
        disabled_wallets: vec![],
        features: vec![],
    }
}

#[rocket::async_test]
async fn post_preview_success() {
    let mut mock_info_provider = MockInfoProvider::new();
    let data = "0x095ea7b3000000000000000000000000ae9844f89d98c150f5e61bfc676d68b4921559900000000000000000000000000000000000000000000000000001c6bf52634000";
    mock_info_provider
        .expect_chain_info()
        .returning_st(move || Ok(get_chain_info()));

    let mock_http_client = {
        let mut mock_http_client = MockHttpClient::new();

        let contract_address = "0x6810e776880C02933D47DB1b9fc05908e5386b96";
        // Known Address Request (to field)
        let mut contract_request = Request::new(
            core_uri!(mock_info_provider, "/v1/contracts/{}/", contract_address).unwrap(),
        );
        contract_request.timeout(Duration::from_millis(contract_info_request_timeout()));
        mock_http_client.expect_get().times(1).returning(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(super::CONTRACTS_RESPONSE),
            })
        });

        // Data Decoder Request
        let mut data_decoder_request =
            Request::new(core_uri!(mock_info_provider, "/v1/data-decoder/").unwrap());
        data_decoder_request.body(Some(
            serde_json::to_string::<DataDecoderRequest>(&DataDecoderRequest {
                data: data.to_string(),
            })
            .unwrap(),
        ));
        mock_http_client
            .expect_post()
            .with(eq(data_decoder_request))
            .times(0) // TODO core_uri!(info_provider, "/v1/data-decoder/") throws Cached value not available (?)
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
        .header(Header::new("Host", "test.gnosis.io"))
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
