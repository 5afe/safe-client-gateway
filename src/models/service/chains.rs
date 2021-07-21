use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChainInfo {
    pub transaction_service: String,
    // do we need to expose this?
    pub chain_id: String,
    pub chain_name: String,
    pub rpc_uri: RpcUri,
    pub block_explorer_uri: String,
    pub native_currency: NativeCurrency,
    pub theme: Theme,
    pub ens_registry_address: Option<String>,
    pub gas_price: GasPrice,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub logo_uri: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub text_color: String,
    pub background_color: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
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
pub struct RpcUri {
    pub authentication: RpcAuthentication,
    pub value: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum RpcAuthentication {
    ApiKeyPath,
    NoAuthentication,
    #[serde(other)]
    Unknown,
}
