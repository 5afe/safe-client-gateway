use std::{sync::Arc, time::Duration};

use mockall::predicate::eq;

use crate::{
    cache::redis::create_service_cache,
    cache::{Cache, MockCache},
    common::models::backend::chains::ChainInfo,
    config::chain_info_request_timeout,
    providers::info::{DefaultInfoProvider, InfoProvider},
    utils::{
        context::RequestContext,
        errors::{ApiError, ErrorDetails},
        http_client::{HttpClient, MockHttpClient, Request, Response},
    },
};

#[rocket::async_test]
async fn default_info_provider_chain_info() {
    let expected =
        serde_json::from_str::<ChainInfo>(crate::tests::json::CHAIN_INFO_RINKEBY).unwrap();
    let request_uri = config_uri!("/v1/chains/{}/", 4);
    let cache = Arc::new(create_service_cache()) as Arc<dyn Cache>;
    cache.invalidate_pattern("*");

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
async fn default_info_provider_chain_info_not_found() {
    let expected = ApiError {
        status: 404,
        details: ErrorDetails {
            code: 1337,
            message: Some(String::from("Not found")),
            arguments: None,
            debug: None,
        },
    };
    let request_uri = config_uri!("/v1/chains/{}/", 4);
    let cache = Arc::new(create_service_cache()) as Arc<dyn Cache>;
    cache.invalidate_pattern("*");

    let mut mock_http_client = MockHttpClient::new();
    let mut chain_request = Request::new(request_uri.clone());
    chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(chain_request))
        .returning(move |_| {
            Err(ApiError::from_http_response(&Response {
                status_code: 404,
                body: String::from("Not found"),
            }))
        });
    let context = RequestContext::new(
        String::from(&request_uri),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
    );
    let info_provider = DefaultInfoProvider::new("4", &context);

    let actual = info_provider.chain_info().await;

    assert_eq!(actual, Err(expected));
}
