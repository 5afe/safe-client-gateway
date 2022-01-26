use crate::tests::main::setup_rocket;
use crate::utils::http_client::MockHttpClient;
use rocket::http::{ContentType, Header, Status};
use rocket::local::asynchronous::Client;

#[rocket::async_test]
async fn health() {
    let mock_http_client = MockHttpClient::new();

    let client = Client::tracked(setup_rocket(
        mock_http_client,
        routes![super::super::routes::health],
    ))
    .await
    .expect("valid rocket instance");

    let request = client
        .get("/health")
        .header(Header::new("Host", "test.gnosis.io"))
        .header(ContentType::JSON);

    let response = request.dispatch().await;
    let actual_status = response.status();
    let actual = response.into_string().await.unwrap();

    assert_eq!(Status::Ok, actual_status);
    assert_eq!("\"\"", actual);
}
