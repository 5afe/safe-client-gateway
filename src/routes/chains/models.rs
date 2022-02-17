use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct ChainInfo {
    pub transaction_service: String,
    // do we need to expose this?
    pub chain_id: String,
    pub chain_name: String,
    pub short_name: String,
    pub l2: bool,
    pub description: String,
    pub rpc_uri: RpcUri,
    pub safe_apps_rpc_uri: RpcUri,
    pub public_rpc_uri: RpcUri,
    pub block_explorer_uri_template: BlockExplorerUriTemplate,
    pub native_currency: NativeCurrency,
    pub theme: Theme,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ens_registry_address: Option<String>,
    pub gas_price: Vec<GasPrice>,
    pub disabled_wallets: Vec<String>,
    pub features: Vec<String>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub logo_uri: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Theme {
    pub text_color: String,
    pub background_color: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum GasPrice {
    #[serde(rename_all = "camelCase")]
    Oracle {
        uri: String,
        gas_parameter: String,
        gwei_factor: String,
    },
    #[serde(rename_all = "camelCase")]
    Fixed {
        wei_value: String,
    },
    Unknown,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct RpcUri {
    pub authentication: RpcAuthentication,
    pub value: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum RpcAuthentication {
    ApiKeyPath,
    NoAuthentication,
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct BlockExplorerUriTemplate {
    pub address: String,
    pub tx_hash: String,
    pub api: String,
}
