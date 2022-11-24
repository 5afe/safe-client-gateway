use crate::common::models::addresses::AddressEx;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Serialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub(super) enum MessageStatus {
    NeedsConfirmation,
    Confirmed,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct Confirmation {
    pub(super) owner: AddressEx,
    pub(super) signature: String,
}

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub(super) enum Message {
    #[serde(rename_all = "camelCase")]
    Message {
        message_hash: String,
        status: MessageStatus,
        logo_uri: Option<String>,
        name: Option<String>,
        message: MessageValue,
        creation_timestamp: i64,
        modified_timestamp: i64,
        confirmations_submitted: usize,
        confirmations_required: usize,
        proposed_by: AddressEx,
        confirmations: Vec<Confirmation>,
        prepared_signature: Option<String>,
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CreateMessage {
    message: String,
    safe_app_id: u64,
    signature: String,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMessage {
    signature: String,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum MessageValue {
    String(String),
    Object(BTreeMap<String, serde_json::Value>),
}
