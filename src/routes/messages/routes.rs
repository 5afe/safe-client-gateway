use super::backend_models::Message;
use super::frontend_models::{Confirmation as FrontendConfirmation, Message as FrontendMessage};
use crate::common::models::addresses::AddressEx;
use crate::common::models::page::{Page, PageMetadata};
use crate::providers::ext::InfoProviderExt;
use crate::providers::info::{DefaultInfoProvider, InfoProvider, SafeInfo};
use crate::routes::messages::backend_models::Confirmation;
use crate::routes::messages::frontend_models::{CreateMessage, MessageStatus};
use crate::utils::context::RequestContext;
use crate::utils::errors::{ApiError, ApiResult, ErrorDetails};
use crate::utils::http_client::{Request, Response};
use crate::utils::urls::build_absolute_uri;
use reqwest::Url;
use rocket::futures::future;
use rocket::response::content;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use std::borrow::Cow;

#[openapi(tag = "Messages")]
#[get("/v1/chains/<chain_id>/safes/<safe_address>/messages?<cursor>")]
pub async fn get_messages(
    context: RequestContext,
    chain_id: String,
    safe_address: String,
    cursor: Option<String>,
) -> ApiResult<content::RawJson<String>> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let page_metadata = PageMetadata::from_cursor(cursor.as_ref().unwrap_or(&"".to_string()));
    let safe_info = info_provider.safe_info(&safe_address).await?;

    // Build Safe Transaction Service URL (with pagination)
    let url = core_uri!(info_provider, "/v1/safes/{}/messages/", safe_address)?;
    let mut url = Url::parse(&url).map_err(|_| ApiError {
        status: 500,
        details: ErrorDetails {
            code: 500,
            message: None,
            arguments: None,
            debug: None,
        },
    })?;
    url.set_query(Some(&page_metadata.to_url_string()));

    // Request
    let http_request = Request::new(url.into_string());
    let body = info_provider.client().get(http_request).await?.body;

    let messages_page: Page<Message> = serde_json::from_str::<Page<Message>>(&body)?;
    let messages: Vec<FrontendMessage> = future::join_all(
        messages_page
            .results
            .iter()
            .map(|message| map_message(&info_provider, &safe_info, &message)),
    )
    .await;

    let next_pagination: Option<PageMetadata> = match messages_page.next {
        None => None,
        Some(next) => page_metadata_from_url(&next),
    };

    let previous_pagination: Option<PageMetadata> = match messages_page.previous {
        None => None,
        Some(previous) => page_metadata_from_url(&previous),
    };

    let body = Page {
        next: get_route_url(&context, &chain_id, &safe_address, &next_pagination),
        previous: get_route_url(&context, &chain_id, &safe_address, &previous_pagination),
        results: messages,
    };

    let body = serde_json::to_string(&body)?;
    return Ok(content::RawJson(body));
}

#[post(
    "/v1/chains/<chain_id>/safes/<safe_address>/messages",
    format = "application/json",
    data = "<message_payload>"
)]
pub async fn create_message(
    context: RequestContext,
    chain_id: String,
    safe_address: String,
    message_payload: Json<CreateMessage>,
) -> ApiResult<String> {
    let info_provider = DefaultInfoProvider::new(&chain_id, &context);
    let url = core_uri!(info_provider, "/v1/safes/{}/messages/", safe_address)?;

    let request = {
        let mut request = Request::new(url);
        request.body(serde_json::to_string(&message_payload.0).ok());
        request
    };
    let response_body: Response = context.http_client().post(request).await?;
    return Ok(response_body.body);
}

fn get_route_url(
    context: &RequestContext,
    chain_id: &str,
    safe_address: &str,
    page_metadata: &Option<PageMetadata>,
) -> Option<String> {
    let cursor: String = page_metadata
        .as_ref()
        .map(|page_metadata| page_metadata.to_url_string())?;
    let absolute_uri: String = build_absolute_uri(
        &context,
        uri!(get_messages(
            chain_id = chain_id.to_string(),
            safe_address = safe_address.to_string(),
            cursor = Some(cursor),
        )),
    );
    return Some(absolute_uri);
}

fn page_metadata_from_url(url: &str) -> Option<PageMetadata> {
    let url = Url::parse(url).ok()?;
    let query_pairs = url.query_pairs();
    let mut limit: u64 = 20;
    let mut offset: u64 = 0;

    for pair in query_pairs {
        match pair.0 {
            Cow::Borrowed("limit") => {
                limit = pair.1.parse::<u64>().ok()?;
            }
            Cow::Borrowed("offset") => {
                offset = pair.1.parse::<u64>().ok()?;
            }
            _ => {}
        }
    }

    return Some(PageMetadata { offset, limit });
}

async fn map_message(
    info_provider: &(impl InfoProvider + Sync),
    safe_info: &SafeInfo,
    message: &Message,
) -> FrontendMessage {
    let confirmations_required = safe_info.threshold as usize;
    let confirmations_submitted = message.confirmations.len();

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

    return FrontendMessage::Message {
        message_hash: message.message_hash.to_string(),
        status: if confirmations_submitted >= confirmations_required {
            MessageStatus::Confirmed
        } else {
            MessageStatus::NeedsConfirmation
        },
        logo_uri: "".to_string(), // TODO fetch from safe-config-service using Safe App Id
        name: "".to_string(),     // TODO fetch from safe-config-service using Safe App Id
        message: message.message.to_string(),
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
