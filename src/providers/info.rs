use crate::services::base_transaction_service_url;
use crate::utils::context::{Context};
use crate::cache::{ServiceCache};
use serde_json;
use serde::{Deserialize, Serialize};
use anyhow::Result;
use ethereum_types::{Address};

pub struct InfoProvider<'p> {
    client: &'p reqwest::blocking::Client,
    cache: ServiceCache
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SafeInfo {
    nonce: u64,
    threshold: u64,
    owners: Vec<Address>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TokenInfo {
    token_type: String,
    decimals: u64,
    symbol: String,
    name: String,
    logo_uri: Option<String>
}

impl InfoProvider<'_> {
    pub fn new<'p>(context: &'p Context) -> InfoProvider<'p> {
        let provider = InfoProvider {
            client: context.client(), cache: context.cache()
        };
        return provider;
    }

    pub fn safe_info(&self, safe: &String) -> Result<SafeInfo> {
        let url = format!(
            "{}/safes/{}/",
            base_transaction_service_url(),
            safe
        );
        let data = self.cache.request_cached(self.client, &url, 60*15)?;
        Ok(serde_json::from_str(&data)?)
    }

    pub fn token_info(&self, token: &String) -> Result<TokenInfo> {
        let url = format!(
            "{}/tokens/{}/",
            base_transaction_service_url(),
            token
        );
        let data = self.cache.request_cached(self.client, &url, 60*15)?;
        Ok(serde_json::from_str(&data)?)
    }
}