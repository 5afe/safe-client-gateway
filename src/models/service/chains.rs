use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ChainInfo {
    pub transaction_service: String,
    // do we need to expose this?
    pub chain_id: String,
    pub chain_name: String,
    pub rpc_url: String,
    pub block_explorer_url: String,
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
    pub logo_url: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Theme {
    pub text_color: String,
    pub background_color: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "lowercase")]
pub enum GasPrice {
    #[serde(rename_all = "camelCase")]
    Oracle {
        url: String,
        gas_parameter: Option<String>,
        gwei_factor: String,
    },
    #[serde(rename_all = "camelCase")]
    Fixed { wei_value: String },
}
