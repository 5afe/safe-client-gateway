use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TxConfirmationRequest {
    pub safe_tx_hash: String,
}
