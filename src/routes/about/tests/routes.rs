extern crate dotenv;

use crate::cache::redis::create_service_cache;
use crate::cache::{Cache, MockCache};
use crate::config::{build_number, version};
use crate::routes::about::models::{About, ChainAbout};
use crate::utils::context::RequestContext;
use crate::utils::http_client::{HttpClient, MockHttpClient, Request, Response};
use dotenv::dotenv;
use rocket::http::{Header, Status};
use rocket::local::asynchronous::Client;
use rocket::{Build, Rocket};
use serde_json::json;
use std::sync::Arc;

fn setup_rocket(mock_http_client: MockHttpClient) -> Rocket<Build> {
    dotenv().ok();

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
        .manage(Arc::new(create_service_cache()) as Arc<dyn Cache>)
        .manage(Arc::new(mock_http_client) as Arc<dyn HttpClient>)
}

#[rocket::async_test]
async fn get_chains_about() {
    let mock_http_client = {
        let mut mock_http_client = MockHttpClient::new();
        mock_http_client
            .expect_get()
            .times(1)
            .return_once(move |_| {
                Ok(Response {
                    status_code: 200,
                    body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
                })
            });
        mock_http_client
    };
    let expected = ChainAbout {
        transaction_service_base_uri: "https://safe-transaction.rinkeby.staging.gnosisdev.com"
            .to_string(),
        about: About {
            name: env!("CARGO_PKG_NAME").to_string(),
            version: version(),
            build_number: build_number(),
        },
    };

    let client = Client::tracked(setup_rocket(mock_http_client))
        .await
        .expect("valid rocket instance");
    let response = {
        let mut response = client.get("/v1/chains/4/about");
        response.add_header(Header::new("Host", "test.gnosis.io"));
        response.dispatch().await
    };

    assert_eq!(response.status(), Status::Ok);
    assert_eq!(
        response.into_string().await.unwrap(),
        serde_json::to_string(&expected).unwrap()
    );
}

#[rocket::async_test]
async fn get_about() {}

#[rocket::async_test]
async fn get_master_copies() {}

#[rocket::async_test]
async fn get_backbone() {}

#[rocket::async_test]
async fn get_redis() {}
