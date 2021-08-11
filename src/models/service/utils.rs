use crate::models::commons::Operation;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SafeTransactionEstimationRequest {
    // Address will not be mapped to AddressEx as it is a POST body that is forwarded to the core services
    pub to: String,
    pub value: String,
    pub data: String,
    pub operation: Operation,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SafeTransactionEstimation {
    pub latest_nonce: u64,
    pub safe_tx_gas: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataDecoderRequest {
    pub data: String,
}
