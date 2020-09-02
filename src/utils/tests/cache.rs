use crate::utils::cache::CachedWithCode;

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
fn cache_with_code_join() {
    let actual = CachedWithCode::join(400, "data");
    let expected = String::from("400;data");

    assert_eq!(actual, expected);
}

#[test]
fn cache_with_code_error_code() {
    let cached_with_code = CachedWithCode{ code: 418, data: "teapot".to_string() };

    assert!(cached_with_code.is_error())
}

#[test]
fn cache_with_code_success_code() {
    let cached_with_code = CachedWithCode{ code: 200, data: "not a teapot".to_string() };

    assert!(!cached_with_code.is_error())
}