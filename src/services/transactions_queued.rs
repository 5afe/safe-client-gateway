use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::models::backend::transactions::MultisigTransaction;
use crate::models::commons::{Page, PageMetadata};
use crate::models::service::transactions::summary::{
    ConflictType, Label, TransactionListItem, TransactionSummary,
};
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use crate::utils::errors::ApiResult;
use crate::utils::extract_query_string;
use itertools::Itertools;
use log::debug;

// use https://doc.rust-lang.org/std/iter/trait.Iterator.html#method.peekable
pub fn get_queued_transactions(
    context: &Context,
    safe_address: &String,
    page_url: &Option<String>,
    timezone_offset: &Option<String>,
    trusted: &Option<bool>,
) -> ApiResult<Page<TransactionListItem>> {
    let mut info_provider = DefaultInfoProvider::new(context);

    // Parse page meta (offset and limit)
    let page_meta = PageMetadata::from_url_string(page_url.as_ref().unwrap_or(&"".to_string()));
    // Adjust the page meta to fetch additional information of adjacent pages
    let adjusted_page_meta = adjust_page_meta(&page_meta);

    // Allow to also query queued transactions that are not submitted by an owner or delegate
    let displayTrustedOnly = trusted.unwrap_or(true);

    // As we require the Safe nonce later we use it here explicitely to query transaction that are in the future
    let safe_nonce = info_provider.safe_info(safe_address)?.nonce as i64;
    let url = format!(
        "{}/v1/safes/{}/multisig-transactions/?{}&nonce__gte={}&ordering=+nonce&trusted={}",
        base_transaction_service_url(),
        safe_address,
        adjusted_page_meta.to_url_string(),
        safe_nonce,
        displayTrustedOnly
    );

    let body = context
        .cache()
        .request_cached(&context.client(), &url, request_cache_duration())?;
    let mut backend_transactions: Page<MultisigTransaction> = serde_json::from_str(&body)?;

    // If there is a next url we remove the last item for information on the next page
    let after_tx = if backend_transactions.next.is_some() {
        backend_transactions.results.pop()
    } else {
        None
    };
    // Use an iterator to avoid shifting the result vector (would potentially trigger copies)
    let mut tx_iter = backend_transactions.results.into_iter();
    // If we are not on the first page then we take the first item to get information on the previous page
    let before_tx = if page_meta.offset == 0 {
        None
    } else {
        tx_iter.next()
    };
    // Nonce of the first item in the next page (-1 if not present)
    let edge_nonce = after_tx.map_or(-1, |tx| tx.nonce as i64);
    // Nonce of the last item in the previous page (-1 if not present)
    let previous_page_nonce = before_tx.map_or(-1, |tx| tx.nonce as i64);
    let mut last_proccessed_nonce = previous_page_nonce;

    let mut service_transactions: Vec<TransactionListItem> = Vec::new();
    for (group_nonce, transaction_group) in
        &tx_iter.group_by(|transaction| transaction.nonce as i64)
    {
        // Check if we need to add section headers
        if last_proccessed_nonce < safe_nonce && group_nonce == safe_nonce {
            // If the last nonce processed was the initial nonce (-1) and this group nonce is the current Safe nonce then we start the Next section
            service_transactions.push(TransactionListItem::Label { label: Label::Next })
        } else if last_proccessed_nonce <= safe_nonce && group_nonce > safe_nonce {
            // If the last nonce processed was the initial nonce (-1) or the current Safe nonce and this group nonce higher than the Safe nonce then we start the Queue section
            service_transactions.push(TransactionListItem::Label {
                label: Label::Queued,
            })
        } // Else: If the last proccessed nonce is higher than the current Safe nonce then all headers should already be present
        // Update last proccessed nonce
        last_proccessed_nonce = group_nonce as i64;

        // Make the group peekable for conflict type checks
        let mut group_iter = transaction_group.peekable();
        // There will be always at least one transaction for a group
        let group_start_tx = group_iter.next().unwrap();
        // Check if this group has the same nonce as the starting item of the next page
        let is_edge_group = group_nonce == edge_nonce;
        // This group has the same nonce as the last group from the previous page => group continues
        let conflict_from_previous_page = previous_page_nonce == group_nonce;
        // If there is more than 1 item in this group or we are in an edge group then we have a conflict
        let has_conflicts = group_iter.peek().is_some() || is_edge_group;
        // If we start a new conflict group then we should render the conflict header
        if (has_conflicts && !conflict_from_previous_page) {
            service_transactions.push(TransactionListItem::ConflictHeader {
                nonce: group_nonce as u64,
            })
        }
        // Add the one transaction that is always present
        add_transation_as_summary(
            &mut info_provider,
            &mut service_transactions,
            &group_start_tx,
            if has_conflicts {
                // We have more conflicts in this or the next page
                ConflictType::HasNext
            } else if conflict_from_previous_page {
                // We continue the group from the previous page but there are no additional conflicts in this page
                ConflictType::End
            } else {
                // No conflict in this or the previous page
                ConflictType::None
            },
        );
        // Add additional conflicts of the group (only present when conflicts in the same page)
        while let Some(tx) = group_iter.next() {
            // Indicate if we are in a conflict group on the edge or if there are more conflicts in this page
            // Else indicate that we are at the end of the conflict group
            let conflict_type = if group_iter.peek().is_some() || is_edge_group {
                ConflictType::HasNext
            } else {
                ConflictType::End
            };
            add_transation_as_summary(
                &mut info_provider,
                &mut service_transactions,
                &tx,
                conflict_type,
            );
        }
    }

    Ok(Page {
        next: build_page_url(
            context,
            &safe_address,
            &page_meta,
            timezone_offset,
            displayTrustedOnly,
            backend_transactions.next,
            1, // Direction forward
        ),
        previous: build_page_url(
            context,
            &safe_address,
            &page_meta,
            timezone_offset,
            displayTrustedOnly,
            backend_transactions.previous,
            -1, // Direction backwards
        ),
        results: service_transactions,
    })
}

fn build_page_url(
    context: &Context,
    safe_address: &String,
    page_meta: &PageMetadata,
    timezone_offset: &Option<String>,
    displayTrustedOnly: bool,
    url: Option<String>,
    direction: i64,
) -> Option<String> {
    url.as_ref()
        .and_then(|link| extract_query_string(link))
        .map(|link| {
            context.build_absolute_url(uri!(
                crate::routes::transactions::queued_transactions: safe_address,
                offset_page_meta(page_meta, direction * (page_meta.limit as i64)),
                timezone_offset.clone().unwrap_or("0".to_string()),
                displayTrustedOnly
            ))
        })
}

fn offset_page_meta(meta: &PageMetadata, offset: i64) -> String {
    PageMetadata {
        offset: (meta.offset + (offset as i64) as u64),
        limit: meta.limit,
    }
    .to_url_string()
}

fn adjust_page_meta(meta: &PageMetadata) -> PageMetadata {
    if meta.offset == 0 {
        PageMetadata {
            offset: 0,
            limit: meta.limit + 1,
        }
    } else {
        PageMetadata {
            offset: meta.offset - 1,
            limit: meta.limit + 2,
        }
    }
}

fn add_transation_as_summary(
    info_provider: &mut InfoProvider,
    items: &mut Vec<TransactionListItem>,
    transaction: &MultisigTransaction,
    conflict_type: ConflictType,
) {
    // Converting a multisig transaction theoretically can result in multiple summaries
    let mut tx_summary_iter = transaction
        .to_transaction_summary(info_provider)
        .unwrap_or(vec![])
        .into_iter()
        .peekable();
    while let Some(summary) = tx_summary_iter.next() {
        // If the summary items are based on an "End" item in a conflict group then we need to make sure that only the last is marked as the "End"
        let tx_conflict_type =
            if conflict_type == ConflictType::End && tx_summary_iter.peek().is_some() {
                ConflictType::HasNext
            } else {
                conflict_type.clone()
            };
        items.push(TransactionListItem::Transaction {
            transaction_summary: summary,
            conflict_type: tx_conflict_type,
        });
    }
}
