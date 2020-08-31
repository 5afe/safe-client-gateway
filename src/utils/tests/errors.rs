use rocket::local::Client;
use rocket::http::Status;
use crate::utils::errors::{ApiError, ApiResult};

#[get("/not_found")]
fn not_found() -> ApiResult<()> {
    Err(ApiError {
        status: 404,
        message: Some("Not found".to_string()),
    })
}

#[get("/reqwest_error")]
fn reqwest_error() -> ApiResult<String> {
    let invalid_request = "http://";
    Ok(reqwest::blocking::get(invalid_request)?.text()?)
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![not_found, reqwest_error])
}

#[test]
fn api_error_not_found() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/not_found").dispatch();
    assert_eq!(response.status(), Status::from_code(404).unwrap());
    assert_eq!(response.body_string(), Some("{\"status\":404,\"message\":\"Not found\"}".into()));
}

#[test]
fn api_error_reqwest_error() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/reqwest_error").dispatch();
    assert_eq!(response.status(), Status::from_code(500).unwrap());
    assert_eq!(response.body_string(), Some("{\"status\":500,\"message\":\"reqwest::Error { kind: Builder, source: EmptyHost }\"}".into()));
}


