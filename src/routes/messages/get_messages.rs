use super::frontend_models::Message as FrontendMessage;
use crate::common::models::page::{Page, PageMetadata};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::routes::messages::backend_models::Message;
use crate::routes::messages::get_messages::MessageItem::DateLabel;
use crate::routes::messages::message_mapper::map_message;
use crate::utils::context::RequestContext;
use crate::utils::errors::{ApiError, ApiResult, ErrorDetails};
use crate::utils::http_client::Request;
use crate::utils::urls::build_absolute_uri;
use chrono::{DateTime, Datelike, NaiveDate, Utc};
use itertools::Itertools;
use reqwest::Url;
use rocket::response::content;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Serialize, Debug)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
enum MessageItem {
    Message(FrontendMessage),
    #[serde(rename_all = "camelCase")]
    DateLabel {
        timestamp: i64,
    },
}

#[get("/v1/chains/<chain_id>/safes/<safe_address>/messages?<cursor>")]
pub async fn route(
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
    let message_groups: Vec<(Option<NaiveDate>, Vec<Message>)> =
        group_messages_by_date(messages_page.results);

    // Build final collection which includes DateLabels and Message types
    let mut message_items: Vec<MessageItem> = vec![];
    for (date_header, messages) in message_groups {
        // Maps the group header into a UNIX timestamp
        let date_label_item: Option<MessageItem> = date_header
            .and_then(|date| date.and_hms_nano_opt(0, 0, 0, 0))
            .map(|date| DateLabel {
                timestamp: date.timestamp_millis(),
            });

        // If we have a resulting DateLabel we push it
        if let Some(message_item) = date_label_item {
            message_items.push(message_item);
        }

        for message in messages {
            let message_item = map_message(&info_provider, &safe_info, &message).await;
            message_items.push(MessageItem::Message(message_item));
        }
    }

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
        results: message_items,
    };

    let body = serde_json::to_string(&body)?;
    return Ok(content::RawJson(body));
}

fn group_messages_by_date(messages: Vec<Message>) -> Vec<(Option<NaiveDate>, Vec<Message>)> {
    let groups = messages
        .into_iter()
        // Sort by descending order (grouping works on consecutive entries)
        .sorted_by(|m1, m2| m2.created.cmp(&m1.created))
        // Group by date
        .group_by(|message| {
            let message_date: DateTime<Utc> = message.created;
            return NaiveDate::from_ymd_opt(
                message_date.year(),
                message_date.month(),
                message_date.day(),
            );
        });

    let mut data_grouped: Vec<(Option<NaiveDate>, Vec<Message>)> = Vec::new();
    for (key, group) in &groups {
        data_grouped.push((key, group.collect()));
    }
    return data_grouped;
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
        uri!(route(
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
