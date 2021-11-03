use crate::cache::cache_operations::{Invalidate, InvalidationPattern, InvalidationScope};
use crate::cache::manager::{CacheManager, RedisCacheManager};
use crate::cache::{
    Cache, MockCache, CACHE_REQS_PREFIX, CACHE_REQS_RESP_PREFIX, CACHE_RESP_PREFIX,
};
use crate::config::base_config_service_uri;
use crate::providers::info::TOKENS_KEY_BASE;
use std::sync::Arc;

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
    std::env::set_var("CONFIG_SERVICE_URI", "https://config-url-example.com");
    let invalidation_pattern = InvalidationPattern::Chains;
    let expected = format!("*{}*", base_config_service_uri());

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

#[test]
fn invalidate_cache_for() {
    let info_cache = Arc::new(MockCache::new()) as Arc<dyn Cache>;
    let default_cache = Arc::new(MockCache::new()) as Arc<dyn Cache>;
    let cache_manager = Arc::new(RedisCacheManager::new_with_mocks(
        &info_cache,
        &default_cache,
    )) as Arc<dyn CacheManager>;
    let invalidation_patterns = vec![
        InvalidationPattern::Any(InvalidationScope::Both, String::from("")),
        InvalidationPattern::Transactions(InvalidationScope::Both, String::from("")),
        InvalidationPattern::Balances(InvalidationScope::Both, String::from("")),
        InvalidationPattern::Collectibles(InvalidationScope::Both, String::from("")),
        InvalidationPattern::Transfers(InvalidationScope::Both, String::from("")),
        InvalidationPattern::Chains,
        InvalidationPattern::Contracts,
        InvalidationPattern::Tokens {
            chain_id: String::from("4"),
        },
    ];

    for invalidation_pattern in invalidation_patterns.iter() {
        let cache = Invalidate::cache_for(invalidation_pattern, cache_manager.clone());
        match invalidation_pattern {
            InvalidationPattern::Chains
            | InvalidationPattern::Contracts
            | InvalidationPattern::Tokens { .. } => {
                assert!(
                    Arc::ptr_eq(&cache.clone(), &info_cache),
                    "Failed pattern was {:#?}",
                    &invalidation_pattern
                )
            }
            _ => assert!(
                Arc::ptr_eq(&cache.clone(), &default_cache),
                "Failed pattern was {:#?}",
                &invalidation_pattern
            ),
        }
    }
}
