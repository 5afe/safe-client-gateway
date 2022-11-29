use super::frontend_models::{
    Confirmation as FrontendConfirmation, Message as FrontendMessage,
    MessageValue as FrontendMessageValue,
};
use crate::common::models::addresses::AddressEx;
use crate::providers::ext::InfoProviderExt;
use crate::providers::info::{InfoProvider, SafeInfo};
use crate::routes::messages::backend_models::{Confirmation, Message, MessageValue};
use crate::routes::messages::frontend_models::MessageStatus;
use rocket::futures::future;

pub(super) async fn map_message(
    info_provider: &(impl InfoProvider + Sync),
    safe_info: &SafeInfo,
    message: &Message,
) -> FrontendMessage {
    let confirmations_required = safe_info.threshold as usize;
    let confirmations_submitted = message.confirmations.len();

    // Get Safe App Info for specific ID.
    // If the Safe App Info cannot be retrieved we return null
    let safe_app_name_logo: (Option<String>, Option<String>) = match message.safe_app_id {
        None => (None, None),
        Some(safe_app_id) => info_provider
            .safe_app_info_by_id(safe_app_id)
            .await
            .map_or((None, None), |safe_app| {
                (Some(safe_app.name), Some(safe_app.logo_uri))
            }),
    };

    // Known Address for proposed_by
    let proposed_by: AddressEx = info_provider
        .address_ex_from_contracts_or_default(&message.proposed_by)
        .await;

    // Known address for each confirmation
    let confirmations: Vec<FrontendConfirmation> = future::join_all(
        message
            .confirmations
            .iter()
            .map(|confirmation| map_confirmation(info_provider, &confirmation)),
    )
    .await;

    return FrontendMessage {
        message_hash: message.message_hash.to_string(),
        status: if confirmations_submitted >= confirmations_required {
            MessageStatus::Confirmed
        } else {
            MessageStatus::NeedsConfirmation
        },
        name: safe_app_name_logo.0,
        logo_uri: safe_app_name_logo.1,
        message: match &message.message {
            MessageValue::String(value) => FrontendMessageValue::String(value.to_string()),
            MessageValue::Object(value) => FrontendMessageValue::Object(value.clone()),
        },
        creation_timestamp: message.created.timestamp_millis(),
        modified_timestamp: message.modified.timestamp_millis(),
        confirmations_submitted,
        confirmations_required,
        proposed_by,
        confirmations,
        prepared_signature: match &message.prepared_signature {
            None => None,
            Some(value) => {
                if confirmations_submitted >= confirmations_required {
                    Some(value.to_string())
                } else {
                    None
                }
            }
        },
    };
}

async fn map_confirmation(
    info_provider: &(impl InfoProvider + Sync),
    confirmation: &Confirmation,
) -> FrontendConfirmation {
    let owner: AddressEx = info_provider
        .address_ex_from_contracts_or_default(&confirmation.owner)
        .await;

    return FrontendConfirmation {
        owner,
        signature: confirmation.signature.to_string(),
    };
}
