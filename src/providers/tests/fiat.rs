use std::str::FromStr;
use std::sync::Arc;

use bigdecimal::BigDecimal;
use mockall::predicate::eq;
use serde_json::json;

use crate::cache::manager::ChainCache;
use crate::cache::Cache;
use crate::providers::fiat::{Exchange, FiatInfoProvider};
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiError;
use crate::utils::http_client::{HttpClient, MockHttpClient, Request, Response};
use crate::{create_cache_manager, RedisCacheManager};

const EXCHANGE_API_BASE_URI: &'static str = "https://test.exchange-rate.api";
const EXCHANGE_API_KEY: &'static str = "some_random_key";

fn setup_exchange_env() {
    std::env::set_var("EXCHANGE_API_BASE_URI", EXCHANGE_API_BASE_URI);
    std::env::set_var("EXCHANGE_API_KEY", EXCHANGE_API_KEY);
}

#[rocket::async_test]
async fn available_currency_codes() {
    setup_exchange_env();
    let cache_manager = create_cache_manager().await;
    cache_manager
        .cache_for_chain(ChainCache::Mainnet)
        .invalidate_pattern("*")
        .await;
    cache_manager
        .cache_for_chain(ChainCache::Other)
        .invalidate_pattern("*")
        .await;

    let mut mock_http_client = MockHttpClient::new();
    let request = Request::new(format!(
        "{}?access_key={}",
        EXCHANGE_API_BASE_URI, EXCHANGE_API_KEY
    ));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(request))
        .returning(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::EXCHANGE_CURRENCY_RATES),
            })
        });
    let context = RequestContext::setup_for_test(
        String::from("request_id"),
        String::from("host"),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &(Arc::new(cache_manager) as Arc<dyn RedisCacheManager>),
    )
    .await;
    let fiat_provider = FiatInfoProvider::new(&context);

    let mut expected =
        serde_json::from_str::<Exchange>(crate::tests::json::EXCHANGE_CURRENCY_RATES)
            .unwrap()
            .rates
            .unwrap()
            .into_keys()
            .collect::<Vec<String>>();

    let mut actual = fiat_provider.available_currency_codes().await.unwrap();

    assert_eq!(expected.sort(), actual.sort());
}

#[rocket::async_test]
async fn available_currency_codes_api_error() {
    setup_exchange_env();
    let cache_manager = create_cache_manager().await;
    cache_manager
        .cache_for_chain(ChainCache::Mainnet)
        .invalidate_pattern("*")
        .await;
    cache_manager
        .cache_for_chain(ChainCache::Other)
        .invalidate_pattern("*")
        .await;
    let api_error_json =
        json!({"success":false,"error":{"code":105,"type":"base_currency_access_restricted"}});

    let mut mock_http_client = MockHttpClient::new();
    let request = Request::new(format!(
        "{}?access_key={}",
        EXCHANGE_API_BASE_URI, EXCHANGE_API_KEY
    ));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(request))
        .returning(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(api_error_json.to_string()),
            })
        });
    let context = RequestContext::setup_for_test(
        String::from("request_id"),
        String::from("host"),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &(Arc::new(cache_manager) as Arc<dyn RedisCacheManager>),
    )
    .await;
    let fiat_provider = FiatInfoProvider::new(&context);

    let actual = fiat_provider.available_currency_codes().await;

    let expected = Err(ApiError::new_from_message_with_code(
        500,
        String::from("Unknown 'Exchange' json structure"),
    ));
    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn exchange_usd_to() {
    setup_exchange_env();
    let cache_manager = create_cache_manager().await;
    cache_manager
        .cache_for_chain(ChainCache::Mainnet)
        .invalidate_pattern("*")
        .await;
    cache_manager
        .cache_for_chain(ChainCache::Other)
        .invalidate_pattern("*")
        .await;

    let mut mock_http_client = MockHttpClient::new();
    let request = Request::new(format!(
        "{}?access_key={}",
        EXCHANGE_API_BASE_URI, EXCHANGE_API_KEY
    ));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(request))
        .returning(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::EXCHANGE_CURRENCY_RATES),
            })
        });
    let context = RequestContext::setup_for_test(
        String::from("request_id"),
        String::from("host"),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &(Arc::new(cache_manager) as Arc<dyn RedisCacheManager>),
    )
    .await;
    let fiat_provider = FiatInfoProvider::new(&context);
    let expected = Ok(1 / BigDecimal::from_str("1.125036").unwrap());

    let actual = fiat_provider.exchange_usd_to("EUR").await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn exchange_usd_to_usd() {
    setup_exchange_env();
    let cache_manager = create_cache_manager().await;
    cache_manager
        .cache_for_chain(ChainCache::Mainnet)
        .invalidate_pattern("*")
        .await;
    cache_manager
        .cache_for_chain(ChainCache::Other)
        .invalidate_pattern("*")
        .await;

    let mut mock_http_client = MockHttpClient::new();
    mock_http_client.expect_get().times(0);
    let context = RequestContext::setup_for_test(
        String::from("request_id"),
        String::from("host"),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &(Arc::new(cache_manager) as Arc<dyn RedisCacheManager>),
    )
    .await;
    let fiat_provider = FiatInfoProvider::new(&context);
    let expected = Ok(BigDecimal::from(1));

    let actual = fiat_provider.exchange_usd_to("USD").await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn exchange_usd_to_unknown_code() {
    setup_exchange_env();
    let cache_manager = create_cache_manager().await;
    cache_manager
        .cache_for_chain(ChainCache::Mainnet)
        .invalidate_pattern("*")
        .await;
    cache_manager
        .cache_for_chain(ChainCache::Other)
        .invalidate_pattern("*")
        .await;

    let mut mock_http_client = MockHttpClient::new();
    let request = Request::new(format!(
        "{}?access_key={}",
        EXCHANGE_API_BASE_URI, EXCHANGE_API_KEY
    ));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(request))
        .returning(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(crate::tests::json::EXCHANGE_CURRENCY_RATES),
            })
        });
    let context = RequestContext::setup_for_test(
        String::from("request_id"),
        String::from("host"),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &(Arc::new(cache_manager) as Arc<dyn RedisCacheManager>),
    )
    .await;
    let fiat_provider = FiatInfoProvider::new(&context);
    let expected = Err(ApiError::new_from_message_with_code(
        422,
        String::from("Currency not found"),
    ));

    let actual = fiat_provider.exchange_usd_to("UNKOWN_CURRENCY_CODE").await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn exchange_usd_to_api_failure() {
    setup_exchange_env();
    let cache_manager = create_cache_manager().await;
    cache_manager
        .cache_for_chain(ChainCache::Mainnet)
        .invalidate_pattern("*")
        .await;
    cache_manager
        .cache_for_chain(ChainCache::Other)
        .invalidate_pattern("*")
        .await;
    let api_error_json =
        json!({"success":false,"error":{"code":105,"type":"base_currency_access_restricted"}});

    let mut mock_http_client = MockHttpClient::new();
    let request = Request::new(format!(
        "{}?access_key={}",
        EXCHANGE_API_BASE_URI, EXCHANGE_API_KEY
    ));

    mock_http_client
        .expect_get()
        .times(1)
        .with(eq(request))
        .returning(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(api_error_json.to_string()),
            })
        });
    let context = RequestContext::setup_for_test(
        String::from("request_id"),
        String::from("host"),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &(Arc::new(cache_manager) as Arc<dyn RedisCacheManager>),
    )
    .await;
    let fiat_provider = FiatInfoProvider::new(&context);

    let actual = fiat_provider.exchange_usd_to("EUR").await;

    let expected = Err(ApiError::new_from_message_with_code(
        500,
        String::from("Unknown 'Exchange' json structure"),
    ));
    assert_eq!(expected, actual);
}
