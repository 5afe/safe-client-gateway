use crate::cache::cache_operations::{InvalidationPattern, InvalidationScope};
use crate::cache::{CACHE_REQS_PREFIX, CACHE_REQS_RESP_PREFIX, CACHE_RESP_PREFIX};
use crate::config::base_config_service_url;
use crate::providers::info::TOKENS_KEY_BASE;

#[test]
fn invalidation_pattern_any_string() {
    let invalidation_pattern =
        InvalidationPattern::Any(InvalidationScope::Both, "some_address".to_string());
    let expected = format!("{}*some_address*", CACHE_REQS_RESP_PREFIX);

    let actual = invalidation_pattern.to_pattern_string();

    assert_eq!(expected, actual);
}

#[test]
fn invalidation_pattern_transactions_string() {
    let invalidation_pattern =
        InvalidationPattern::Transactions(InvalidationScope::Both, "some_address".to_string());
    let expected = format!("{}*/some_address/*transactions/*", CACHE_REQS_RESP_PREFIX);

    let actual = invalidation_pattern.to_pattern_string();

    assert_eq!(expected, actual);
}

#[test]
fn invalidation_pattern_transfers_string() {
    let invalidation_pattern =
        InvalidationPattern::Transfers(InvalidationScope::Requests, "some_address".to_string());
    let expected = format!("{}*/some_address/*transfer*", CACHE_REQS_PREFIX);

    let actual = invalidation_pattern.to_pattern_string();

    assert_eq!(expected, actual);
}

#[test]
fn invalidation_pattern_tokens_string() {
    let invalidation_pattern = InvalidationPattern::Tokens {
        chain_id: "4".to_string(),
    };
    let expected = format!("{}_{}", TOKENS_KEY_BASE.to_string(), "4");

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
        InvalidationPattern::Balances(InvalidationScope::Both, "some_address".to_string());
    let expected = format!("{}*/some_address/balances*", CACHE_REQS_RESP_PREFIX);

    let actual = invalidation_pattern.to_pattern_string();

    assert_eq!(expected, actual);
}

#[test]
fn invalidation_pattern_collectibles_string() {
    let invalidation_pattern =
        InvalidationPattern::Collectibles(InvalidationScope::Both, "some_address".to_string());
    let expected = format!("{}*/some_address/collectibles*", CACHE_REQS_RESP_PREFIX);

    let actual = invalidation_pattern.to_pattern_string();

    assert_eq!(expected, actual);
}

#[test]
fn invalidation_pattern_chains_string() {
    std::env::set_var("CONFIG_SERVICE_URL", "https://config-url-example.com");
    let invalidation_pattern = InvalidationPattern::Chains;
    let expected = format!("*{}*", base_config_service_url());

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
