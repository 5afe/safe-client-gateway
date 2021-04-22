use crate::models::backend::transactions::MultisigTransaction;
use crate::rocket::response::Responder;
use crate::utils::errors::{ApiError, ErrorDetails};
use rocket::local::asynchronous::{Client, LocalRequest};

//TODO what is this tests doing...
#[rocket::async_test]
async fn api_error_responder_json() {
    let api_error = ApiError {
        status: 418,
        details: ErrorDetails {
            code: 1337,
            message: Some("Not found".to_string()),
            arguments: None,
        },
    };
    let expected_error_json = r#"{"code":1337,"message":"Not found"}"#;

    let rocket = rocket::build();
    let client = Client::debug(rocket).await.expect("valid rocket instance");
    let request = client.get("/");
    let mut response = api_error.respond_to(&request).unwrap();

    let status_code: u16 = response.status().code;
    let body_json = &response.body_string().await.unwrap();

    assert_eq!(status_code, 418);
    assert_eq!(body_json, expected_error_json);
}

#[test]
fn api_error_from_anyhow_error() {
    let error = api_error!("Error message");
    let error_details = ErrorDetails {
        code: 1337,
        message: Some("Error message".to_string()),
        arguments: None,
    };

    let actual = ApiError::from(error);

    assert_eq!(actual.status, 500);
    assert_eq!(actual.details, error_details);
}

#[test]
fn api_error_from_serde_error() {
    let error = serde_json::from_str::<MultisigTransaction>("{").expect_err("Error message");
    let error_details = ErrorDetails {
        code: 1337,
        message: Some(format!("{:?}", &error)),
        arguments: None,
    };

    let actual = ApiError::from(error);

    assert_eq!(actual.status, 500);
    assert_eq!(actual.details, error_details);
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
    let expected_error = ErrorDetails {
        code: 1,
        message: Some("Checksum address validation failed".to_string()),
        arguments: Some(vec![
            "0x1230b3d59858296A31053C1b8562Ecf89A2f888b".to_string()
        ]),
    };

    let actual = ApiError::from_backend_error(422, &expected_error_json);

    assert_eq!(actual.status, 422);
    assert_eq!(actual.details, expected_error);
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
    let expected_error = ErrorDetails {
        code: 42,
        message: Some(expected_error_json.to_owned()),
        arguments: None,
    };

    let actual = ApiError::from_backend_error(422, &expected_error_json);

    assert_eq!(actual.status, 422);
    assert_eq!(actual.details, expected_error);
}
