use rocket::local::Client;
use rocket::http::Status;
use crate::utils::errors::{ApiError, ApiResult};

#[get("/")]
fn not_found() -> ApiResult<()> {
    Err(ApiError {
        status: 404,
        message: Some("Not found".to_string()),
    })
}

fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![not_found])
}

#[test]
fn failure_route() {
    let client = Client::new(rocket()).expect("valid rocket instance");
    let mut response = client.get("/").dispatch();
    assert_eq!(response.status(), Status::from_code(404).unwrap());
    assert_eq!(response.body_string(), Some("{\"status\":404,\"message\":\"Not found\"}".into()));
}


