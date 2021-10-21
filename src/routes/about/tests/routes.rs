use crate::cache::{Cache, MockCache};
use crate::utils::context::RequestContext;
use crate::utils::http_client::{HttpClient, MockHttpClient, Request, Response};
use rocket::http::Status;
use rocket::local::asynchronous::Client;
use rocket::{Build, Rocket};
use std::sync::Arc;

#[rocket::async_test]
async fn testing_mocked_http_client() {
    let response_json = "{\"valid\":\"json\"}";

    let mut mock_http_client = MockHttpClient::new();
    mock_http_client
        .expect_get()
        .times(1)
        .return_once(move |_| {
            Ok(Response {
                status_code: 200,
                body: String::from(response_json),
            })
        });

    let mock_cache = MockCache::new();

    let request_context = RequestContext::mock(
        "request_id".to_string(),
        "host".to_string(),
        mock_http_client,
        mock_cache,
    );
    let request = Request::new("https://example.com".to_string());

    let actual = request_context
        .http_client()
        .get(request)
        .await
        .expect("response error");
    assert_eq!(response_json, actual.body);
}

fn setup_rocket(
    mock_cache: Arc<dyn Cache>,
    mock_http_client: Arc<dyn HttpClient>,
) -> Rocket<Build> {
    rocket::build()
        .mount(
            "/",
            routes![
                super::super::routes::backbone,
                super::super::routes::get_about,
                super::super::routes::get_chains_about,
                super::super::routes::redis,
                super::super::routes::get_master_copies,
            ],
        )
        .manage(mock_cache.clone())
        .manage(mock_http_client.clone())
}

#[rocket::async_test]
async fn get_chains_about() {
    // let client = Client::tracked(rocket()).expect("valid rocket instance");
    // let mut response = client.get("/v1/chains/4/about").dispatch();

    // assert_eq!(response.status(), Status::Ok);
    // assert_eq!(response.into_string().unwrap(), "Hello, world!");
}

#[rocket::async_test]
async fn get_about() {}

#[rocket::async_test]
async fn get_master_copies() {}

#[rocket::async_test]
async fn get_backbone() {}

#[rocket::async_test]
async fn get_redis() {}
