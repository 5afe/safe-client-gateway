use rocket::local::Client;
use crate::utils::errors::ApiError;
use rocket::response::Responder;
use crate::models::backend::transactions::MultisigTransaction;

#[test]
fn api_error_responder_json() {
    let api_error = ApiError { status: 404, message: Some(String::from("Not found")) };
    let rocket = rocket::ignite();
    let client = Client::new(rocket).expect("valid rocket instance");

    let local_request = client.get("/");
    let request = local_request.inner();
    let mut response = api_error.respond_to(&request).unwrap();
    let body_json = response.body().unwrap().into_string().unwrap();

    assert_eq!(response.status().code, 404);
    assert_eq!(body_json, "{\"status\":404,\"message\":\"Not found\"}");
}

#[test]
fn api_error_from_anyhow_error() {
    let error = anyhow::anyhow!("Error message");

    let actual = ApiError::from(error);

    assert_eq!(actual.status, 500);
    assert_eq!(actual.message.unwrap(), "Error message");
}

#[test]
fn api_error_from_serde_error() {
    let error = serde_json::from_str::<MultisigTransaction>("{").expect_err("Error message");
    let error_message = format!("{:?}", &error);

    let actual = ApiError::from(error);

    assert_eq!(actual.status, 500);
    assert_eq!(actual.message.unwrap(), error_message);
}