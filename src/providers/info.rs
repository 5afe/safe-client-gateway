use crate::config::{
    base_transaction_service_url, exchange_api_cache_duration, info_cache_duration,
    safe_app_manifest_cache,
};
use crate::providers::address_info::{AddressInfo, ContractInfo};
use crate::utils::cache::{Cache, CacheExt};
use crate::utils::context::Context;
use crate::utils::errors::{ApiError, ApiResult};
use crate::utils::json::default_if_null;
use mockall::automock;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

#[automock]
pub trait InfoProvider {
    fn safe_info(&mut self, safe: &str) -> ApiResult<SafeInfo>;
    fn token_info(&mut self, token: &str) -> ApiResult<TokenInfo>;
    fn safe_app_info(&mut self, url: &str) -> ApiResult<SafeAppInfo>;
    fn address_info(&mut self, address: &str) -> ApiResult<AddressInfo>;
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

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SafeAppInfo {
    pub name: String,
    pub url: String,
    pub logo_url: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Manifest {
    pub(super) name: String,
    pub(super) description: String,
    #[serde(rename(deserialize = "iconPath"))]
    pub(super) icon_path: String,
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
    fn safe_info(&mut self, safe: &str) -> ApiResult<SafeInfo> {
        let url = format!("{}/v1/safes/{}/", base_transaction_service_url(), safe);
        self.cached(|this| &mut this.safe_cache, url)
    }

    fn token_info(&mut self, token: &str) -> ApiResult<TokenInfo> {
        if token != "0x0000000000000000000000000000000000000000" {
            let url = format!("{}/v1/tokens/{}/", base_transaction_service_url(), token);
            self.cached(|this| &mut this.token_cache, url)
        } else {
            Err(ApiError::new_from_message("Token Address is 0x0"))
        }
    }

    fn safe_app_info(&mut self, url: &str) -> ApiResult<SafeAppInfo> {
        let manifest_url = format!("{}/manifest.json", url);
        let manifest_json =
            self.cache
                .request_cached(self.client, &manifest_url, safe_app_manifest_cache())?;
        let manifest = serde_json::from_str::<Manifest>(&manifest_json)?;
        Ok(SafeAppInfo {
            name: manifest.name.to_owned(),
            url: url.to_owned(),
            logo_url: format!("{}/{}", url, manifest.icon_path),
        })
    }

    fn address_info(&mut self, address: &str) -> ApiResult<AddressInfo> {
        self.token_info(address)
            .map(|it| AddressInfo {
                name: it.name,
                logo_uri: it.logo_uri.to_owned(),
            })
            .or_else(|_| {
                let url = format!(
                    "{}/v1/contracts/{}/",
                    base_transaction_service_url(),
                    address
                );
                let contract_info_json =
                    self.cache
                        .request_cached(self.client, &url, safe_app_manifest_cache())?;
                let contract_info = serde_json::from_str::<ContractInfo>(&contract_info_json)?;
                if contract_info.display_name.trim().is_empty() {
                    Err(ApiError::new_from_message("No display name"))
                } else {
                    Ok(AddressInfo {
                        name: contract_info.display_name.to_owned(),
                        logo_uri: contract_info.logo_uri.to_owned(),
                    })
                }
            })
    }
}

impl DefaultInfoProvider<'_> {
    pub fn exchange_usd_to(&self, currency_code: &str) -> ApiResult<f64> {
        let currency_code = currency_code.to_uppercase();
        let exchange = self.fetch_exchange()?;
        match exchange.rates {
            Some(rates) => rates
                .get(&currency_code)
                .cloned()
                .ok_or(ApiError::new_from_message("Currency not found")),
            None => Err(ApiError::new_from_message("Currency not found")),
        }
    }

    pub fn available_currency_codes(&self) -> ApiResult<Vec<String>> {
        let exchange = self.fetch_exchange()?;
        Ok(exchange
            .rates
            .map_or(vec![], |s| s.keys().cloned().collect::<Vec<_>>()))
    }

    fn fetch_exchange(&self) -> ApiResult<Exchange> {
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
    ) -> ApiResult<T>
    where
        T: Clone + DeserializeOwned,
    {
        let url = url.into();
        match local_cache(self).get(&url) {
            Some(value) => value
                .clone()
                .ok_or(ApiError::new_from_message("Could not decode cached")),
            None => {
                let data = self
                    .cache
                    .request_cached(self.client, &url, info_cache_duration())?;
                let value: Option<T> = serde_json::from_str(&data).unwrap_or(None);
                local_cache(self).insert(url, value.clone());
                value.ok_or(ApiError::new_from_message("Could not decode response"))
            }
        }
    }
}
