use std::env;

pub fn base_transaction_service_url() -> String {
    format!("{}{}", env::var("TRANSACTION_SERVICE_URL").unwrap(), "/api")
}

pub fn base_exchange_api_url() -> String {
    format!(
        "{}?access_key={}",
        env::var("EXCHANGE_API_BASE_URL").unwrap(),
        env::var("EXCHANGE_API_KEY").unwrap()
    )
}

pub fn webhook_token() -> String {
    env::var("WEBHOOK_TOKEN").unwrap()
}

pub fn scheme() -> String {
    env::var("SCHEME").unwrap_or(String::from("https"))
}

fn usize_with_default(key: &str, default: usize) -> usize {
    match env::var(key) {
        Ok(value) => value.parse().unwrap(),
        Err(_) => default,
    }
}

fn u64_with_default(key: &str, default: u64) -> u64 {
    match env::var(key) {
        Ok(value) => value.parse().unwrap(),
        Err(_) => default,
    }
}

fn bool_with_default(key: &str, default: bool) -> bool {
    match env::var(key) {
        Ok(value) => value.parse().unwrap(),
        Err(_) => default,
    }
}

// TIME DURATION VALUES
fn indefinite_timeout() -> usize {
    usize_with_default("INDEFINITE_TIMEOUT", 60 * 60)
}

pub fn short_error_duration() -> usize {
    usize_with_default("SHORT_ERROR_DURATION", 60)
}

pub fn long_error_duration() -> usize {
    usize_with_default("LONG_ERROR_DURATION", 60 * 15)
}

// FUNCTIONAL TIMEOUTS
pub fn safe_info_cache_duration() -> usize {
    usize_with_default("SAFE_INFO_CACHE_DURATION", indefinite_timeout())
}

pub fn address_info_cache_duration() -> usize {
    usize_with_default("ADDRESS_INFO_CACHE_DURATION", indefinite_timeout())
}

pub fn token_info_cache_duration() -> usize {
    usize_with_default("TOKEN_INFO_CACHE_DURATION", 60 * 60 * 24)
}

pub fn exchange_api_cache_duration() -> usize {
    usize_with_default("EXCHANGE_API_CACHE_DURATION", 60 * 60 * 12)
}

pub fn request_cache_duration() -> usize {
    usize_with_default("REQUEST_CACHE_DURATION", indefinite_timeout())
}

pub fn about_cache_duration() -> usize {
    usize_with_default("ABOUT_CACHE_DURATION", 60 * 15)
}

pub fn balances_cache_duration() -> usize {
    usize_with_default("BALANCES_REQUEST_CACHE_DURATION", 60)
}

pub fn safe_app_manifest_cache_duration() -> usize {
    usize_with_default("SAFE_APP_MANIFEST_CACHE_DURATION", indefinite_timeout())
}

// REQUEST TIMEOUTS in milliseconds
pub fn internal_client_connect_timeout() -> u64 {
    u64_with_default("INTERNAL_CLIENT_CONNECT_TIMEOUT", 1000)
}

pub fn safe_app_info_request_timeout() -> u64 {
    u64_with_default("SAFE_APP_INFO_REQUEST_TIMEOUT", 3000)
}

pub fn transaction_request_timeout() -> u64 {
    u64_with_default("TRANSACTION_REQUEST_TIMEOUT", 30000)
}

pub fn safe_info_request_timeout() -> u64 {
    u64_with_default("SAFE_INFO_REQUEST_TIMEOUT", 10000)
}

pub fn token_info_request_timeout() -> u64 {
    u64_with_default("TOKEN_INFO_REQUEST_TIMEOUT", 30000)
}

pub fn default_request_timeout() -> u64 {
    u64_with_default("DEFAULT_REQUEST_TIMEOUT", 5000)
}

// ERRORS
pub fn request_error_cache_timeout() -> usize {
    usize_with_default("REQS_ERROR_CACHE_DURATION", short_error_duration())
}

pub fn log_all_error_responses() -> bool {
    bool_with_default("LOG_ALL_ERROR_RESPONSES", false)
}

// OTHERS
pub fn redis_scan_count() -> usize {
    usize_with_default("REDIS_SCAN_COUNT", 300)
}

pub fn build_number() -> Option<String> {
    option_env!("BUILD_NUMBER").map(|it| it.to_string())
}

pub fn native_coin_decimals() -> u64 {
    u64_with_default("NATIVE_COIN_DECIMALS", 18)
}

pub fn native_coin_symbol() -> String {
    env::var("NATIVE_COIN_SYMBOL")
        .unwrap_or(String::from("ETH"))
        .to_string()
}

pub fn native_coin_name() -> String {
    env::var("NATIVE_COIN_NAME")
        .unwrap_or(String::from("Ether"))
        .to_string()
}

pub fn version() -> String {
    option_env!("VERSION")
        .unwrap_or(env!("CARGO_PKG_VERSION"))
        .to_string()
}
