use ethereum_types::Address;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[typetag::serde(tag = "transaction_type")]
pub trait ServiceTransaction {}

#[derive(Deserialize, Serialize, Debug)]
pub struct Transfer {
    pub to: Address,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SettingsChange {
    pub date: DateTime<Utc>,
}

#[typetag::serde(name = "TRANSFER")]
impl ServiceTransaction for Transfer {}

#[typetag::serde(name = "SETTINGS_CHANGE")]
impl ServiceTransaction for SettingsChange {}