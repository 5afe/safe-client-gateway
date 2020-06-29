use serde_repr::{Deserialize_repr, Serialize_repr};
use serde::{Deserialize, Serialize};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(u8)]
pub enum Operation {
    CALL = 0,
    DELEGATE = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ServiceTransactionType {
    SettingsChange,
    Transfer,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub count: usize,
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}