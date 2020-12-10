use crate::json::{MULTISIG_TX_AWAITING_EXECUTION, MULTISIG_TX_SETTINGS_CHANGE};
use crate::models::backend::transactions::MultisigTransaction;
use crate::models::commons::{Page, PageMetadata};
use crate::providers::info::*;
use crate::services::transactions_queued::{
    adjust_page_meta, get_edge_nonce, get_previous_page_nonce,
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
    let edge_tx = get_multisig_tx(MULTISIG_TX_AWAITING_EXECUTION);
    let expected = edge_tx.nonce as i64;
    let results = vec![get_multisig_tx(MULTISIG_TX_SETTINGS_CHANGE), edge_tx];
    let mut page: Page<MultisigTransaction> = Page {
        results,
        previous: None,
        next: Some("some_url".to_string()),
    };

    let actual = get_edge_nonce(&mut page);

    assert_eq!(expected, actual);
    assert_eq!(
        vec![get_multisig_tx(MULTISIG_TX_SETTINGS_CHANGE)],
        page.results
    );
}

#[test]
fn get_edge_nonce_without_next() {
    let results = vec![
        get_multisig_tx(MULTISIG_TX_SETTINGS_CHANGE),
        get_multisig_tx(MULTISIG_TX_AWAITING_EXECUTION),
    ];
    let mut page: Page<MultisigTransaction> = Page {
        results,
        previous: None,
        next: None,
    };

    let actual = get_edge_nonce(&mut page);

    assert_eq!(-1, actual);

    let expected = vec![
        get_multisig_tx(MULTISIG_TX_SETTINGS_CHANGE),
        get_multisig_tx(MULTISIG_TX_AWAITING_EXECUTION),
    ];
    assert_eq!(expected, page.results);
}

#[test]
fn get_previous_page_nonce_offset_0() {
    let page_meta = PageMetadata {
        offset: 0,
        limit: 20,
    };
    let results = vec![
        get_multisig_tx(MULTISIG_TX_AWAITING_EXECUTION),
        get_multisig_tx(MULTISIG_TX_SETTINGS_CHANGE),
    ];
    let mut results_iter = results.into_iter();

    let actual = get_previous_page_nonce(&page_meta, &mut results_iter);

    assert_eq!(-1, actual);
    assert_eq!(
        get_multisig_tx(MULTISIG_TX_AWAITING_EXECUTION),
        results_iter.next().unwrap()
    );
}

#[test]
fn get_previous_page_nonce_offset_greater_than_0() {
    let page_meta = PageMetadata {
        offset: 20,
        limit: 20,
    };
    let previous_page_tx = get_multisig_tx(MULTISIG_TX_AWAITING_EXECUTION);
    let expected = previous_page_tx.nonce as i64;
    let results = vec![
        previous_page_tx,
        get_multisig_tx(MULTISIG_TX_SETTINGS_CHANGE),
    ];
    let mut results_iter = results.into_iter();

    let actual = get_previous_page_nonce(&page_meta, &mut results_iter);

    assert_eq!(expected, actual);
    assert_eq!(
        get_multisig_tx(MULTISIG_TX_SETTINGS_CHANGE),
        results_iter.next().unwrap()
    );
}

fn get_multisig_tx(source: &str) -> MultisigTransaction {
    serde_json::from_str::<MultisigTransaction>(source).unwrap()
}
