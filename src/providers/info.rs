use crate::config::{base_transaction_service_url, safe_info_cache_duration, token_info_cache_duration};
use crate::utils::context::{Context};
use crate::cache::{ServiceCache};
use serde_json;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use ethereum_types::{Address};
use std::collections::HashMap;

pub struct InfoProvider<'p> {
    client: &'p reqwest::blocking::Client,
    cache: ServiceCache,
    safe_cache: HashMap<String, SafeInfo>,
    token_cache: HashMap<String, TokenInfo>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SafeInfo {
    nonce: u64,
    threshold: u64,
    owners: Vec<Address>
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct TokenInfo {
    token_type: String,
    decimals: u64,
    symbol: String,
    name: String,
    logo_uri: Option<String>
}

macro_rules! with_in_memory_cache {
    ($this:ident.$cache:ident, $key:expr, $factory:expr) => {
        match $this.$cache.get($key) {
            Some(value) => Ok(value.clone()),
            None => {
                let value = $factory($key)?;
                $this.$cache.insert($key.to_owned(), value.clone());
                Ok(value)
            }
        }
    }
}

impl InfoProvider<'_> {
    pub fn new<'p>(context: &'p Context) -> InfoProvider<'p> {
        let provider = InfoProvider {
            client: context.client(), cache: context.cache(), 
            safe_cache: HashMap::new(), token_cache: HashMap::new(), 
        };
        return provider;
    }

    pub fn safe_info(&mut self, safe: &String) -> Result<SafeInfo> {
        let url = format!(
            "{}/safes/{}/",
            base_transaction_service_url(),
            safe
        );
        with_in_memory_cache!(self.safe_cache, &url, |url: &String| -> Result<SafeInfo> {
            let data = self.cache.request_cached(self.client, &url, safe_info_cache_duration())?;
            Ok(serde_json::from_str(&data)?)
        })
    }

    pub fn token_info(&mut self, token: &String) -> Result<TokenInfo> {
        let url = format!(
            "{}/tokens/{}/",
            base_transaction_service_url(),
            token
        );
        with_in_memory_cache!(self.token_cache, &url, |url: &String| -> Result<TokenInfo> {
            let data = self.cache.request_cached(self.client, &url, token_info_cache_duration())?;
            Ok(serde_json::from_str(&data)?)
        })
    }
}