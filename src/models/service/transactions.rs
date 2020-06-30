use ethereum_types::Address;
use serde::Serialize;
use chrono::{DateTime, Utc};
use crate::models::commons::TransferType;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Transaction {
    Transfer {
        to: Address,
        transafer_type: TransferType,
    },
    SettingsChange { date: DateTime<Utc> },
    Custom { to: Address },
    Unknown,
}
