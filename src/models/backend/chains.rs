use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChainInfo {
    pub recommended_master_copy_version: String,
    pub transaction_service: String,
    pub chain_id: String,
    pub chain_name: String,
    pub rpc_url: String,
    pub block_explorer_url: String,
    pub native_currency: NativeCurrency,
    pub theme: Theme,
    pub ens_registry_address: Option<String>,
    pub gas_price: GasPrice,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct NativeCurrency {
    pub name: String,
    pub symbol: String,
    pub decimals: u64,
    pub logo_url: String,
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
        url: String,
        gas_parameter: String,
        gwei_factor: String,
    },
    #[serde(rename_all = "camelCase")]
    Fixed { wei_value: String },
    #[serde(other)]
    Unknown,
}
