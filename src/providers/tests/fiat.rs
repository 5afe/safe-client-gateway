use crate::{
    cache::redis::create_service_cache,
    cache::Cache,
    common::models::{backend::chains::ChainInfo, page::Page},
    config::{
        chain_info_request_timeout, safe_app_info_request_timeout, safe_info_request_timeout,
        token_info_request_timeout,
    },
    providers::info::{DefaultInfoProvider, InfoProvider, SafeAppInfo, SafeInfo, TokenInfo},
    utils::{
        context::RequestContext,
        errors::{ApiError, ErrorDetails},
        http_client::{HttpClient, MockHttpClient, Request, Response},
    },
};
use mockall::predicate::eq;
use std::{sync::Arc, time::Duration};

#[rocket::async_test]
async fn available_currency_codes() {
    let cache = Arc::new(create_service_cache()) as Arc<dyn Cache>;
    cache.invalidate_pattern("*");
    let mut mock_http_client = MockHttpClient::new();

    let context = RequestContext::setup_for_test(
        String::from("request_id"),
        String::from("host"),
        &(Arc::new(mock_http_client) as Arc<dyn HttpClient>),
        &cache,
    );
}

#[rocket::async_test]
async fn available_currency_codes_not_found() {}
