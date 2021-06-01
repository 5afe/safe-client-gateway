use super::*;
use crate::models::commons::{DataDecoded, Operation};
use crate::providers::info::{SafeAppInfo, TokenInfo};
use serde::Serialize;
use std::collections::HashMap;

/// Sample JSON
///
/// <details>
/// <summary>Sample 1: Multisig Transaction awaiting confirmation</summary>
///
/// ```json
/// {
///   "executedAt": null,
///   "txStatus": "AWAITING_CONFIRMATIONS",
///   "txInfo": {
///     "type": "SettingsChange",
///     "dataDecoded": {
///       "method": "changeThreshold",
///       "parameters": [
///         {
///           "name": "_threshold",
///           "type": "uint256",
///           "value": "2"
///         }
///       ]
///     }
///   },
///   "txData": {
///     "hexData": "0x694e80c30000000000000000000000000000000000000000000000000000000000000002",
///     "dataDecoded": {
///       "method": "changeThreshold",
///       "parameters": [
///         {
///           "name": "_threshold",
///           "type": "uint256",
///           "value": "2"
///         }
///       ]
///     },
///     "to": "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
///     "value": "0",
///     "operation": 0
///   },
///   "detailedExecutionInfo": {
///     "type": "MULTISIG",
///     "submittedAt": 1596792600322,
///     "nonce": 180,
///     "safeTxHash": "0x0ef685fb7984d7314c1368497e1b0c73016066bec41f966d32f18354b88fbd46",
///     "signers": [
///       "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23",
///       "0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72",
///       "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D",
///       "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd",
///       "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"
///     ],
///     "confirmationsRequired": 3,
///     "confirmations": [
///       {
///         "signer": "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23",
///         "signature": "0x1b01f3d79a50576e82d1da31810c0313bed9b76b016e1d9c6216512b2c7e53bb70df8163e568ca8ec1b8c7e7ef0a8db52d6ab2b7f47dc51c31729dd064ce375b1c"
///       }
///     ]
///   },
///   "txHash": null
/// }
/// ```
/// </details>
///
///
/// <details>
/// <summary>Sample 2: Ethereum transaction</summary>
///
/// ```json
/// {
///   "executedAt": 1596719563000,
///   "txStatus": "SUCCESS",
///   "txInfo": {
///     "type": "Transfer",
///     "sender": "0x938bae50a210b80EA233112800Cd5Bc2e7644300",
///     "recipient": "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
///     "direction": "INCOMING",
///     "transferInfo": {
///       "type": "ETHER",
///       "value": "50000000000000"
///     }
///   },
///   "txData": null,
///   "detailedExecutionInfo": null,
///   "txHash": "0x70b3c7f81a49f270fe86673f6c08beecfee384a89ef8b0869e46584905d4ecc2"
/// }
/// ```
/// </details>
///
///
/// <details>
/// <summary>Sample 3: Settings change</summary>
///
/// ```json
///     {
///       "id": "multisig_0x57d94fe21bbee8f6646c420ee23126cd1ba1b9a53a6c9b10099a043da8f32eea",
///       "timestamp": 1595429831000,
///       "txStatus": "SUCCESS",
///       "txInfo": {
///         "type": "SettingsChange",
///         "dataDecoded": {
///           "method": "addOwnerWithThreshold",
///           "parameters": [
///             {
///               "name": "owner",
///               "type": "address",
///               "value": "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D"
///             },
///             {
///               "name": "_threshold",
///               "type": "uint256",
///               "value": "2"
///             }
///           ]
///         }
///       },
///       "executionInfo": {
///         "nonce": 135,
///         "confirmationsRequired": 2,
///         "confirmationsSubmitted": 2
///       }
///     }
/// ```
/// </details>
#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionDetails {
    pub executed_at: Option<i64>,
    pub tx_status: TransactionStatus,
    pub tx_info: TransactionInfo,
    pub tx_data: Option<TransactionData>,
    pub detailed_execution_info: Option<DetailedExecutionInfo>,
    pub tx_hash: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_app_info: Option<SafeAppInfo>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum DetailedExecutionInfo {
    Multisig(MultisigExecutionDetails),
    Module(ModuleExecutionDetails),
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MultisigExecutionDetails {
    pub submitted_at: i64,
    pub nonce: u64,
    pub safe_tx_gas: usize,
    pub base_gas: usize,
    pub gas_price: String,
    pub gas_token: String,
    pub refund_receiver: String,
    pub safe_tx_hash: String,
    pub executor: Option<String>,
    pub signers: Vec<String>,
    pub confirmations_required: u64,
    pub confirmations: Vec<MultisigConfirmation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejectors: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_token_info: Option<TokenInfo>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MultisigConfirmation {
    pub signer: String,
    pub signature: Option<String>,
    pub submitted_at: i64,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModuleExecutionDetails {
    pub address: String,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    pub hex_data: Option<String>,
    pub data_decoded: Option<DataDecoded>,
    pub to: String,
    pub value: Option<String>,
    pub operation: Operation,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_info_index: Option<HashMap<String, AddressInfo>>,
}
