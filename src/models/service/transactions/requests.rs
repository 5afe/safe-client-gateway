use crate::models::commons::Operation;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmationRequest {
    pub signed_safe_tx_hash: String,
}

/// MultisigTransactionRequest
///
/// <details>
/// <summary>Example body of MultisigTransactionRequest for submitting a Cancellation Transaction</summary>
///
/// ```json
/// {
///   "to": "0xBe8C10Dbf4c6148f9834C56C3331f8191f355552",
///   "value": "0",
///   "data": "0x",
///   "nonce": "39",
///   "operation": 0,
///   "safeTxGas": "0",
///   "baseGas": "0",
///   "gasPrice": "0",
///   "gasToken": "0x0000000000000000000000000000000000000000",
///   "refundReceiver": "0x0000000000000000000000000000000000000000",
///   "safeTxHash": "61b0acaeae49e74306536c3371cd80bb46aeb5732859b6ffd776ad24e2d57f8e",
///   "sender": "0xBe8C10Dbf4c6148f9834C56C3331f8191f355552",
///   "signature": "a519dd9aa226a5f6f1816035af85d43d834c3284912574bef0c04aaff1f21004602a5339da424cf53de9aac068f67a3417f3cba40e3049637ae901b6f345b4ac1b"
/// }
/// ```
/// </details>
#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
// Addresses are not mapped to AddressEx as this is a request body that is forwarded to the core services
pub struct MultisigTransactionRequest {
    pub to: String,
    pub value: String,
    pub data: Option<String>,
    pub nonce: String,
    pub operation: Operation,
    pub safe_tx_gas: String,
    pub base_gas: String,
    pub gas_price: String,
    pub gas_token: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_receiver: Option<String>,
    #[serde(rename(serialize = "contractTransactionHash"))]
    pub safe_tx_hash: String,
    pub sender: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub signature: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin: Option<String>,
}
