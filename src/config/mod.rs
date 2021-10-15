use std::env;
use std::str::FromStr;

#[cfg(test)]
mod tests;

pub fn redis_uri() -> String {
    env::var("REDIS_URI").expect("REDIS_URI missing in env")
}

pub fn base_config_service_uri() -> String {
    format!("{}{}", env::var("CONFIG_SERVICE_URI").unwrap(), "/api")
}

pub fn base_exchange_api_uri() -> String {
    format!(
        "{}?access_key={}",
        env::var("EXCHANGE_API_BASE_URI").unwrap(),
        env::var("EXCHANGE_API_KEY").unwrap()
    )
}

pub fn webhook_token() -> String {
    env::var("WEBHOOK_TOKEN").expect("WEBHOOK_TOKEN missing in env")
}

pub fn scheme() -> String {
    env_with_default("SCHEME", "https".into())
}

// TIME DURATION VALUES
fn indefinite_timeout() -> usize {
    env_with_default("INDEFINITE_TIMEOUT", 60 * 60 * 1000)
}

pub fn short_error_duration() -> usize {
    env_with_default("SHORT_ERROR_DURATION", 60 * 1000)
}

pub fn long_error_duration() -> usize {
    env_with_default("LONG_ERROR_DURATION", 60 * 15 * 1000)
}

// FUNCTIONAL TIMEOUTS
pub fn safe_info_cache_duration() -> usize {
    env_with_default("SAFE_INFO_CACHE_DURATION", indefinite_timeout())
}

pub fn address_info_cache_duration() -> usize {
    env_with_default("ADDRESS_INFO_CACHE_DURATION", indefinite_timeout())
}

pub fn token_info_cache_duration() -> usize {
    env_with_default("TOKEN_INFO_CACHE_DURATION", 60 * 60 * 24 * 1000)
}

pub fn chain_info_cache_duration() -> usize {
    env_with_default("CHAIN_INFO_CACHE_DURATION", indefinite_timeout())
}

pub fn chain_info_response_cache_duration() -> usize {
    env_with_default("CHAIN_INFO_RESPONSE_CACHE_DURATION", 1) // set to negligible value
}

pub fn exchange_api_cache_duration() -> usize {
    env_with_default("EXCHANGE_API_CACHE_DURATION", 60 * 60 * 12 * 1000)
}

pub fn request_cache_duration() -> usize {
    env_with_default("REQUEST_CACHE_DURATION", indefinite_timeout())
}

pub fn about_cache_duration() -> usize {
    env_with_default("ABOUT_CACHE_DURATION", 60 * 15 * 1000)
}

pub fn balances_cache_duration() -> usize {
    env_with_default("BALANCES_REQUEST_CACHE_DURATION", 60 * 1000)
}

pub fn safe_app_manifest_cache_duration() -> usize {
    env_with_default("SAFE_APP_MANIFEST_CACHE_DURATION", indefinite_timeout())
}

pub fn owners_for_safes_cache_duration() -> usize {
    env_with_default("OWNERS_FOR_SAFES_CACHE_DURATION", 60 * 1000)
}

pub fn safe_apps_cache_duration() -> usize {
    env_with_default("SAFE_APPS_CACHE_DURATION", indefinite_timeout())
}

pub fn token_price_cache_duration() -> usize {
    env_with_default("TOKEN_PRICE_CACHE_DURATION", 1) // set to negligible value
}

// REQUEST TIMEOUTS
pub fn internal_client_connect_timeout() -> u64 {
    env_with_default("INTERNAL_CLIENT_CONNECT_TIMEOUT", 1000)
}

pub fn safe_app_info_request_timeout() -> u64 {
    env_with_default("SAFE_APP_INFO_REQUEST_TIMEOUT", 3000)
}

pub fn transaction_request_timeout() -> u64 {
    env_with_default("TRANSACTION_REQUEST_TIMEOUT", 30000)
}

pub fn safe_info_request_timeout() -> u64 {
    env_with_default("SAFE_INFO_REQUEST_TIMEOUT", 10000)
}

pub fn token_info_request_timeout() -> u64 {
    env_with_default("TOKEN_INFO_REQUEST_TIMEOUT", 15000)
}

pub fn chain_info_request_timeout() -> u64 {
    env_with_default("CHAIN_INFO_REQUEST_TIMEOUT", 15000)
}

pub fn contract_info_request_timeout() -> u64 {
    env_with_default("CONTRACT_INFO_REQUEST_TIMEOUT", 3000)
}

pub fn balances_request_timeout() -> u64 {
    env_with_default("BALANCES_REQUEST_TIMEOUT", 20000)
}

pub fn collectibles_request_timeout() -> u64 {
    env_with_default("COLLECTIBLES_REQUEST_TIMEOUT", 20000)
}

pub fn default_request_timeout() -> u64 {
    env_with_default("DEFAULT_REQUEST_TIMEOUT", 10000)
}

// ERRORS
pub fn request_error_cache_duration() -> usize {
    env_with_default("REQS_ERROR_CACHE_DURATION", short_error_duration())
}

pub fn log_all_error_responses() -> bool {
    env_with_default("LOG_ALL_ERROR_RESPONSES", false)
}

// OTHERS
pub fn redis_scan_count() -> usize {
    env_with_default("REDIS_SCAN_COUNT", 300)
}

pub fn feature_flag_nested_decoding() -> bool {
    env_with_default("FEATURE_FLAG_NESTED_DECODING", true)
}

pub fn feature_flag_balances_rate_implementation() -> bool {
    env_with_default("FEATURE_FLAG_BALANCES_RATE_IMPLEMENTATION", false)
}

pub fn vpc_transaction_service_uri() -> bool {
    env_with_default("VPC_TRANSACTION_SERVICE_URI", true)
}

pub fn concurrent_balance_token_requests() -> usize {
    env_with_default("CONCURRENT_BALANCE_TOKEN_REQUESTS", 5)
}

pub fn log_threshold() -> f32 {
    env_with_default("LOG_THRESHOLD", 1.0)
}

pub fn build_number() -> Option<String> {
    option_env!("BUILD_NUMBER").map(|it| it.to_string())
}

pub fn version() -> String {
    option_env!("VERSION")
        .unwrap_or(env!("CARGO_PKG_VERSION"))
        .to_string()
}

fn env_with_default<T: FromStr>(key: &str, default: T) -> T
where
    <T as FromStr>::Err: std::fmt::Debug,
{
    match env::var(key) {
        Ok(value) => value
            .parse()
            .expect(&format!("Parsing of {} env var key failed", &key)),
        Err(_) => default,
    }
}
