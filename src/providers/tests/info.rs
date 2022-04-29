use crate::cache::redis::create_service_cache;
use crate::cache::Cache;
use crate::common::models::backend::chains::ChainInfo;
use crate::common::models::page::Page;
use crate::config::{
    chain_info_request_timeout, contract_info_request_timeout, safe_app_info_request_timeout,
    safe_info_request_timeout, token_info_request_timeout,
};
use crate::create_service_cache_mainnet;
use crate::providers::address_info::ContractInfo;
use crate::providers::info::{DefaultInfoProvider, InfoProvider, SafeAppInfo, SafeInfo, TokenInfo};
use crate::utils::context::RequestContext;
use crate::utils::errors::{ApiError, ErrorDetails};
use crate::utils::http_client::{HttpClient, MockHttpClient, Request, Response};
use mockall::predicate::eq;
use std::sync::Arc;
use std::time::Duration;

#[rocket::async_test]
async fn default_info_provider_chain_info() {
    let expected =
        serde_json::from_str::<ChainInfo>(crate::tests::json::CHAIN_INFO_RINKEBY).unwrap();
    let request_uri = config_uri!("/v1/chains/{}/", 4);
    let cache = Arc::new(create_service_cache().await) as Arc<dyn Cache>;
    let mainnet_cache = Arc::new(create_service_cache_mainnet().await) as Arc<dyn Cache>;

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
    let context = RequestContext::setup_for_test(
        String::from(&request_uri),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
        &mainnet_cache,
    )
    .await;

    let info_provider = DefaultInfoProvider::new("4", &context);

    let actual = info_provider.chain_info().await.unwrap();

    assert_eq!(actual, expected)
}

#[rocket::async_test]
async fn default_info_provider_chain_info_not_found() {
    let request_uri = config_uri!("/v1/chains/{}/", 4);
    let cache = Arc::new(create_service_cache().await) as Arc<dyn Cache>;
    let mainnet_cache = Arc::new(create_service_cache_mainnet().await) as Arc<dyn Cache>;
    cache.invalidate_pattern("*").await;

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
    let context = RequestContext::setup_for_test(
        String::from(&request_uri),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
        &mainnet_cache,
    )
    .await;
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
    let cache = Arc::new(create_service_cache().await) as Arc<dyn Cache>;
    let mainnet_cache = Arc::new(create_service_cache_mainnet().await) as Arc<dyn Cache>;

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

    let context = RequestContext::setup_for_test(
        String::from(&request_uri),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
        &mainnet_cache,
    )
    .await;
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
    let cache = Arc::new(create_service_cache().await) as Arc<dyn Cache>;
    let mainnet_cache = Arc::new(create_service_cache_mainnet().await) as Arc<dyn Cache>;

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

    let context = RequestContext::setup_for_test(
        String::from(&request_uri),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
        &mainnet_cache,
    )
    .await;
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

#[rocket::async_test]
async fn default_info_provider_token_info() {
    let token_address = "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46";
    let request_uri = config_uri!("/v1/chains/{}/", 4);
    let cache = Arc::new(create_service_cache().await) as Arc<dyn Cache>;
    let mainnet_cache = Arc::new(create_service_cache_mainnet().await) as Arc<dyn Cache>;

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

    let mut token_request = Request::new(String::from(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/tokens/?limit=20000",
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
    let context = RequestContext::setup_for_test(
        String::from(&request_uri),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
        &mainnet_cache,
    )
    .await;
    let expected = Ok(serde_json::from_str::<TokenInfo>(crate::tests::json::TOKEN_BAT).unwrap());

    let info_provider = DefaultInfoProvider::new("4", &context);
    let actual = info_provider.token_info(token_address).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn default_info_provider_token_info_request_failure() {
    let token_address = "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46";
    let request_uri = config_uri!("/v1/chains/{}/", 4);
    let cache = Arc::new(create_service_cache().await) as Arc<dyn Cache>;
    let mainnet_cache = Arc::new(create_service_cache_mainnet().await) as Arc<dyn Cache>;

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

    let mut token_request = Request::new(String::from(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/tokens/?limit=20000",
    ));
    token_request.timeout(Duration::from_millis(token_info_request_timeout()));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(token_request))
        .returning(move |_| {
            Err(ApiError::from_http_response(&Response {
                status_code: 404,
                body: String::from("Not found"),
            }))
        });
    let context = RequestContext::setup_for_test(
        String::from(&request_uri),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
        &mainnet_cache,
    )
    .await;
    let expected = Err(ApiError::new_from_message_with_code(
        404,
        String::from("Not found"),
    ));

    let info_provider = DefaultInfoProvider::new("4", &context);
    let actual = info_provider.token_info(token_address).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn default_info_provider_token_info_not_found_in_cache() {
    let token_address = "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De41";
    let request_uri = config_uri!("/v1/chains/{}/", 4);
    let cache = Arc::new(create_service_cache().await) as Arc<dyn Cache>;
    let mainnet_cache = Arc::new(create_service_cache_mainnet().await) as Arc<dyn Cache>;

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

    let mut token_request = Request::new(String::from(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/tokens/?limit=20000",
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
    let context = RequestContext::setup_for_test(
        String::from(&request_uri),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
        &mainnet_cache,
    )
    .await;
    let expected = Err(ApiError::new_from_message("Could not generate value"));

    let info_provider = DefaultInfoProvider::new("4", &context);
    let actual = info_provider.token_info(token_address).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn default_info_provider_token_info_address_0x0() {
    let token_address = "0x0000000000000000000000000000000000000000";
    let cache = Arc::new(create_service_cache().await) as Arc<dyn Cache>;
    let mainnet_cache = Arc::new(create_service_cache_mainnet().await) as Arc<dyn Cache>;

    let mock_http_client = MockHttpClient::new();

    let context = RequestContext::setup_for_test(
        String::from(""),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
        &mainnet_cache,
    )
    .await;
    let expected = Err(ApiError::new_from_message_with_code(
        500,
        String::from("Token Address is 0x0"),
    ));

    let info_provider = DefaultInfoProvider::new("4", &context);
    let actual = info_provider.token_info(token_address).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn default_info_provider_safe_app_info() {
    let origin_url = "https://app.uniswap.org";
    let cache = Arc::new(create_service_cache().await) as Arc<dyn Cache>;
    let mainnet_cache = Arc::new(create_service_cache_mainnet().await) as Arc<dyn Cache>;

    let mut mock_http_client = MockHttpClient::new();
    let mut safe_app_request = Request::new(format!("{}/manifest.json", &origin_url));
    safe_app_request.timeout(Duration::from_millis(safe_app_info_request_timeout()));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(safe_app_request))
        .returning(move |_| {
            Ok(Response {
                body: String::from(crate::tests::json::UNISWAP_SAFE_APP_MANIFEST),
                status_code: 200,
            })
        });

    let context = RequestContext::setup_for_test(
        String::from(""),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
        &mainnet_cache,
    )
    .await;
    let expected = Ok(SafeAppInfo {
        name: String::from("Uniswap"),
        url: String::from(origin_url),
        logo_uri: format!("{}/{}", &origin_url, "./images/256x256_App_Icon_Pink.svg"),
    });

    let info_provider = DefaultInfoProvider::new("4", &context);
    let actual = info_provider.safe_app_info(origin_url).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn default_info_provider_safe_app_info_not_found() {
    let origin_url = "https://app.uniswap.org";
    let cache = Arc::new(create_service_cache().await) as Arc<dyn Cache>;
    let mainnet_cache = Arc::new(create_service_cache_mainnet().await) as Arc<dyn Cache>;

    let mut mock_http_client = MockHttpClient::new();
    let mut safe_app_request = Request::new(format!("{}/manifest.json", &origin_url));
    safe_app_request.timeout(Duration::from_millis(safe_app_info_request_timeout()));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(safe_app_request))
        .returning(move |_| {
            Err(ApiError::from_http_response(&Response {
                status_code: 404,
                body: String::from("Not found"),
            }))
        });

    let context = RequestContext::setup_for_test(
        String::from(""),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
        &mainnet_cache,
    )
    .await;
    let expected = Err(ApiError::new_from_message_with_code(
        404,
        String::from("Not found"),
    ));

    let info_provider = DefaultInfoProvider::new("4", &context);
    let actual = info_provider.safe_app_info(origin_url).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn contract_info() {
    let bip_contract_address = "0x00000000000045166C45aF0FC6E4Cf31D9E14B9A";
    let cache = Arc::new(create_service_cache().await) as Arc<dyn Cache>;
    let mainnet_cache = Arc::new(create_service_cache_mainnet().await) as Arc<dyn Cache>;

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

    let mut contract_info_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/contracts/{}/",
        &bip_contract_address
    ));
    contract_info_request.timeout(Duration::from_millis(contract_info_request_timeout()));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(contract_info_request))
        .returning(move |_| {
            Ok(Response {
                status_code: 202,
                body: String::from(crate::tests::json::CONTRACT_INFO_BID),
            })
        });

    let context = RequestContext::setup_for_test(
        String::from(""),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
        &mainnet_cache,
    )
    .await;
    let expected =
        serde_json::from_str::<ContractInfo>(crate::tests::json::CONTRACT_INFO_BID).unwrap();

    let info_provider = DefaultInfoProvider::new("4", &context);
    let actual = info_provider
        .contract_info(bip_contract_address)
        .await
        .unwrap();

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn contract_info_not_found() {
    let bip_contract_address = "0x00000000000045166C45aF0FC6E4Cf31D9E14B9A";
    let cache = Arc::new(create_service_cache().await) as Arc<dyn Cache>;
    let mainnet_cache = Arc::new(create_service_cache_mainnet().await) as Arc<dyn Cache>;

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

    let mut contract_info_request = Request::new(format!(
        "https://safe-transaction.rinkeby.staging.gnosisdev.com/api/v1/contracts/{}/",
        &bip_contract_address
    ));
    contract_info_request.timeout(Duration::from_millis(contract_info_request_timeout()));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(contract_info_request))
        .returning(move |_| {
            Err(ApiError::from_http_response(&Response {
                status_code: 404,
                body: String::from("Not found"),
            }))
        });

    let context = RequestContext::setup_for_test(
        String::from(""),
        config_uri!(""),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
        &mainnet_cache,
    )
    .await;
    let expected = Err(ApiError::new_from_message_with_code(
        404,
        String::from("Not found"),
    ));

    let info_provider = DefaultInfoProvider::new("4", &context);
    let actual = info_provider.contract_info(bip_contract_address).await;

    assert_eq!(expected, actual);
}
