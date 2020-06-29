use ethereum_types::Address;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum Transaction {
    Transfer {
        to: Address,
    },
    SettingsChange { date: DateTime<Utc> },
    Custom { to: Address },
    Unknown,
}
