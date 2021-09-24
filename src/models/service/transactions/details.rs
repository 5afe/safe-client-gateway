use super::*;
use crate::models::commons::{DataDecoded, Operation};
use crate::providers::info::{SafeAppInfo, TokenInfo};
use serde::Serialize;
use std::collections::HashMap;

/// Top level object returned by the `/v1/transactions/<details_id>` endpoint
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
///     "to": { "value" : "0x1230B3d59858296A31053C1b8562Ecf89A2f888b" },
///     "value": "0",
///     "operation": 0
///   },
///   "detailedExecutionInfo": {
///     "type": "MULTISIG",
///     "submittedAt": 1596792600322,
///     "nonce": 180,
///     "safeTxHash": "0x0ef685fb7984d7314c1368497e1b0c73016066bec41f966d32f18354b88fbd46",
///     "signers": [
///       { "value" : "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23" },
///       { "value" : "0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72" },
///       { "value" : "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D" },
///       { "value" : "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd" },
///       { "value" : "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0" }
///     ],
///     "confirmationsRequired": 3,
///     "confirmations": [
///       {
///         "signer": { "value" : "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23" } ,
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
///   "executedAt": 1604531696000,
///   "txStatus": "SUCCESS",
///   "txInfo": {
///     "type": "Transfer",
///     "sender": {
///      "value": "0x05c85Ab5B09Eb8A55020d72daf6091E04e264af9"
///     },
///     "recipient": {
///       "value": "0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f"
///     },
///     "direction": "INCOMING",
///     "transferInfo": {
///       "type": "NATIVE_COIN",
///       "value": "100000000000000000"
///     }
///   },
///   "txData": null,
///   "detailedExecutionInfo": null,
///   "txHash": "0x7e95b9df8b1c1385665d0bccfbd5d6f913e18915750395d84dd490c7d9be9940"
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
///   "executedAt": 1595429831000,
///   "txStatus": "SUCCESS",
///   "txInfo": {
///     "type": "SettingsChange",
///     "dataDecoded": {
///       "method": "addOwnerWithThreshold",
///       "parameters": [
///         {
///           "name": "owner",
///           "type": "address",
///           "value": "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D"
///         },
///         {
///           "name": "_threshold",
///           "type": "uint256",
///           "value": "2"
///         }
///       ]
///     },
///     "settingsInfo": {
///       "type": "ADD_OWNER",
///       "owner": {
///         "value": "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D"
///       },
///       "threshold": 2
///     }
///   },
///   "txData": {
///     "hexData": "0x0d582f13000000000000000000000000a3daa0d9ae02daa17a664c232ada1b739ef5ae8d0000000000000000000000000000000000000000000000000000000000000002",
///     "dataDecoded": {
///       "method": "addOwnerWithThreshold",
///       "parameters": [
///         {
///           "name": "owner",
///           "type": "address",
///           "value": "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D"
///         },
///         {
///           "name": "_threshold",
///           "type": "uint256",
///           "value": "2"
///         }
///       ]
///     },
///     "to": {
///       "value": "0x1230B3d59858296A31053C1b8562Ecf89A2f888b"
///     },
///     "value": "0",
///     "operation": 0
///   },
///   "detailedExecutionInfo": {
///     "type": "MULTISIG",
///     "submittedAt": 1595429831000,
///     "nonce": 135,
///     "safeTxGas": 59786,
///     "baseGas": 0,
///     "gasPrice": "0",
///     "gasToken": "0x0000000000000000000000000000000000000000",
///     "refundReceiver": {
///       "value": "0x0000000000000000000000000000000000000000"
///     },
///     "safeTxHash": "0x57d94fe21bbee8f6646c420ee23126cd1ba1b9a53a6c9b10099a043da8f32eea",
///     "executor": {
///       "value": "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"
///     },
///     "signers": [
///       {
///         "value": "0x6169b3e26fac6870208e12074d91DF7eD124339e"
///       },
///       {
///         "value": "0x12536c4952c71b32c9f97FdC2BAC8CFB156A9dAf"
///       },
///       {
///         "value": "0x365ffD124183EcD9c301F5aB2E754b6f74F15876"
///       },
///       {
///         "value": "0xd0ba955b8F34561907Abb588603a2400e06BD2d2"
///       },
///       {
///         "value": "0xec7b7F5C0031e6C933931Ade1833aac867c5CD5f"
///       },
///       {
///         "value": "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"
///       },
///       {
///         "value": "0xfDDB1e19a973d7EDf1211970AF3E42d40acfd20F"
///       },
///       {
///         "value": "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164"
///       },
///       {
///         "value": "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"
///       }
///     ],
///     "confirmationsRequired": 2,
///     "confirmations": [
///       {
///         "signer": {
///           "value": "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"
///         },
///         "signature": "0x00000000000000000000000065f8236309e5a99ff0d129d04e486ebce20dc7b0000000000000000000000000000000000000000000000000000000000000000001",
///         "submittedAt": 1595429831000
///       },
///       {
///         "signer": {
///           "value": "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"
///         },
///         "signature": "0x2e8318bcd59462b715384e39fd848ccd0f54610cacdbb6adb46ec5c374d1bd2f1f8b3e1873f292bcafffd62c59281a00e2890478f1dfe72866d6edf3fd3522711b",
///         "submittedAt": 1595429831000
///       }
///     ]
///   },
///   "txHash": "0x7a6373c2d18e6e9dda16f4bd7f16a24600314487f3e30fb63a227b368d18121a"
/// }
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
    pub safe_tx_gas: String,
    pub base_gas: String,
    pub gas_price: String,
    // As this is a token we will keep the token information separated
    pub gas_token: String,
    pub refund_receiver: AddressEx,
    pub safe_tx_hash: String,
    pub executor: Option<AddressEx>,
    pub signers: Vec<AddressEx>,
    pub confirmations_required: u64,
    pub confirmations: Vec<MultisigConfirmation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rejectors: Option<Vec<AddressEx>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas_token_info: Option<TokenInfo>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MultisigConfirmation {
    pub signer: AddressEx,
    pub signature: Option<String>,
    pub submitted_at: i64,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModuleExecutionDetails {
    pub address: AddressEx,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionData {
    pub hex_data: Option<String>,
    pub data_decoded: Option<DataDecoded>,
    pub to: AddressEx,
    pub value: Option<String>,
    pub operation: Operation,
    #[serde(skip_serializing_if = "Option::is_none")]
    // Mapping with info for the addresses in data_decoded
    pub address_info_index: Option<HashMap<String, AddressEx>>,
}
