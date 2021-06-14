use crate::cache::cache_operations::RequestCached;
use crate::cache::redis::ServiceCache;
use crate::cache::Cache;
use crate::config::{
    address_info_cache_duration, base_config_service_url, base_exchange_api_url,
    base_transaction_service_url, chain_info_cache_duration, chain_info_request_timeout,
    exchange_api_cache_duration, long_error_duration, safe_app_info_request_timeout,
    safe_app_manifest_cache_duration, safe_info_cache_duration, safe_info_request_timeout,
    short_error_duration, token_info_cache_duration, token_info_request_timeout,
};
use crate::models::chains::ChainInfo;
use crate::models::commons::Page;
use crate::providers::address_info::{AddressInfo, ContractInfo};
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use crate::utils::json::default_if_null;
use crate::utils::urls::build_manifest_url;
use lazy_static::lazy_static;
use mockall::automock;
use rocket::futures::TryFutureExt;
use rocket::tokio::sync::Mutex;
use semver::Version;
use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::future::Future;
use std::time::Duration;

pub const TOKENS_KEY: &'static str = "dip_ti";
lazy_static! {
    pub static ref SAFE_V_1_3_0: Version = Version::new(1, 3, 0);
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

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SafeInfo {
    pub address: String,
    pub nonce: u64,
    pub threshold: u64,
    pub owners: Vec<String>,
    pub master_copy: String,
    pub modules: Option<Vec<String>>,
    pub fallback_handler: String,
    pub guard: String,
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
#[rocket::async_trait]
pub trait InfoProvider {
    async fn chain_info(&self, chain_id: &str) -> ApiResult<ChainInfo>;
    async fn safe_info(&self, chain_id: &str, safe: &str) -> ApiResult<SafeInfo>;
    async fn token_info(&self, chain_id: &str, token: &str) -> ApiResult<TokenInfo>;
    async fn safe_app_info(&self, chain_id: &str, url: &str) -> ApiResult<SafeAppInfo>;
    async fn contract_info(&self, chain_id: &str, address: &str) -> ApiResult<AddressInfo>;
    async fn full_address_info_search(
        &self,
        chain_id: &str,
        address: &str,
    ) -> ApiResult<AddressInfo>;
}

pub struct DefaultInfoProvider<'p, C: Cache> {
    client: &'p reqwest::Client,
    cache: &'p C,
    // Mutex is an async Mutex, meaning that the lock is non-blocking
    safe_cache: Mutex<HashMap<String, Option<SafeInfo>>>,
    token_cache: Mutex<HashMap<String, Option<TokenInfo>>>,
    chain_cache: Mutex<HashMap<String, Option<ChainInfo>>>,
}

#[rocket::async_trait]
impl<C: Cache> InfoProvider for DefaultInfoProvider<'_, C> {
    async fn chain_info(&self, chain_id: &str) -> ApiResult<ChainInfo> {
        let chain_cache = &mut self.chain_cache.lock().await;
        Self::cached(chain_cache, || self.load_chain_info(chain_id), chain_id).await
    }

    async fn safe_info(&self, chain_id: &str, safe: &str) -> ApiResult<SafeInfo> {
        let safe_cache = &mut self.safe_cache.lock().await;
        Self::cached(
            safe_cache,
            || self.load_safe_info(chain_id, safe.to_string()),
            safe,
        )
        .await
    }

    async fn token_info(&self, chain_id: &str, token: &str) -> ApiResult<TokenInfo> {
        if token != "0x0000000000000000000000000000000000000000" {
            let token_cache = &mut self.token_cache.lock().await;
            Self::cached(
                token_cache,
                || self.load_token_info(chain_id, token.to_string()),
                token,
            )
            .await
        } else {
            bail!("Token Address is 0x0")
        }
    }

    async fn safe_app_info(&self, chain_id: &str, url: &str) -> ApiResult<SafeAppInfo> {
        let manifest_url = build_manifest_url(url)?;

        let manifest_json = RequestCached::new(manifest_url)
            .cache_duration(safe_app_manifest_cache_duration())
            .error_cache_duration(long_error_duration())
            .cache_all_errors()
            .request_timeout(safe_app_info_request_timeout())
            .execute(self.client, self.cache)
            .await?;
        let manifest = serde_json::from_str::<Manifest>(&manifest_json)?;
        Ok(SafeAppInfo {
            name: manifest.name.to_owned(),
            url: url.to_owned(),
            logo_url: format!("{}/{}", url, manifest.icon_path),
        })
    }

    async fn contract_info(&self, chain_id: &str, address: &str) -> ApiResult<AddressInfo> {
        let url = core_uri!(self, chain_id, "/v1/contracts/{}/", address)?;
        let contract_info_json = RequestCached::new(url)
            .cache_duration(address_info_cache_duration())
            .error_cache_duration(long_error_duration())
            .execute(self.client, self.cache)
            .await?;
        let contract_info = serde_json::from_str::<ContractInfo>(&contract_info_json)?;
        if contract_info.display_name.trim().is_empty() {
            bail!("No display name")
        } else {
            Ok(AddressInfo {
                name: contract_info.display_name.to_owned(),
                logo_uri: contract_info.logo_uri.to_owned(),
            })
        }
    }

    async fn full_address_info_search(
        &self,
        chain_id: &str,
        address: &str,
    ) -> ApiResult<AddressInfo> {
        self.token_info(&chain_id, &address)
            .map_ok(|it| AddressInfo {
                name: it.name,
                logo_uri: it.logo_uri,
            })
            .or_else(|_| async move { self.contract_info(&chain_id, &address).await })
            .await
    }
}

impl<'a> DefaultInfoProvider<'a, ServiceCache<'a>> {
    pub fn new(context: &'a Context) -> Self {
        DefaultInfoProvider {
            client: context.client(),
            cache: context.cache(),
            chain_cache: Default::default(),
            safe_cache: Default::default(),
            token_cache: Default::default(),
        }
    }
}

impl<C: Cache> DefaultInfoProvider<'_, C> {
    async fn cached<'a, T, Fut>(
        local_cache: &'a mut HashMap<String, Option<T>>,
        generator: impl FnOnce() -> Fut,
        key: impl Into<String>,
    ) -> ApiResult<T>
    where
        T: Clone + DeserializeOwned + 'a,
        Fut: Future<Output = ApiResult<Option<T>>>,
    {
        let key = key.into();
        match local_cache.get(&key) {
            Some(value) => value
                .clone()
                .ok_or(api_error!("Cached value not available")),
            None => {
                let value: Option<T> = generator().await?;
                local_cache.insert(key, value.clone());
                value.ok_or(api_error!("Could not generate value"))
            }
        }
    }

    async fn load_chain_info(&self, chain_id: &str) -> ApiResult<Option<ChainInfo>> {
        // TODO: revert
        // let url = format!("{}/v1/chains/{}/", base_config_service_url(), chain_id);
        let url = "https://gist.githubusercontent.com/jpalvarezl/2639c9da0d637f1f6c9bbeb9abad340b/raw/0350bd2443c6d89b5ff1cc6d171e2d074c46b01a/chains_by_id.json".to_string();
        let data = RequestCached::new(url)
            .cache_duration(chain_info_cache_duration())
            .error_cache_duration(short_error_duration())
            .request_timeout(chain_info_request_timeout())
            .execute(self.client, self.cache)
            .await?;
        let result = serde_json::from_str::<Option<ChainInfo>>(&data).unwrap_or(None); // what do we do on network not found?
        Ok(result)
    }

    async fn load_safe_info(&self, chain_id: &str, safe: String) -> ApiResult<Option<SafeInfo>> {
        let url = core_uri!(self, chain_id, "/v1/safes/{}/", safe)?;
        let data = RequestCached::new(url)
            .cache_duration(safe_info_cache_duration())
            .error_cache_duration(short_error_duration())
            .request_timeout(safe_info_request_timeout())
            .execute(self.client, self.cache)
            .await?;
        Ok(serde_json::from_str(&data).unwrap_or(None))
    }

    async fn populate_token_cache(&self, chain_id: &str) -> ApiResult<()> {
        let url = core_uri!(self, chain_id, "/v1/tokens/?limit=10000")?;
        let response = self
            .client
            .get(&url)
            .timeout(Duration::from_millis(token_info_request_timeout()))
            .send()
            .await?;
        let data: Page<TokenInfo> = response.json().await?;
        for token in data.results.iter() {
            self.cache
                .insert_in_hash(TOKENS_KEY, &token.address, &serde_json::to_string(&token)?);
        }
        Ok(())
    }

    async fn check_token_cache(&self, chain_id: &str) -> ApiResult<()> {
        if self.cache.has_key(TOKENS_KEY) {
            return Ok(());
        }
        self.cache.insert_in_hash(TOKENS_KEY, "state", "populating");
        let result = self.populate_token_cache(chain_id).await;
        if result.is_ok() {
            self.cache
                .expire_entity(TOKENS_KEY, token_info_cache_duration());
            self.cache.insert_in_hash(TOKENS_KEY, "state", "populated");
        } else {
            self.cache.expire_entity(TOKENS_KEY, short_error_duration());
            self.cache.insert_in_hash(TOKENS_KEY, "state", "errored");
        }
        result
    }

    async fn load_token_info(&self, chain_id: &str, token: String) -> ApiResult<Option<TokenInfo>> {
        self.check_token_cache(chain_id).await?;
        match self.cache.get_from_hash(TOKENS_KEY, &token) {
            Some(cached) => Ok(Some(serde_json::from_str::<TokenInfo>(&cached)?)),
            None => Ok(None),
        }
    }

    pub async fn exchange_usd_to(&self, currency_code: &str) -> ApiResult<f64> {
        if &currency_code.to_lowercase() == "usd" {
            return Ok(1.0);
        }

        let currency_code = currency_code.to_uppercase();
        let exchange = self.fetch_exchange().await?;
        match exchange.rates {
            Some(rates) => {
                let base_to_usd = rates.get("USD").unwrap_or(&0.0);
                rates
                    .get(&currency_code)
                    .cloned()
                    .map(|base_to_requested_code| base_to_requested_code / base_to_usd)
                    .ok_or(client_error!(422, "Currency not found"))
            }
            None => Err(client_error!(422, "Currency not found")),
        }
    }

    pub async fn available_currency_codes(&self) -> ApiResult<Vec<String>> {
        let exchange = self.fetch_exchange().await?;
        Ok(exchange
            .rates
            .map_or(vec![], |s| s.keys().cloned().collect::<Vec<_>>()))
    }

    async fn fetch_exchange(&self) -> ApiResult<Exchange> {
        let url = base_exchange_api_url();
        let body = RequestCached::new(url)
            .cache_duration(exchange_api_cache_duration())
            .error_cache_duration(short_error_duration())
            .execute(self.client, self.cache)
            .await?;
        Ok(serde_json::from_str::<Exchange>(&body)?)
    }
}
