use crate::common::models::addresses::AddressEx;
use serde::Serialize;

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
        logo_uri: String,
        name: String,
        message: String,
        creation_timestamp: i64,
        modified_timestamp: i64,
        confirmations_submitted: usize,
        confirmations_required: usize,
        proposed_by: AddressEx,
        confirmations: Vec<Confirmation>,
        prepared_signature: Option<String>,
    },
}
