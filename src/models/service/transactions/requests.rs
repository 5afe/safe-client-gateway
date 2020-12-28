use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmationRequest {
    pub signed_safe_tx_hash: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SendEthRequest {
    pub receiver: String,
    pub sender: String,
    pub value: String,
    pub data: String,
    pub transaction_hash: String,
    pub signed_transaction_hash: String,
    pub nonce: String,
}
