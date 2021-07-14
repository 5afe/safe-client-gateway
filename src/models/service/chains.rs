use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChainInfo {
    pub transaction_service: String, // do we need to expose this?
    pub chain_id: String,
    pub chain_name: String,
    pub rpc_url: String,
    pub block_explorer_url: String,
    pub native_currency: NativeCurrency,
    pub theme: Theme,
    pub ens_registry_address: Option<String>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub logo_url: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub text_color: String,
    pub background_color: String,
}
