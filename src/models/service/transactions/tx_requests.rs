use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TxConfirmationRequest {
    pub safe_tx_hash: String,
}
