use crate::{
    cache::redis::create_service_cache,
    cache::Cache,
    providers::fiat::{Exchange, FiatInfoProvider},
    utils::{
        context::RequestContext,
        errors::{ApiError, ErrorDetails},
        http_client::{HttpClient, MockHttpClient, Request, Response},
    },
};
use mockall::predicate::eq;
use std::{sync::Arc, time::Duration};

const EXCHANGE_API_BASE_URI: &'static str = "https://test.exchange-rate.api";
const EXCHANGE_API_KEY: &'static str = "some_random_key";

fn setup_exchange_env() {
    std::env::set_var("EXCHANGE_API_BASE_URI", EXCHANGE_API_BASE_URI);
    std::env::set_var("EXCHANGE_API_KEY", EXCHANGE_API_KEY);
}

#[rocket::async_test]
async fn available_currency_codes() {
    setup_exchange_env();
    let cache = Arc::new(create_service_cache()) as Arc<dyn Cache>;
    cache.invalidate_pattern("*");

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
        &cache,
    );
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
}
