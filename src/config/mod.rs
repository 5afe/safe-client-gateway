use std::env;

pub fn base_transaction_service_url() -> String {
    format!("{}{}", env::var("TRANSACTION_SERVICE_URL").unwrap(), "/api/v1")
}

pub fn scheme() -> String {
    env::var("SCHEME").unwrap_or(String::from("https"))
}

fn usize_with_default(key: &str, default: usize) -> usize {
    match env::var(key) {
        Ok(value) => value.parse().unwrap(),
        Err(_) => default
    }
}

pub fn token_info_cache_duration() -> usize {
    usize_with_default("TOKEN_INFO_CACHE_DURATION", 60 * 15)
}

pub fn safe_info_cache_duration() -> usize {
    usize_with_default("SAFE_INFO_CACHE_DURATION", 60 * 15)
}

pub fn request_cache_duration() -> usize {
    usize_with_default("REQUEST_CACHE_DURATION", 60 * 15)
}

