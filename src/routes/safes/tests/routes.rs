use crate::tests::main::setup_rocket;
use crate::utils::errors::{ApiError, ErrorDetails};
use crate::utils::http_client::{MockHttpClient, Request, Response};
use core::time::Duration;
use mockall::predicate::eq;
use rocket::http::{ContentType, Header, Status};

#[rocket::async_test]
async fn get_safe_info() {
    // let mut chain_request = Request::new(config_uri!("/v1/chains/{}/", 4));
    // chain_request.timeout(Duration::from_millis(chain_info_request_timeout()));

    // let mut mock_http_client = MockHttpClient::new();
    // mock_http_client
    //     .expect_get()
    //     .times(1)
    //     .with(eq(chain_request))
    //     .return_once(move |_| {
    //         Ok(Response {
    //             status_code: 200,
    //             body: String::from(crate::tests::json::CHAIN_INFO_RINKEBY),
    //         })
    //     });
}

#[rocket::async_test]
async fn get_safe_info_not_found() {}
