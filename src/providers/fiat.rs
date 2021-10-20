use crate::cache::cache_operations::RequestCached;
use crate::cache::redis::ServiceCache;
use crate::cache::Cache;
use crate::config::{base_exchange_api_uri, exchange_api_cache_duration, short_error_duration};
use crate::utils::context::{Context, RequestContext};
use crate::utils::errors::ApiResult;
use crate::utils::http_client::HttpClient;
use bigdecimal::BigDecimal;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Deserialize, Clone, Debug)]
pub struct Exchange {
    pub rates: Option<HashMap<String, BigDecimal>>,
    pub base: String,
}

pub struct FiatInfoProvider {
    client: Arc<dyn HttpClient>,
    cache: Arc<dyn Cache>,
}

impl FiatInfoProvider {
    pub fn new(context: &RequestContext) -> Self {
        FiatInfoProvider {
            client: context.http_client(),
            cache: context.cache(),
        }
    }

    pub async fn exchange_usd_to(&self, currency_code: &str) -> ApiResult<BigDecimal> {
        if &currency_code.to_lowercase() == "usd" {
            return Ok(BigDecimal::from(1));
        }

        let currency_code = currency_code.to_uppercase();
        let exchange = self.fetch_exchange().await?;
        match exchange.rates {
            Some(rates) => {
                let base_to_usd = rates.get("USD").unwrap_or(&BigDecimal::from(0)).to_owned();
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
        let url = base_exchange_api_uri();
        let body = RequestCached::new(url, &self.client, &self.cache)
            .cache_duration(exchange_api_cache_duration())
            .error_cache_duration(short_error_duration())
            .execute()
            .await?;
        Ok(serde_json::from_str::<Exchange>(&body)?)
    }
}
