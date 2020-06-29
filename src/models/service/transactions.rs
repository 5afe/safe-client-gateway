use ethereum_types::Address;
use serde::Serialize;
use chrono::{DateTime, Utc};

#[derive(Serialize, Debug)]
pub enum Transaction {
    Transfer { to: Address },
    SettingsChange { date: DateTime<Utc> },
    Custom { to: Address },
    Unknown
}
