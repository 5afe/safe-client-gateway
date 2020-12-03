use crate::config::{base_transaction_service_url, request_cache_duration};
use crate::models::backend::transactions::MultisigTransaction;
use crate::models::commons::{Page, PageMetadata};
use crate::models::service::transactions::summary::{
    ConflictType, TransactionListItem, TransactionSummary,
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
    let page_meta = PageMetadata::from_url_string(page_url.as_ref().unwrap_or(&"".to_string())).unwrap();
    let adjusted_page_meta = adjust_page_meta(&page_meta);
    let displayTrustedOnly = trusted.unwrap_or(true);
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
    debug!("request URL: {}", &url);
    debug!("page_url: {:#?}", page_url);

    let mut backend_transactions: Page<MultisigTransaction> = serde_json::from_str(&body)?;
    let after_tx = if backend_transactions.next.is_some() { backend_transactions.results.pop() } else { None };
    let mut tx_iter = backend_transactions.results.into_iter();
    let before_tx = if page_meta.offset == 0 { None } else { tx_iter.next() };

    let edge_nonce = after_tx.map_or(-1, |tx| tx.nonce as i64);
    let previous_page_nonce = before_tx.map_or(-1, |tx| tx.nonce as i64);
    let mut late_proccessed_nonce = previous_page_nonce;

    let mut service_transactions: Vec<TransactionListItem> = Vec::new();
    for (group_nonce, transaction_group) in &tx_iter.group_by(|transaction| transaction.nonce as i64 )
    {
        let mut group_iter = transaction_group.peekable();
        if late_proccessed_nonce < safe_nonce && group_nonce == safe_nonce {
            service_transactions.push(TransactionListItem::StringLabel {
                label: "Next".to_string()
            })
        } else if late_proccessed_nonce == safe_nonce && group_nonce > safe_nonce {
            service_transactions.push(TransactionListItem::StringLabel {
                label: "Queue".to_string()
            })
        }
        late_proccessed_nonce = group_nonce as i64;
        let group_start_tx = group_iter.next().unwrap();
        let is_edge_group = group_nonce == edge_nonce;
        let has_conflicts = group_iter.peek().is_some() || is_edge_group;
        let conflict_from_previous_page = previous_page_nonce == group_nonce;
        if (has_conflicts) {
            service_transactions.push(TransactionListItem::StringLabel {
                label: format!("Conflict {}", group_nonce)
            })
        }
        add_transation_as_summary(
            &mut info_provider, 
            &mut service_transactions, 
            &group_start_tx, 
            if has_conflicts { 
                ConflictType::HasNext 
            } else if conflict_from_previous_page {
                ConflictType::End 
            } else { 
                ConflictType::None 
            }
        );
        if (has_conflicts || conflict_from_previous_page) {
            while let Some(tx) = group_iter.next() {
                let conflict_type = if group_iter.peek().is_some() { ConflictType::HasNext } else { ConflictType::End };
                add_transation_as_summary(&mut info_provider, &mut service_transactions, &tx, conflict_type);
            }
        }
        /* 
        transaction
                .to_transaction_summary(&mut info_provider)
                .unwrap_or(vec![])
        transaction_group.for_each(|tx| {
            service_transactions_with_dates.push(TransactionListItem::Transaction {
                transaction_summary: tx,
                conflict_type: ConflictType::None,
            })
        });
        */
    }

    Ok(Page {
        next: build_page_url(context, &safe_address, &page_meta, timezone_offset, displayTrustedOnly, backend_transactions.next, 1),
        previous: build_page_url(context, &safe_address, &page_meta, timezone_offset, displayTrustedOnly, backend_transactions.previous, -1),
        results: service_transactions,
    })
}

fn build_page_url(context: &Context, safe_address: &String, page_meta: &PageMetadata, timezone_offset: &Option<String>, displayTrustedOnly: bool, url: Option<String>, direction: i64) -> Option<String> {
    url
        .as_ref()
        .and_then(|link| extract_query_string(link))
        .map(|link| {
            context.build_absolute_url(uri!(
                crate::routes::transactions::queued_transactions: safe_address,
                offset_page_meta(page_meta, direction * page_meta.limit),
                timezone_offset.clone().unwrap_or("0".to_string()),
                displayTrustedOnly
            ))
        })
}

fn offset_page_meta(meta: &PageMetadata, offset: i64) -> String {
    PageMetadata {
        offset: meta.offset + offset,
        limit: meta.limit
    }.to_url_string()
}

fn adjust_page_meta(meta: &PageMetadata) -> PageMetadata {
    if meta.offset == 0 {
        PageMetadata {
            offset: 0,
            limit: meta.limit + 1
        }
    } else {
        PageMetadata {
            offset: meta.offset - 1,
            limit: meta.limit + 2
        }
    }
}

fn add_transation_as_summary(info_provider: &mut InfoProvider, items: &mut Vec<TransactionListItem>, transaction: &MultisigTransaction, conflict_type: ConflictType) {
    let mut tx_summary_iter = transaction
        .to_transaction_summary(info_provider)
        .unwrap_or(vec![])
        .into_iter()
        .peekable();
    while let Some(summary) = tx_summary_iter.next() {
        let tx_conflict_type = if conflict_type == ConflictType::End && tx_summary_iter.peek().is_some() { ConflictType::HasNext } else { conflict_type.clone() };
        items.push(TransactionListItem::Transaction {
            transaction_summary: summary,
            conflict_type: tx_conflict_type,
        });
    }
}
