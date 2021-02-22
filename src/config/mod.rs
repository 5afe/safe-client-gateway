use std::env;

pub fn base_transaction_service_url() -> String {
    format!("{}{}", env::var("TRANSACTION_SERVICE_URL").unwrap(), "/api")
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

pub fn info_cache_duration() -> usize {
    usize_with_default("INFO_CACHE_DURATION", 60 * 15)
}

pub fn exchange_api_cache_duration() -> usize {
    usize_with_default("EXCHANGE_API_CACHE_DURATION", 60 * 60 * 12)
}

pub fn request_cache_duration() -> usize {
    usize_with_default("REQUEST_CACHE_DURATION", 60 * 15)
}

pub fn about_cache_duration() -> usize {
    usize_with_default("ABOUT_CACHE_DURATION", request_cache_duration())
}

pub fn safe_app_manifest_cache() -> usize {
    usize_with_default("SAFE_APP_MANIFEST_CACHE_DURATION", 60 * 60 * 24 * 7)
}

pub fn internal_client_connect_timeout() -> u64 {
    u64_with_default("INTERNAL_CLIENT_CONNECT_TIMEOUT", 1000)
}

pub fn client_error_cache_duration() -> usize {
    usize_with_default("CLIENT_ERROR_CACHE_DURATION", 60 * 3)
}

pub fn log_all_error_responses() -> bool {
    bool_with_default("LOG_ALL_ERROR_RESPONSES", false)
}

pub fn build_number() -> Option<String> {
    option_env!("BUILD_NUMBER").map(|it| it.to_string())
}

pub fn version() -> String {
    option_env!("VERSION")
        .unwrap_or(env!("CARGO_PKG_VERSION"))
        .to_string()
}
