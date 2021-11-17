use std::{sync::Arc, time::Duration};

use mockall::predicate::eq;

use crate::{
    cache::redis::create_service_cache,
    cache::Cache,
    common::models::backend::chains::ChainInfo,
    config::chain_info_request_timeout,
    providers::info::{DefaultInfoProvider, InfoProvider},
    utils::{
        context::RequestContext,
        http_client::{HttpClient, MockHttpClient, Request, Response},
    },
};

#[rocket::async_test]
async fn default_info_provider_chain_info() {
    let expected =
        serde_json::from_str::<ChainInfo>(crate::tests::json::CHAIN_INFO_RINKEBY).unwrap();
    let request_uri = config_uri!("/v1/chains/{}/", 4);
    let mut mock_http_client = MockHttpClient::new();
    let cache = Arc::new(create_service_cache()) as Arc<dyn Cache>;

    let mut chain_request = Request::new(request_uri.clone());
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
    let context = RequestContext::new(
        String::from(&request_uri),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
    );

    let info_provider = DefaultInfoProvider::new("4", &context);

    let actual = info_provider.chain_info().await.unwrap();

    assert_eq!(actual, expected)
}

#[rocket::async_test]
async fn default_info_provider_chain_info_in_mem_cache() {}

#[rocket::async_test]
async fn default_info_provider_chain_info_hits_redis_cache() {}

#[rocket::async_test]
async fn default_info_provider_chain_info_not_found() {}
