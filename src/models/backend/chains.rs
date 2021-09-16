use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChainInfo {
    pub recommended_master_copy_version: String,
    pub transaction_service: String,
    pub chain_id: String,
    pub chain_name: String,
    pub l2: bool,
    pub description: String,
    pub rpc_uri: RpcUri,
    pub block_explorer_uri_template: BlockExplorerUriTemplate,
    pub native_currency: NativeCurrency,
    pub theme: Theme,
    pub ens_registry_address: Option<String>,
    pub gas_price: Vec<GasPrice>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub logo_uri: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub text_color: String,
    pub background_color: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum GasPrice {
    #[serde(rename_all = "camelCase")]
    Oracle {
        uri: String,
        gas_parameter: String,
        gwei_factor: String,
    },
    #[serde(rename_all = "camelCase")]
    Fixed { wei_value: String },
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RpcUri {
    pub authentication: RpcAuthentication,
    pub value: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RpcAuthentication {
    ApiKeyPath,
    NoAuthentication,
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockExplorerUriTemplate {
    pub address: String,
    pub tx_hash: String,
}
