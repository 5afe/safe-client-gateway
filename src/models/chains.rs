use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChainInfo {
    pub chain_id: String,
    pub chain_name: String,
    rpc_url: String,
    block_explorer_url: String,
    native_currency: NativeCurrency,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    name: String,
    symbol: String,
    decimals: u64,
}
