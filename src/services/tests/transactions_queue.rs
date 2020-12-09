use crate::json::BACKEND_TRANSACTION_LIST_PAGE;
use crate::models::backend::transactions::Transaction;
use crate::models::commons::{Page, PageMetadata};
use crate::models::service::transactions::summary::{
    ConflictType, TransactionListItem, TransactionSummary,
};
use crate::models::service::transactions::TransactionStatus::Success;
use crate::models::service::transactions::TransferDirection::{Incoming, Outgoing};
use crate::models::service::transactions::{Custom, TransactionInfo, Transfer};
use crate::models::service::transactions::{Erc20Transfer, TransferInfo};
use crate::providers::info::*;
use crate::services::transactions_queued::{
    adjust_page_meta
};

#[test]
fn adjust_page_meta_offset_0() {
    let input = PageMetadata {
        offset: 0,
        limit: 50,
    };
    let expected = PageMetadata {
        offset: 0,
        limit: 51,
    };

    let actual = adjust_page_meta(&input);

    assert_eq!(expected, actual);
}

#[test]
fn adjust_page_meta_offset_greater_than_0() {
    let input = PageMetadata {
        offset: 1,
        limit: 50,
    };
    let expected = PageMetadata {
        offset: 0,
        limit: 52,
    };

    let actual = adjust_page_meta(&input);

    assert_eq!(expected, actual);
}

#[test]
fn get_edge_nonce_with_next() {
}
