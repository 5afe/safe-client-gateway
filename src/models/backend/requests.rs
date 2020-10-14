use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmationRequest {
    pub safe: String,
    pub to: String,
    pub value: String,
    pub data: String,
    pub operation: String,
    pub gas_token: String,
    pub safe_tx_gas: String,
    pub base_gas: String,
    pub gas_price: String,
    pub refund_receiver: String,
    pub nonce: String,
    pub contract_transaction_hash: String, // safe_tx_hash
    pub sender: String,
    pub signature: String,
    pub origin: String,
}