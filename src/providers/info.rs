use crate::cache::cache_operations::RequestCached;
use crate::cache::Cache;
use crate::common::models::addresses::AddressEx;
use crate::common::models::backend::chains::ChainInfo;
use crate::common::models::backend::safes::MasterCopy;
use crate::common::models::page::Page;
use crate::config::{
    address_info_cache_duration, chain_info_cache_duration, chain_info_request_timeout,
    contract_info_request_timeout, default_request_timeout, long_error_duration,
    request_cache_duration, safe_app_info_request_timeout, safe_app_manifest_cache_duration,
    safe_info_cache_duration, safe_info_request_timeout, short_error_duration,
    token_info_cache_duration, token_info_request_timeout,
};
use crate::providers::address_info::ContractInfo;
use crate::utils::context::RequestContext;
use crate::utils::errors::ApiResult;
use crate::utils::http_client::{HttpClient, Request};
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
use std::sync::Arc;
use std::time::Duration;

pub const TOKENS_KEY_BASE: &'static str = "dip_ti";
lazy_static! {
    pub static ref SAFE_V_1_3_0: Version = Version::new(1, 3, 0);
}

// TODO: move models that are (de)serialized into models module.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TokenType {
    Erc721,
    Erc20,
    NativeToken,
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub stru:ct SafeInfo {
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
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct SafeAppInfo {
    pub name: String,
    pub url: String,
    pub logo_uri: String,
}

#[derive(Deserialize, Debug, PartialEq)]
struct Manifest {
    pub(super) name: String,
    pub(super) description: String,
    #[serde(rename(deserialize = "iconPath"))]
    pub(super) icon_path: String,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TokenInfo {
    #[serde(rename = "type")]
    pub token_type: TokenType,
    // No need to map to AddressEx as the information are present in this struct
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
    async fn chain_info(&self) -> ApiResult<ChainInfo>;
    async fn safe_info(&self, safe: &str) -> ApiResult<SafeInfo>;
    async fn token_info(&self, token: &str) -> ApiResult<TokenInfo>;
    async fn safe_app_info(&self, url: &str) -> ApiResult<SafeAppInfo>;
    async fn address_ex_from_any_source(&self, address: &str) -> ApiResult<AddressEx>;
    async fn address_ex_from_contracts(&self, address: &str) -> ApiResult<AddressEx>;
    fn chain_id(&self) -> &str;

    fn client(&self) -> Arc<dyn HttpClient>;
    fn cache(&self) -> Arc<dyn Cache>;
}

pub struct DefaultInfoProvider<'p> {
    pub chain_id: &'p str,
    client: Arc<dyn HttpClient>,
    cache: Arc<dyn Cache>,
    // Mutex is an async Mutex, meaning that the lock is non-blocking
    safe_cache: Mutex<HashMap<String, Option<SafeInfo>>>,
    token_cache: Mutex<HashMap<String, Option<TokenInfo>>>,
    chain_cache: Mutex<HashMap<String, Option<ChainInfo>>>,
}

#[rocket::async_trait]
impl InfoProvider for DefaultInfoProvider<'_> {
    fn chain_id(&self) -> &str {
        self.chain_id
    }

    async fn chain_info(&self) -> ApiResult<ChainInfo> {
        let chain_cache = &mut self.chain_cache.lock().await;
        Self::cached(chain_cache, || self.load_chain_info(), self.chain_id).await
    }

    async fn safe_info(&self, safe: &str) -> ApiResult<SafeInfo> {
        let safe_cache = &mut self.safe_cache.lock().await;
        Self::cached(safe_cache, || self.load_safe_info(safe.to_string()), safe).await
    }

    async fn token_info(&self, token: &str) -> ApiResult<TokenInfo> {
        if token != "0x0000000000000000000000000000000000000000" {
            let token_cache = &mut self.token_cache.lock().await;
            Self::cached(
                token_cache,
                || self.load_token_info(token.to_string()),
                token,
            )
            .await
        } else {
            bail!("Token Address is 0x0")
        }
    }

    async fn safe_app_info(&self, url: &str) -> ApiResult<SafeAppInfo> {
        let manifest_url = build_manifest_url(url)?;

        let manifest_json = RequestCached::new(manifest_url, &self.client, &self.cache)
            .cache_duration(safe_app_manifest_cache_duration())
            .error_cache_duration(long_error_duration())
            .cache_all_errors()
            .request_timeout(safe_app_info_request_timeout())
            .execute()
            .await?;
        let manifest = serde_json::from_str::<Manifest>(&manifest_json)?;
        Ok(SafeAppInfo {
            name: manifest.name.to_owned(),
            url: url.to_owned(),
            logo_uri: format!("{}/{}", url, manifest.icon_path),
        })
    }

    async fn address_ex_from_contracts(&self, address: &str) -> ApiResult<AddressEx> {
        let url = core_uri!(self, "/v1/contracts/{}/", address)?;
        let contract_info_json = RequestCached::new(url, &self.client, &self.cache)
            .cache_duration(address_info_cache_duration())
            .error_cache_duration(long_error_duration())
            .request_timeout(contract_info_request_timeout())
            .execute()
            .await?;
        let contract_info = serde_json::from_str::<ContractInfo>(&contract_info_json)?;
        if contract_info.display_name.trim().is_empty() {
            bail!("No display name")
        } else {
            Ok(AddressEx {
                value: address.to_owned(),
                name: Some(contract_info.display_name.to_owned()),
                logo_uri: contract_info.logo_uri.to_owned(),
            })
        }
    }

    async fn address_ex_from_any_source(&self, address: &str) -> ApiResult<AddressEx> {
        self.token_info(&address)
            .map_ok(|it| AddressEx {
                value: address.to_owned(),
                name: Some(it.name),
                logo_uri: it.logo_uri,
            })
            .or_else(|_| async move { self.address_ex_from_contracts(&address).await })
            .await
    }

    fn client(&self) -> Arc<dyn HttpClient> {
        self.client.clone()
    }

    fn cache(&self) -> Arc<dyn Cache> {
        self.cache.clone()
    }
}

impl<'a> DefaultInfoProvider<'a> {
    pub fn new(chain_id: &'a str, context: &RequestContext) -> Self {
        DefaultInfoProvider {
            chain_id,
            client: context.http_client(),
            cache: context.cache(),
            safe_cache: Default::default(),
            token_cache: Default::default(),
            chain_cache: Default::default(),
        }
    }
}

impl DefaultInfoProvider<'_> {
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

    async fn load_safe_info(&self, safe: String) -> ApiResult<Option<SafeInfo>> {
        let url = core_uri!(self, "/v1/safes/{}/", safe)?;
        let data = RequestCached::new(url, &self.client, &self.cache)
            .cache_duration(safe_info_cache_duration())
            .error_cache_duration(short_error_duration())
            .request_timeout(safe_info_request_timeout())
            .execute()
            .await?;
        Ok(serde_json::from_str(&data).ok())
    }

    async fn populate_token_cache(&self) -> ApiResult<()> {
        let url = core_uri!(self, "/v1/tokens/?limit=10000")?;
        let request = {
            let mut request = Request::new(url);
            request.timeout(Duration::from_millis(token_info_request_timeout()));
            request
        };

        let response = self.client.get(request).await?;
        let data: Page<TokenInfo> = serde_json::from_str(&response.body)?;
        let token_key = generate_token_key(self.chain_id);
        for token in data.results.iter() {
            self.cache
                .insert_in_hash(&token_key, &token.address, &serde_json::to_string(&token)?);
        }
        Ok(())
    }

    async fn check_token_cache(&self) -> ApiResult<()> {
        let token_key = generate_token_key(&self.chain_id);
        if self.cache.has_key(&token_key) {
            return Ok(());
        }
        self.cache.insert_in_hash(&token_key, "state", "populating");
        let result = self.populate_token_cache().await;
        if result.is_ok() {
            self.cache
                .expire_entity(&token_key, token_info_cache_duration());
            self.cache.insert_in_hash(&token_key, "state", "populated");
        } else {
            self.cache.expire_entity(&token_key, short_error_duration());
            self.cache.insert_in_hash(&token_key, "state", "errored");
        }
        result
    }

    async fn load_token_info(&self, token: String) -> ApiResult<Option<TokenInfo>> {
        self.check_token_cache().await?;
        match self
            .cache
            .get_from_hash(&generate_token_key(&self.chain_id), &token)
        {
            Some(cached) => Ok(Some(serde_json::from_str::<TokenInfo>(&cached)?)),
            None => Ok(None),
        }
    }

    async fn load_chain_info(&self) -> ApiResult<Option<ChainInfo>> {
        let url = config_uri!("/v1/chains/{}/", self.chain_id);
        let data = RequestCached::new(url, &self.client, &self.cache)
            .cache_duration(chain_info_cache_duration())
            .error_cache_duration(short_error_duration())
            .request_timeout(chain_info_request_timeout())
            .execute()
            .await?;
        let result = serde_json::from_str::<ChainInfo>(&data).ok();
        Ok(result)
    }

    pub async fn master_copies(&self) -> ApiResult<Vec<MasterCopy>> {
        let url = core_uri!(self, "/v1/about/master-copies/")?;
        let body = RequestCached::new(url, &self.client, &self.cache)
            .cache_duration(request_cache_duration())
            .error_cache_duration(short_error_duration())
            .request_timeout(default_request_timeout())
            .execute()
            .await?;
        Ok(serde_json::from_str(&body)?)
    }
}

pub fn generate_token_key(chain_id: &str) -> String {
    format!("{}_{}", TOKENS_KEY_BASE, chain_id)
}
