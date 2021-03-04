use crate::config::{
    address_info_cache_duration, base_transaction_service_url, exchange_api_cache_duration,
    long_error_duration, request_cache_duration, safe_app_manifest_cache_duration,
    safe_info_cache_duration, short_error_duration, token_info_cache_duration,
};
use crate::models::commons::Page;
use crate::providers::address_info::{AddressInfo, ContractInfo};
use crate::utils::cache::{Cache, CacheExt};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use crate::utils::json::default_if_null;
use mockall::automock;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;

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
    pub address: String,
    pub nonce: u64,
    pub threshold: u64,
    pub owners: Vec<String>,
    pub master_copy: String,
    pub modules: Option<Vec<String>>,
    pub fallback_handler: Option<String>,
    pub version: Option<String>,
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

impl InfoProvider for DefaultInfoProvider<'_> {
    fn safe_info(&mut self, safe: &str) -> ApiResult<SafeInfo> {
        self.cached(
            |this| &mut this.safe_cache,
            DefaultInfoProvider::load_safe_info,
            safe,
        )
    }

    fn token_info(&mut self, token: &str) -> ApiResult<TokenInfo> {
        if token != "0x0000000000000000000000000000000000000000" {
            self.cached(
                |this| &mut this.token_cache,
                DefaultInfoProvider::load_token_info,
                token,
            )
        } else {
            bail!("Token Address is 0x0")
        }
    }

    fn safe_app_info(&mut self, url: &str) -> ApiResult<SafeAppInfo> {
        let manifest_url = format!("{}/manifest.json", url);
        let manifest_json = self.cache.request_cached(
            self.client,
            &manifest_url,
            safe_app_manifest_cache_duration(),
            long_error_duration(),
        )?;
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
                let contract_info_json = self.cache.request_cached(
                    self.client,
                    &url,
                    address_info_cache_duration(),
                    long_error_duration(),
                )?;
                let contract_info = serde_json::from_str::<ContractInfo>(&contract_info_json)?;
                if contract_info.display_name.trim().is_empty() {
                    bail!("No display name")
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
        generator: impl Fn(&mut Self, &String) -> ApiResult<Option<T>>,
        key: impl Into<String>,
    ) -> ApiResult<T>
    where
        T: Clone + DeserializeOwned,
    {
        let key = key.into();
        match local_cache(self).get(&key) {
            Some(value) => value
                .clone()
                .ok_or(api_error!("Cached value not available")),
            None => {
                let value: Option<T> = generator(self, &key)?;
                local_cache(self).insert(key, value.clone());
                value.ok_or(api_error!("Could not generate value"))
            }
        }
    }

    fn load_safe_info(&mut self, safe: &String) -> ApiResult<Option<SafeInfo>> {
        let url = format!("{}/v1/safes/{}/", base_transaction_service_url(), safe);
        let data: String = self.cache.request_cached(
            self.client,
            &url,
            safe_info_cache_duration(),
            short_error_duration(),
        )?;
        Ok(serde_json::from_str(&data).unwrap_or(None))
    }

    fn populate_token_cache(&mut self) -> ApiResult<()> {
        let url = format!("{}/v1/tokens/?limit=10000", base_transaction_service_url());
        let response = self.client.get(&url).send()?;
        let data: Page<TokenInfo> = response.json()?;
        for token in data.results.iter() {
            self.cache.create(
                &format!("dip_ti_{}", token.address),
                &serde_json::to_string(&token)?,
                request_cache_duration(),
            )
        }
        Ok(())
    }

    fn check_token_cache(&mut self) -> ApiResult<()> {
        if self.cache.fetch("dip_tcl").is_some() {
            // Cache is still up to data
            return Ok(());
        }
        let result = self.populate_token_cache();
        // If error we use a shorter cache timeout (we do not want to DoS our service in case of an error)
        self.cache.create(
            "dip_tcl",
            "",
            if result.is_ok() {
                token_info_cache_duration()
            } else {
                short_error_duration()
            },
        );
        result
    }

    // TODO: Check Eviction policies: https://redis.io/topics/lru-cache
    fn load_token_info(&mut self, token: &String) -> ApiResult<Option<TokenInfo>> {
        self.check_token_cache()?;
        match self.cache.fetch(&format!("dip_ti_{}", token)) {
            Some(cached) => Ok(Some(serde_json::from_str::<TokenInfo>(&cached)?)),
            None => Ok(None),
        }
    }

    pub fn exchange_usd_to(&self, currency_code: &str) -> ApiResult<f64> {
        let currency_code = currency_code.to_uppercase();
        let exchange = self.fetch_exchange()?;
        match exchange.rates {
            Some(rates) => rates
                .get(&currency_code)
                .cloned()
                .ok_or(api_error!("Currency not found")),
            None => Err(api_error!("Currency not found")),
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
        let body = self.cache.request_cached(
            self.client,
            &url,
            exchange_api_cache_duration(),
            short_error_duration(),
        )?;
        Ok(serde_json::from_str::<Exchange>(&body)?)
    }
}
