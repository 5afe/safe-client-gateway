use crate::config::{
    base_transaction_service_url, exchange_api_cache_duration, info_cache_duration, long_cache,
};
use crate::utils::cache::{Cache, CacheExt};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use crate::utils::json::default_if_null;
use anyhow::Result;
use mockall::automock;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[automock]
pub trait InfoProvider {
    fn safe_info(&mut self, safe: &str) -> Result<SafeInfo>;
    fn token_info(&mut self, token: &str) -> Result<TokenInfo>;
    fn raw_request(&mut self, url: &str) -> ApiResult<String>;
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

    fn raw_request(&mut self, url: &str) -> ApiResult<String> {
        self.cache.request_cached(self.client, &url, long_cache())
    }
}

impl DefaultInfoProvider<'_> {
    pub fn exchange_usd_to(&self, currency_code: &str) -> Result<f64> {
        let currency_code = currency_code.to_uppercase();
        let exchange = self.fetch_exchange()?;
        match exchange.rates {
            Some(rates) => rates
                .get(&currency_code)
                .cloned()
                .ok_or(anyhow::anyhow!("Currency not found")),
            None => Err(anyhow::anyhow!("Currency not found")),
        }
    }

    pub fn available_currency_codes(&self) -> ApiResult<Vec<String>> {
        let exchange = self.fetch_exchange()?;
        Ok(exchange
            .rates
            .map_or(vec![], |s| s.keys().cloned().collect::<Vec<_>>()))
    }

    fn fetch_exchange(&self) -> Result<Exchange> {
        let url = format!("https://api.exchangeratesapi.io/latest?base=USD");
        let body = self
            .cache
            .request_cached(self.client, &url, exchange_api_cache_duration())?;
        Ok(serde_json::from_str::<Exchange>(&body)?)
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
            Some(value) => value
                .clone()
                .ok_or(anyhow::anyhow!("Could not decode cached")),
            None => {
                let data = self
                    .cache
                    .request_cached(self.client, &url, info_cache_duration())?;
                let value: Option<T> = serde_json::from_str(&data).unwrap_or(None);
                local_cache(self).insert(url, value.clone());
                value.ok_or(anyhow::anyhow!("Could not decode response"))
            }
        }
    }
}
