use crate::cache::cache_op_executors::{
    CACHE_REQS_PREFIX, CACHE_REQS_RESP_PREFIX, CACHE_RESP_PREFIX,
};
use crate::cache::cache_operations::{InvalidationPattern, InvalidationScope};
use crate::providers::info::TOKENS_KEY;

#[test]
fn invalidation_pattern_any_string() {
    let invalidation_pattern =
        InvalidationPattern::Any("some_address".to_string(), InvalidationScope::Both);
    let expected = format!("{}*some_address*", CACHE_REQS_RESP_PREFIX);

    let actual = invalidation_pattern.to_pattern_string();

    assert_eq!(expected, actual);
}

#[test]
fn invalidation_pattern_transactions_string() {
    let invalidation_pattern =
        InvalidationPattern::Transactions("some_address".to_string(), InvalidationScope::Both);
    let expected = format!("{}*transactions/*some_address", CACHE_REQS_RESP_PREFIX);

    let actual = invalidation_pattern.to_pattern_string();

    assert_eq!(expected, actual);
}

#[test]
fn invalidation_pattern_tokens_string() {
    let invalidation_pattern = InvalidationPattern::Tokens;
    let expected = TOKENS_KEY.to_string();

    let actual = invalidation_pattern.to_pattern_string();

    assert_eq!(expected, actual);
}

#[test]
fn invalidation_pattern_contracts_string() {
    let invalidation_pattern = InvalidationPattern::Contracts;
    let expected = String::from("*contract*");

    let actual = invalidation_pattern.to_pattern_string();

    assert_eq!(expected, actual);
}

#[test]
fn invalidation_pattern_balances_string() {
    let invalidation_pattern =
        InvalidationPattern::Balances("some_address".to_string(), InvalidationScope::Both);
    let expected = format!("{}*/some_address/balances*", CACHE_REQS_RESP_PREFIX);

    let actual = invalidation_pattern.to_pattern_string();

    assert_eq!(expected, actual);
}

#[test]
fn invalidation_pattern_collectibles_string() {
    let invalidation_pattern =
        InvalidationPattern::Collectibles("some_address".to_string(), InvalidationScope::Both);
    let expected = format!("{}*/some_address/collectibles*", CACHE_REQS_RESP_PREFIX);

    let actual = invalidation_pattern.to_pattern_string();

    assert_eq!(expected, actual);
}

#[test]
fn invalidation_scope_both_to_string() {
    assert_eq!(
        CACHE_REQS_RESP_PREFIX,
        InvalidationScope::Both.invalidation_scope_string()
    )
}

#[test]
fn invalidation_scope_requests_to_string() {
    assert_eq!(
        CACHE_REQS_PREFIX,
        InvalidationScope::Requests.invalidation_scope_string()
    )
}

#[test]
fn invalidation_scope_responses_to_string() {
    assert_eq!(
        CACHE_RESP_PREFIX,
        InvalidationScope::Responses.invalidation_scope_string()
    )
}
