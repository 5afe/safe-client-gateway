use crate::models::commons::Operation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmationRequest {
    pub signed_safe_tx_hash: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MultisigTransactionRequest {
    pub to: String,
    pub value: String,
    pub data: String,
    pub nonce: String,
    pub operation: Operation,
    pub safe_tx_gas: String,
    pub base_gas: String,
    pub gas_price: String,
    pub gas_token: String,
    pub refund_receiver: String,
    pub contract_transaction_hash: String,
    pub sender: String,
    pub signature: String,
}
