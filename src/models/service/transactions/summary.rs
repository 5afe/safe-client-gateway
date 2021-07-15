use super::*;
use crate::providers::info::SafeAppInfo;
use serde::Serialize;

///TransactionSummary - object returned for [TransactionListItem::Transaction]
///
///<details>
/// <summary>Sample 1: History of executed transactions with date labels per day</summary>
///
/// ```json
/// {
///   "next": null,
///   "previous": null,
///   "results": [
///     {
///       "type": "DATE_LABEL",
///       "timestamp": 1604620800000
///     },
///     {
///       "type": "TRANSACTION",
///       "transaction": {
///         "id": "multisig_0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f_0x28b4cc29c036c2df40a1ba8d684cdab736abaf7d5cb84b217428462a2b4e3318",
///         "timestamp": 1604700419000,
///         "txStatus": "SUCCESS",
///         "txInfo": {
///           "type": "Custom",
///           "to": "0x8D29bE29923b68abfDD21e541b9374737B49cdAD",
///           "dataSize": "580",
///           "value": "0",
///           "methodName": "multiSend"
///         },
///         "executionInfo": {
///           "nonce": 2,
///           "confirmationsRequired": 1,
///           "confirmationsSubmitted": 1
///         }
///       },
///       "conflictType": "None"
///     },
///     {
///       "type": "TRANSACTION",
///       "transaction": {
///         "id": "multisig_0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f_0x2729fd437ad8f523ea0b8ca7f46401de38fc96cd62f6a0b07ac5637c4c195f3b",
///         "timestamp": 1604684216000,
///         "txStatus": "SUCCESS",
///         "txInfo": {
///           "type": "Custom",
///           "to": "0x8D29bE29923b68abfDD21e541b9374737B49cdAD",
///           "dataSize": "580",
///           "value": "0",
///           "methodName": "multiSend"
///         },
///         "executionInfo": {
///           "nonce": 1,
///           "confirmationsRequired": 1,
///           "confirmationsSubmitted": 1
///         }
///       },
///       "conflictType": "None"
///     },
///     {
///       "type": "DATE_LABEL",
///       "timestamp": 1604448000000
///     },
///     {
///       "type": "TRANSACTION",
///       "transaction": {
///         "id": "multisig_0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f_0xb487741c687a81496034b08d7e6d94986cbae38dc6ebd2aa8e547cb8f8542cc0",
///         "timestamp": 1604533603000,
///         "txStatus": "SUCCESS",
///         "txInfo": {
///           "type": "Custom",
///           "to": "0x8D29bE29923b68abfDD21e541b9374737B49cdAD",
///           "dataSize": "260",
///           "value": "0",
///           "methodName": "multiSend"
///         },
///         "executionInfo": {
///           "nonce": 0,
///           "confirmationsRequired": 1,
///           "confirmationsSubmitted": 1
///         }
///       },
///       "conflictType": "None"
///     },
///     {
///       "type": "TRANSACTION",
///       "transaction": {
///         "id": "ethereum_0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f_0x7e95b9df8b1c1385665d0bccfbd5d6f913e18915750395d84dd490c7d9be9940_0xbf9e8a462afc9675",
///         "timestamp": 1604531696000,
///         "txStatus": "SUCCESS",
///         "txInfo": {
///           "type": "Transfer",
///           "sender": "0x05c85Ab5B09Eb8A55020d72daf6091E04e264af9",
///           "recipient": "0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f",
///           "direction": "INCOMING",
///           "transferInfo": {
///             "type": "ETHER",
///             "value": "100000000000000000"
///           }
///         },
///         "executionInfo": null
///       },
///       "conflictType": "None"
///     },
///     {
///       "type": "TRANSACTION",
///       "transaction": {
///         "id": "creation_0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f",
///         "timestamp": 1604531396000,
///         "txStatus": "SUCCESS",
///         "txInfo": {
///           "type": "Creation",
///           "creator": "0x05c85Ab5B09Eb8A55020d72daf6091E04e264af9",
///           "transactionHash": "0xbfe5f021d0cfaf98ec445f757802be9e86b818301e2d892bcf3a9ee5e688d37f",
///           "implementation": "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F",
///           "factory": "0x76E2cFc1F5Fa8F6a5b3fC4c8F4788F0116861F9B"
///         },
///         "executionInfo": null
///       },
///       "conflictType": "None"
///     }
///   ]
/// }
/// ```
///
/// </details>
#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct TransactionSummary {
    pub id: String,
    pub timestamp: i64,
    pub tx_status: TransactionStatus,
    pub tx_info: TransactionInfo,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_info: Option<ExecutionInfo>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safe_app_info: Option<SafeAppInfo>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ExecutionInfo {
    Multisig(MultisigExecutionInfo),
    Module(ModuleExecutionInfo),
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct MultisigExecutionInfo {
    pub nonce: u64,
    pub confirmations_required: u64,
    pub confirmations_submitted: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub missing_signers: Option<Vec<AddressEx>>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ModuleExecutionInfo {
    pub address: AddressEx,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransactionListItem {
    #[serde(rename_all = "camelCase")]
    Transaction {
        transaction: TransactionSummary,
        conflict_type: ConflictType,
    },
    DateLabel {
        timestamp: i64,
    },
    Label {
        label: Label,
    },
    ConflictHeader {
        nonce: u64,
    },
}

#[derive(Serialize, Debug, PartialEq)]
pub enum Label {
    Next,
    Queued,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum ConflictType {
    None,
    HasNext,
    End,
}
