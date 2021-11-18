use crate::{
    cache::redis::create_service_cache,
    cache::Cache,
    common::models::backend::chains::ChainInfo,
    config::{chain_info_request_timeout, safe_info_request_timeout},
    providers::info::{DefaultInfoProvider, InfoProvider, SafeInfo},
    utils::{
        context::RequestContext,
        errors::{ApiError, ErrorDetails},
        http_client::{HttpClient, MockHttpClient, Request, Response},
    },
};
use mockall::predicate::eq;
use std::{sync::Arc, time::Duration};

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
    let expected = ApiError {
        status: 404,
        details: ErrorDetails {
            code: 1337,
            message: Some(String::from("Not found")),
            arguments: None,
            debug: None,
        },
    };
    let info_provider = DefaultInfoProvider::new("4", &context);

    let actual = info_provider.chain_info().await;

    assert_eq!(actual, Err(expected));
}

#[rocket::async_test]
async fn default_info_provider_safe_info() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
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

    let mut safe_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/safes/{}/",
        safe_address
    ));
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

    let context = RequestContext::new(
        String::from(&request_uri),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
    );
    let expected = serde_json::from_str::<SafeInfo>(crate::tests::json::SAFE_WITH_MODULES)
        .expect("SafeInfo deserialization issue");
    let info_provider = DefaultInfoProvider::new("4", &context);

    let actual = info_provider.safe_info(safe_address).await.unwrap();

    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn default_info_provider_safe_info_not_found() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
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

    let mut safe_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/safes/{}/",
        safe_address
    ));
    safe_request.timeout(Duration::from_millis(safe_info_request_timeout()));
    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(safe_request))
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
    let expected = ApiError {
        status: 404,
        details: ErrorDetails {
            code: 1337,
            message: Some(String::from("Not found")),
            arguments: None,
            debug: None,
        },
    };
    let info_provider = DefaultInfoProvider::new("4", &context);

    let actual = info_provider.safe_info(safe_address).await;

    assert_eq!(actual, Err(expected));
}
