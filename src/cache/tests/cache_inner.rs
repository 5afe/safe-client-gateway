use crate::cache::inner_cache::CachedWithCode;
use crate::utils::errors::{ApiError, ErrorDetails};

#[test]
fn cache_with_code_split_success() {
    let input = "400;123";

    let cached_with_code = CachedWithCode::split(input);
    let expected = CachedWithCode {
        code: 400,
        data: String::from("123"),
    };

    assert_eq!(cached_with_code, expected);
}

#[test]
#[should_panic]
fn cache_with_code_split_failure_parse() {
    CachedWithCode::split("400A;123");
}

#[test]
#[should_panic]
fn cache_with_code_split_failure_not_enough_parts() {
    CachedWithCode::split("400MissingSeparatorForSomeReason");
}

#[test]
fn cache_with_code_split_multiple_separators() {
    let expected = CachedWithCode {
        code: 404,
        data: String::from("foo;bar"),
    };
    let actual = CachedWithCode::split("404;foo;bar");

    assert_eq!(actual, expected);
}

#[test]
fn cache_with_code_split_data_is_only_separators() {
    let expected = CachedWithCode {
        code: 404,
        data: String::from(";;;;;"),
    };
    let actual = CachedWithCode::split("404;;;;;;");

    assert_eq!(actual, expected);
}

#[test]
fn cache_with_code_join() {
    let actual = CachedWithCode::join(400, "data");
    let expected = String::from("400;data");

    assert_eq!(actual, expected);
}

#[test]
fn cache_with_code_error_code() {
    let cached_with_code = CachedWithCode {
        code: 418,
        data: "teapot".to_string(),
    };

    assert!(cached_with_code.is_error())
}

#[test]
fn cache_with_code_success_code() {
    let cached_with_code = CachedWithCode {
        code: 200,
        data: "not a teapot".to_string(),
    };

    assert!(!cached_with_code.is_error())
}

#[test]
fn cache_with_code_unwrap_ok() {
    let cached_with_code = CachedWithCode {
        code: 200,
        data: "not a teapot".to_string(),
    };

    assert_eq!(cached_with_code.to_result().unwrap(), "not a teapot");
}

#[test]
fn cache_with_code_unwrap_err() {
    let cached_with_code = CachedWithCode {
        code: 418,
        data: "teapot".to_string(),
    };
    let expected = ApiError {
        status: 418,
        details: ErrorDetails {
            code: 42,
            message: Some(String::from("teapot")),
            arguments: None,
        },
    };

    assert_eq!(cached_with_code.to_result().expect_err(""), expected);
}
