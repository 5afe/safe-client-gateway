use crate::common::models::addresses::AddressEx;
use serde::Serialize;
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
#[serde(rename_all = "camelCase")]
pub(super) struct Message {
    pub(super) message_hash: String,
    pub(super) status: MessageStatus,
    pub(super) logo_uri: Option<String>,
    pub(super) name: Option<String>,
    pub(super) message: MessageValue,
    pub(super) creation_timestamp: i64,
    pub(super) modified_timestamp: i64,
    pub(super) confirmations_submitted: usize,
    pub(super) confirmations_required: usize,
    pub(super) proposed_by: AddressEx,
    pub(super) confirmations: Vec<Confirmation>,
    pub(super) prepared_signature: Option<String>,
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum MessageValue {
    String(String),
    Object(BTreeMap<String, serde_json::Value>),
}
