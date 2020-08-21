use crate::utils::cache::ServiceCache;
use crate::config::{
    base_transaction_service_url, info_cache_duration,
};
use crate::utils::context::Context;
use anyhow::Result;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use mockall::automock;

#[automock]
pub trait InfoProvider {
    fn safe_info(&mut self, safe: &String) -> Result<SafeInfo>;
    fn token_info(&mut self, token: &String) -> Result<TokenInfo>;
}

pub struct DefaultInfoProvider<'p> {
    client: &'p reqwest::blocking::Client,
    cache: &'p ServiceCache,
    safe_cache: HashMap<String, Option<SafeInfo>>,
    token_cache: HashMap<String, Option<TokenInfo>>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenType {
    Erc721,
    Erc20,
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

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    #[serde(rename = "type")]
    pub token_type: TokenType,
    pub address: String,
    pub decimals: u64,
    pub symbol: String,
    pub name: String,
    pub logo_uri: Option<String>,
}

impl InfoProvider for DefaultInfoProvider<'_> {
    fn safe_info(&mut self, safe: &String) -> Result<SafeInfo> {
        let url = format!("{}/safes/{}/", base_transaction_service_url(), safe);
        self.cached(|this| &mut this.safe_cache, url)
    }

    fn token_info(&mut self, token: &String) -> Result<TokenInfo> {
        let url = format!("{}/tokens/{}/", base_transaction_service_url(), token);
        self.cached(|this| &mut this.token_cache, url)
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
                let data =
                    self.cache
                        .request_cached(self.client, &url, info_cache_duration())?;
                let value: Option<T> = serde_json::from_str(&data).unwrap_or(None);
                local_cache(self).insert(url, value.clone());
                value.ok_or(anyhow::anyhow!("Could not decode response"))
            }
        }
    }
}
