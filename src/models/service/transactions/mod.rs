use crate::models::commons::DataDecoded;
use crate::providers::address_info::AddressInfo;
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
pub enum TransactionStatus {
    AwaitingConfirmations,
    AwaitingExecution,
    Cancelled,
    Failed,
    Success,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "type")]
pub enum TransactionInfo {
    Transfer(Transfer),
    SettingsChange(SettingsChange),
    Custom(Custom),
    Creation(Creation),
    Unknown,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Transfer {
    pub sender: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_info: Option<AddressInfo>,
    pub recipient: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recipient_info: Option<AddressInfo>,
    pub direction: TransferDirection,
    pub transfer_info: TransferInfo,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferDirection {
    Incoming,
    Outgoing,
    Unknown,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TransferInfo {
    Erc20(Erc20Transfer),
    Erc721(Erc721Transfer),
    Ether(EtherTransfer),
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Erc20Transfer {
    pub token_address: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub logo_uri: Option<String>,
    pub decimals: Option<u64>,
    pub value: String,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Erc721Transfer {
    pub token_address: String,
    pub token_id: String,
    pub token_name: Option<String>,
    pub token_symbol: Option<String>,
    pub logo_uri: Option<String>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct EtherTransfer {
    pub value: String,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SettingsChange {
    pub data_decoded: DataDecoded,
    pub settings_info: Option<SettingsInfo>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(tag = "type")]
pub enum SettingsInfo {
    SetFallbackHandler {
        handler: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        handler_info: Option<AddressInfo>,
    },
    AddOwner {
        owner: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        owner_info: Option<AddressInfo>,
        threshold: u64,
    },
    RemoveOwner {
        owner: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        owner_info: Option<AddressInfo>,
        threshold: u64,
    },
    #[serde(rename_all = "camelCase")]
    SwapOwner {
        old_owner: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        old_owner_info: Option<AddressInfo>,
        new_owner: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        new_owner_info: Option<AddressInfo>,
    },
    ChangeThreshold {
        threshold: u64,
    },
    ChangeImplementation {
        implementation: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        implementation_info: Option<AddressInfo>,
    },
    EnableModule {
        module: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        module_info: Option<AddressInfo>,
    },
    DisableModule {
        module: String,
        #[serde(skip_serializing_if = "Option::is_none")]
        module_info: Option<AddressInfo>,
    },
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Custom {
    pub to: String,
    pub data_size: String,
    pub value: String,
    pub method_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action_count: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_info: Option<AddressInfo>,
}

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Creation {
    pub creator: String,
    pub transaction_hash: String,
    pub implementation: Option<String>,
    pub factory: Option<String>,
}
