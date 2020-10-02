use crate::utils::cache::{Cache, CacheExt};
use crate::config::{base_transaction_service_url, info_cache_duration};
use crate::utils::context::Context;
use crate::utils::json::default_if_null;
use serde_json;
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use mockall::automock;
use log::debug;
use crate::utils::errors::ApiError;

#[automock]
pub trait InfoProvider {
    fn safe_info(&mut self, safe: &str) -> Result<SafeInfo>;
    fn token_info(&mut self, token: &str) -> Result<TokenInfo>;
}

pub struct DefaultInfoProvider<'p> {
    client: &'p reqwest::blocking::Client,
    cache: &'p dyn Cache,
    safe_cache: HashMap<String, Option<SafeInfo>>,
    token_cache: HashMap<String, Option<TokenInfo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenType {
    Erc721,
    Erc20,
    Ether,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SafeInfo {
    pub nonce: u64,
    pub threshold: u64,
    pub owners: Vec<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Exchange {
    pub rates: Option<HashMap<String, f64>>,
    pub base: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    #[serde(rename = "type")]
    pub token_type: TokenType,
    pub address: String,
    #[serde(deserialize_with = "default_if_null")]
    pub decimals: u64,
    pub symbol: String,
    pub name: String,
    pub logo_uri: Option<String>,
}

impl InfoProvider for DefaultInfoProvider<'_> {
    fn safe_info(&mut self, safe: &str) -> Result<SafeInfo> {
        let url = format!("{}/v1/safes/{}/", base_transaction_service_url(), safe);
        self.cached(|this| &mut this.safe_cache, url)
    }

    fn token_info(&mut self, token: &str) -> Result<TokenInfo> {
        if token != "0x0000000000000000000000000000000000000000" {
            let url = format!("{}/v1/tokens/{}/", base_transaction_service_url(), token);
            self.cached(|this| &mut this.token_cache, url)
        } else {
            anyhow::bail!("Token Address is 0x0")
        }
    }
}

impl DefaultInfoProvider<'_> {
    pub fn exchange_usd_to(&self, currency_code: &str) -> Result<f64> {
        let currency_code = currency_code.to_uppercase();
        let url = format!("https://api.exchangeratesapi.io/latest?base=USD&symbols={}", &currency_code);
        debug!("exchange url: {}", &url);
        let request = self.client.get(&url).send()?;
        let body = request.text()?;
        let exchange = serde_json::from_str::<Exchange>(&body)?;
        match exchange.rates {
            Some(rate) => rate.get(&currency_code).cloned().ok_or(anyhow::anyhow!("Currency not found")),
            None => Err(anyhow::anyhow!("Currency not found")),
        }
    }
}

impl DefaultInfoProvider<'_> {
    pub fn new<'p>(context: &'p Context) -> DefaultInfoProvider<'p> {
        DefaultInfoProvider {
            client: context.client(),
            cache: context.cache(),
            safe_cache: HashMap::new(),
            token_cache: HashMap::new(),
        }
    }

    fn cached<T>(
        &mut self,
        local_cache: impl Fn(&mut Self) -> &mut HashMap<String, Option<T>>,
        url: impl Into<String>,
    ) -> Result<T>
        where
            T: Clone + DeserializeOwned,
    {
        let url = url.into();
        match local_cache(self).get(&url) {
            Some(value) => value.clone().ok_or(anyhow::anyhow!("Could not decode cached")),
            None => {
                let data = self.cache.request_cached(self.client, &url, info_cache_duration())?;
                let value: Option<T> = serde_json::from_str(&data).unwrap_or(None);
                local_cache(self).insert(url, value.clone());
                value.ok_or(anyhow::anyhow!("Could not decode response"))
            }
        }
    }
}
