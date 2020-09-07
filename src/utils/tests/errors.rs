use rocket::local::Client;
use crate::utils::errors::{ApiError, ApiErrorMessage, BackendError};
use rocket::response::Responder;
use crate::models::backend::transactions::MultisigTransaction;

#[test]
fn api_error_responder_json() {
    let api_error = ApiError { status: 418, message: ApiErrorMessage::SingleLine(String::from("Not found")) };
    let rocket = rocket::ignite();
    let client = Client::new(rocket).expect("valid rocket instance");

    let local_request = client.get("/");
    let request = local_request.inner();
    let mut response = api_error.respond_to(&request).unwrap();
    let body_json = response.body().unwrap().into_string().unwrap();

    assert_eq!(response.status().code, 418);
    assert_eq!(body_json, "{\"status\":418,\"message\":\"Not found\"}");
}

#[test]
fn api_error_from_anyhow_error() {
    let error = anyhow::anyhow!("Error message");

    let actual = ApiError::from(error);

    assert_eq!(actual.status, 500);
    assert_eq!(actual.message, ApiErrorMessage::SingleLine(String::from("Error message")));
}

#[test]
fn api_error_from_serde_error() {
    let error = serde_json::from_str::<MultisigTransaction>("{").expect_err("Error message");
    let error_message = format!("{:?}", &error);

    let actual = ApiError::from(error);

    assert_eq!(actual.status, 500);
    assert_eq!(actual.message, ApiErrorMessage::SingleLine(error_message));
}

#[test]
fn api_error_known_error_json_structure() {
    let expected_error_json = r#"{
        "code": 1,
        "message": "Checksum address validation failed",
        "arguments": [
          "0x1230b3d59858296A31053C1b8562Ecf89A2f888b"
        ]
    }"#;
    // let expected_error = serde_json::from_str::<BackendError>(&expected_error_json).unwrap();
    let expected_error = BackendError {
        code: 1,
        message: Some("Checksum address validation failed".to_string()),
        arguments: Some(vec!["0x1230b3d59858296A31053C1b8562Ecf89A2f888b".to_string()]),
    };

    let actual = ApiError::from_backend_error(422, &expected_error_json);

    assert_eq!(actual.status, 422);
    match actual.message {
        ApiErrorMessage::BackendError(backend_error) => {
            assert_eq!(backend_error, expected_error);
        }
        ApiErrorMessage::SingleLine(_) => { panic!("Failed to deserialize error"); }
    }
}

#[test]
fn api_error_unknown_error_json_structure() {
    let expected_error_json = r#"{
        "code": 1,
        "message": ["Checksum address validation failed"],
        "arguments": [
          "0x1230b3d59858296A31053C1b8562Ecf89A2f888b"
        ]
    }"#;

    let actual = ApiError::from_backend_error(422, &expected_error_json);

    assert_eq!(actual.status, 1337);
    match actual.message {
        ApiErrorMessage::BackendError(_) => {
            panic!("Failed to deserialize error");
        }
        ApiErrorMessage::SingleLine(message) => {
            assert_eq!(message, expected_error_json);
        }
    }
}