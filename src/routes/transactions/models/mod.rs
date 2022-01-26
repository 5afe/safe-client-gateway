use crate::common::models::addresses::AddressEx;
use crate::common::models::data_decoded::DataDecoded;
use serde::Serialize;

pub mod details;
pub mod requests;
pub mod summary;

pub const ID_SEPARATOR: &str = "_";
pub const ID_PREFIX_MULTISIG_TX: &str = "multisig";
pub const ID_PREFIX_MODULE_TX: &str = "module";
pub const ID_PREFIX_ETHEREUM_TX: &str = "ethereum";
pub const ID_PREFIX_CREATION_TX: &str = "creation";

#[derive(PartialEq, Debug)]
pub(crate) enum TransactionIdParts {
    Creation(String),
    Multisig {
        safe_address: String,
        safe_tx_hash: String,
    },
    Module {
        safe_address: String,
        transaction_hash: String,
        details_hash: String,
    },
    Ethereum {
        safe_address: String,
        transaction_hash: String,
        details_hash: String,
    },
    TransactionHash(String),
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum TransactionStatus {
    AwaitingConfirmations,
    AwaitingExecution,
    Cancelled,
    Failed,
    Success,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum TransactionInfo {
    Transfer(Transfer),
    SettingsChange(SettingsChange),
    Custom(Custom),
    Creation(Creation),
    Unknown,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Transfer {
    pub sender: AddressEx,
    pub recipient: AddressEx,
    pub direction: TransferDirection,
    pub transfer_info: TransferInfo,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum TransferDirection {
    Incoming,
    Outgoing,
    Unknown,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum TransferInfo {
    Erc20(Erc20Transfer),
    Erc721(Erc721Transfer),
    NativeCoin(NativeCoinTransfer),
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Erc20Transfer {
    // No need to map to AddressEx as the information are present in this struct
    pub token_address: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub logo_uri: Option<String>,
    pub decimals: Option<u64>,
    pub value: String,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Erc721Transfer {
    // No need to map to AddressEx as the information are present in this struct
    pub token_address: String,
    pub token_id: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub logo_uri: Option<String>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct NativeCoinTransfer {
    pub value: String,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct SettingsChange {
    pub data_decoded: DataDecoded,
    pub settings_info: Option<SettingsInfo>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum SettingsInfo {
    #[serde(rename_all = "camelCase")]
    SetFallbackHandler { handler: AddressEx },
    #[serde(rename_all = "camelCase")]
    AddOwner { owner: AddressEx, threshold: u64 },
    #[serde(rename_all = "camelCase")]
    RemoveOwner { owner: AddressEx, threshold: u64 },
    #[serde(rename_all = "camelCase")]
    SwapOwner {
        old_owner: AddressEx,
        new_owner: AddressEx,
    },
    #[serde(rename_all = "camelCase")]
    ChangeThreshold { threshold: u64 },
    #[serde(rename_all = "camelCase")]
    ChangeImplementation { implementation: AddressEx },
    #[serde(rename_all = "camelCase")]
    EnableModule { module: AddressEx },
    #[serde(rename_all = "camelCase")]
    DisableModule { module: AddressEx },
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Custom {
    pub to: AddressEx,
    pub data_size: String,
    pub value: String,
    pub method_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_count: Option<usize>,
    pub is_cancellation: bool,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct Creation {
    pub creator: AddressEx,
    pub transaction_hash: String,
    pub implementation: Option<AddressEx>,
    pub factory: Option<AddressEx>,
}
