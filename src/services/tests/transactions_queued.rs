use crate::json::{
    BACKEND_QUEUED_TRANSACTION_LIST_PAGE_CONFLICT_393,
    BACKEND_QUEUED_TRANSACTION_LIST_PAGE_CONFLICT_394,
    BACKEND_QUEUED_TRANSACTION_LIST_PAGE_NO_CONFLICTS, MULTISIG_TX_AWAITING_EXECUTION,
    MULTISIG_TX_SETTINGS_CHANGE, TOKEN_BAT,
};
use crate::models::backend::transactions::MultisigTransaction;
use crate::models::commons::{Page, PageMetadata};
use crate::models::service::transactions::summary::{
    ConflictType, ExecutionInfo, Label, TransactionListItem, TransactionSummary,
};
use crate::models::service::transactions::TransferDirection::Outgoing;
use crate::models::service::transactions::{
    Erc20Transfer, TransactionInfo, TransactionStatus, Transfer, TransferInfo,
};
use crate::providers::info::*;
use crate::services::transactions_queued::{
    adjust_page_meta, get_edge_nonce, get_previous_page_nonce, process_transactions,
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

#[rocket::async_test]
async fn process_transactions_empty_list() {
    let input_list: Vec<MultisigTransaction> = vec![];
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);

    let mut tx_iter = input_list.into_iter();
    let safe_nonce = 0;
    let previous_page_nonce = 0;
    let edge_nonce = 0;

    let actual = process_transactions(
        &mut mock_info_provider,
        safe_nonce,
        &mut tx_iter,
        previous_page_nonce,
        edge_nonce,
    )
    .await;

    let expected: Vec<TransactionListItem> = vec![];

    assert_eq!(expected, actual);
}

#[test]
fn process_transactions_no_conflicts_everything_queued() {
    let input_list: Vec<MultisigTransaction> = serde_json::from_str::<Page<MultisigTransaction>>(
        BACKEND_QUEUED_TRANSACTION_LIST_PAGE_NO_CONFLICTS,
    )
    .unwrap()
    .results;

    let bat_token_info = serde_json::from_str::<TokenInfo>(TOKEN_BAT).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(
        r#"{
              "address": "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
              "nonce": 391,
              "threshold": 3,
              "owners": [
                "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0",
                "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164",
                "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"
              ],
              "masterCopy": "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F",
              "modules": [],
              "fallbackHandler": "0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44",
              "version": "1.1.1"
            }"#,
    )
    .unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(3)
        .returning(move |_| Ok(safe_info.clone()));
    mock_info_provider
        .expect_token_info()
        .times(3)
        .returning(move |_| Ok(bat_token_info.clone()));
    mock_info_provider
        .expect_full_address_info_search()
        .times(3)
        .returning(move |_| bail!("No address info"));

    let mut tx_iter = input_list.into_iter();

    let safe_nonce = 391;
    let previous_page_nonce = -1;
    let edge_nonce = -1;

    let actual = process_transactions(
        &mut mock_info_provider,
        safe_nonce,
        &mut tx_iter,
        previous_page_nonce,
        edge_nonce,
    );

    let expected: Vec<TransactionListItem> = vec![
        TransactionListItem::Label {
            label: Label::Queued,
        },
        TransactionListItem::Transaction {
            transaction: TransactionSummary {
                id: "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x0fe072e76498e0db46fc79113662026a4f8fb34e840491aefeff6dec21c766cb".to_string(),
                timestamp: 1607602242476,
                tx_status: TransactionStatus::AwaitingConfirmations,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                    sender_info: None,
                    recipient: "0xf2565317F3Ae8Ae9EA98E9Fe1e7FADC77F823cbD".to_string(),
                    recipient_info: None,
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(Erc20Transfer {
                        token_address: "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46".to_string(),
                        token_name: Some("BigAmount".to_string()),
                        token_symbol: Some("BA-T".to_string()),
                        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46.png".to_string()),
                        decimals: Some(1),
                        value: "10".to_string()
                    })
                }),
                execution_info: Some(ExecutionInfo{
                    nonce: 392,
                    confirmations_required: 3,
                    confirmations_submitted:1,
                    missing_signers: Some(vec!["0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(), "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164".to_string()])
                }),
                safe_app_info: None,
            },

            conflict_type: ConflictType::None,

        },
        TransactionListItem::Transaction {
            transaction: TransactionSummary {
                id: "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa".to_string(),
                timestamp: 1607602284354,
                tx_status: TransactionStatus::AwaitingConfirmations,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                    sender_info: None,
                    recipient: "0xF353eBBa77e5E71c210599236686D51cA1F88b84".to_string(),
                    recipient_info: None,
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(Erc20Transfer {
                        token_address: "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46".to_string(),
                        token_name: Some("BigAmount".to_string()),
                        token_symbol: Some("BA-T".to_string()),
                        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46.png".to_string()),
                        decimals: Some(1),
                        value: "20".to_string()
                    })
                }),
                execution_info: Some(ExecutionInfo{
                    nonce: 393,
                    confirmations_required: 3,
                    confirmations_submitted:1,
                    missing_signers: Some(vec!["0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(), "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164".to_string()])
                }),
                safe_app_info: None,
            },
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: TransactionSummary {
                id: "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0xca7a464a3479af396c2975b4b3f5f7b90fc56747404ebaad5ec838c2954d2f9c".to_string(),
                timestamp: 1607602424072,
                tx_status: TransactionStatus::AwaitingConfirmations,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                    sender_info: None,
                    recipient: "0xF353eBBa77e5E71c210599236686D51cA1F88b84".to_string(),
                    recipient_info: None,
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(Erc20Transfer {
                        token_address: "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46".to_string(),
                        token_name: Some("BigAmount".to_string()),
                        token_symbol: Some("BA-T".to_string()),
                        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46.png".to_string()),
                        decimals: Some(1),
                        value: "20".to_string()
                    })
                }),
                execution_info: Some(ExecutionInfo{
                    nonce: 394,
                    confirmations_required: 3,
                    confirmations_submitted:1,
                    missing_signers: Some(vec!["0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(), "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164".to_string()])
                }),
                safe_app_info: None,
            },
            conflict_type: ConflictType::None,
        },
    ];

    assert_eq!(expected, actual);
}

#[test]
fn process_transactions_conflicts_in_queued() {
    let input_list: Vec<MultisigTransaction> = serde_json::from_str::<Page<MultisigTransaction>>(
        BACKEND_QUEUED_TRANSACTION_LIST_PAGE_CONFLICT_394,
    )
    .unwrap()
    .results;

    let bat_token_info = serde_json::from_str::<TokenInfo>(TOKEN_BAT).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(
        r#"{
              "address": "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
              "nonce": 393,
              "threshold": 3,
              "owners": [
                "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0",
                "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164",
                "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"
              ],
              "masterCopy": "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F",
              "modules": [],
              "fallbackHandler": "0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44",
              "version": "1.1.1"
            }"#,
    )
    .unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(3)
        .returning(move |_| Ok(safe_info.clone()));
    mock_info_provider
        .expect_token_info()
        .times(3)
        .returning(move |_| Ok(bat_token_info.clone()));
    mock_info_provider
        .expect_full_address_info_search()
        .times(3)
        .returning(move |_| bail!("No address info"));

    let mut tx_iter = input_list.into_iter();

    let safe_nonce = 393;
    let previous_page_nonce = -1;
    let edge_nonce = -1;

    let actual = process_transactions(
        &mut mock_info_provider,
        safe_nonce,
        &mut tx_iter,
        previous_page_nonce,
        edge_nonce,
    );

    let expected: Vec<TransactionListItem> = vec![
        TransactionListItem::Label {
            label: Label::Next,
        },
        TransactionListItem::Transaction {
            transaction: TransactionSummary {
                id: "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x0fe072e76498e0db46fc79113662026a4f8fb34e840491aefeff6dec21c766cb".to_string(),
                timestamp: 1607602242476,
                tx_status: TransactionStatus::AwaitingConfirmations,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                    sender_info: None,
                    recipient: "0xf2565317F3Ae8Ae9EA98E9Fe1e7FADC77F823cbD".to_string(),
                    recipient_info: None,
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(Erc20Transfer {
                        token_address: "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46".to_string(),
                        token_name: Some("BigAmount".to_string()),
                        token_symbol: Some("BA-T".to_string()),
                        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46.png".to_string()),
                        decimals: Some(1),
                        value: "10".to_string()
                    })
                }),
                execution_info: Some(ExecutionInfo{
                    nonce: 393,
                    confirmations_required: 3,
                    confirmations_submitted:1,
                    missing_signers: Some(vec!["0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(), "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164".to_string()])
                }),
                safe_app_info: None,
            },
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Label {
            label: Label::Queued,
        },
        TransactionListItem::ConflictHeader {
            nonce: 394
        },
        TransactionListItem::Transaction {
            transaction: TransactionSummary {
                id: "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa".to_string(),
                timestamp: 1607602284354,
                tx_status: TransactionStatus::AwaitingConfirmations,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                    sender_info: None,
                    recipient: "0xF353eBBa77e5E71c210599236686D51cA1F88b84".to_string(),
                    recipient_info: None,
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(Erc20Transfer {
                        token_address: "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46".to_string(),
                        token_name: Some("BigAmount".to_string()),
                        token_symbol: Some("BA-T".to_string()),
                        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46.png".to_string()),
                        decimals: Some(1),
                        value: "20".to_string()
                    })
                }),
                execution_info: Some(ExecutionInfo{
                    nonce: 394,
                    confirmations_required: 3,
                    confirmations_submitted:1,
                    missing_signers: Some(vec!["0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(), "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164".to_string()])
                }),
                safe_app_info: None,
            },
            conflict_type: ConflictType::HasNext,
        },
        TransactionListItem::Transaction {
            transaction: TransactionSummary {
                id: "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0xca7a464a3479af396c2975b4b3f5f7b90fc56747404ebaad5ec838c2954d2f9c".to_string(),
                timestamp: 1607602424072,
                tx_status: TransactionStatus::AwaitingConfirmations,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                    sender_info: None,
                    recipient: "0xF353eBBa77e5E71c210599236686D51cA1F88b84".to_string(),
                    recipient_info: None,
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(Erc20Transfer {
                        token_address: "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46".to_string(),
                        token_name: Some("BigAmount".to_string()),
                        token_symbol: Some("BA-T".to_string()),
                        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46.png".to_string()),
                        decimals: Some(1),
                        value: "20".to_string()
                    })
                }),
                execution_info: Some(ExecutionInfo{
                    nonce: 394,
                    confirmations_required: 3,
                    confirmations_submitted:1,
                    missing_signers: Some(vec!["0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(), "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164".to_string()])
                }),
                safe_app_info: None,
            },
            conflict_type: ConflictType::End,
        }
    ];

    assert_eq!(expected, actual);
}

#[test]
fn process_transactions_conflicts_in_next() {
    let input_list: Vec<MultisigTransaction> = serde_json::from_str::<Page<MultisigTransaction>>(
        BACKEND_QUEUED_TRANSACTION_LIST_PAGE_CONFLICT_393,
    )
    .unwrap()
    .results;

    let bat_token_info = serde_json::from_str::<TokenInfo>(TOKEN_BAT).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(
        r#"{
              "address": "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
              "nonce": 393,
              "threshold": 3,
              "owners": [
                "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0",
                "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164",
                "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"
              ],
              "masterCopy": "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F",
              "modules": [],
              "fallbackHandler": "0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44",
              "version": "1.1.1"
            }"#,
    )
    .unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(3)
        .returning(move |_| Ok(safe_info.clone()));
    mock_info_provider
        .expect_token_info()
        .times(3)
        .returning(move |_| Ok(bat_token_info.clone()));
    mock_info_provider
        .expect_full_address_info_search()
        .times(3)
        .returning(move |_| bail!("No address info"));

    let mut tx_iter = input_list.into_iter();

    let safe_nonce = 393;
    let previous_page_nonce = -1;
    let edge_nonce = -1;

    let actual = process_transactions(
        &mut mock_info_provider,
        safe_nonce,
        &mut tx_iter,
        previous_page_nonce,
        edge_nonce,
    );

    let expected: Vec<TransactionListItem> = vec![
        TransactionListItem::Label {
            label: Label::Next,
        },
        TransactionListItem::ConflictHeader {
            nonce: 393
        },
        TransactionListItem::Transaction {
            transaction: TransactionSummary {
                id: "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x0fe072e76498e0db46fc79113662026a4f8fb34e840491aefeff6dec21c766cb".to_string(),
                timestamp: 1607602242476,
                tx_status: TransactionStatus::AwaitingConfirmations,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                    sender_info: None,
                    recipient: "0xf2565317F3Ae8Ae9EA98E9Fe1e7FADC77F823cbD".to_string(),
                    recipient_info: None,
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(Erc20Transfer {
                        token_address: "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46".to_string(),
                        token_name: Some("BigAmount".to_string()),
                        token_symbol: Some("BA-T".to_string()),
                        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46.png".to_string()),
                        decimals: Some(1),
                        value: "10".to_string()
                    }),

                }),
                execution_info: Some(ExecutionInfo{
                    nonce: 393,
                    confirmations_required: 3,
                    confirmations_submitted:1,
                    missing_signers: Some(vec!["0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(), "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164".to_string()])
                }),
                safe_app_info: None,
            },
            conflict_type: ConflictType::HasNext,
        },
        TransactionListItem::Transaction {
            transaction: TransactionSummary {
                id: "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa".to_string(),
                timestamp: 1607602284354,
                tx_status: TransactionStatus::AwaitingConfirmations,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                    sender_info: None,
                    recipient: "0xF353eBBa77e5E71c210599236686D51cA1F88b84".to_string(),
                    recipient_info: None,
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(Erc20Transfer {
                        token_address: "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46".to_string(),
                        token_name: Some("BigAmount".to_string()),
                        token_symbol: Some("BA-T".to_string()),
                        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46.png".to_string()),
                        decimals: Some(1),
                        value: "20".to_string()
                    })
                }),
                execution_info: Some(ExecutionInfo{
                    nonce: 393,
                    confirmations_required: 3,
                    confirmations_submitted:1,
                    missing_signers: Some(vec!["0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(), "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164".to_string()])
                }),
                safe_app_info: None,
            },
            conflict_type: ConflictType::End,
        },
        TransactionListItem::Label {
            label: Label::Queued,
        },
        TransactionListItem::Transaction {
            transaction: TransactionSummary {
                id: "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0xca7a464a3479af396c2975b4b3f5f7b90fc56747404ebaad5ec838c2954d2f9c".to_string(),
                timestamp: 1607602424072,
                tx_status: TransactionStatus::AwaitingConfirmations,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                    sender_info: None,
                    recipient: "0xF353eBBa77e5E71c210599236686D51cA1F88b84".to_string(),
                    recipient_info: None,
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(Erc20Transfer {
                        token_address: "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46".to_string(),
                        token_name: Some("BigAmount".to_string()),
                        token_symbol: Some("BA-T".to_string()),
                        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46.png".to_string()),
                        decimals: Some(1),
                        value: "20".to_string()
                    })
                }),
                execution_info: Some(ExecutionInfo{
                    nonce: 394,
                    confirmations_required: 3,
                    confirmations_submitted:1,
                    missing_signers: Some(vec!["0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(), "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164".to_string()])
                }),
                safe_app_info: None,
            },
            conflict_type: ConflictType::None,
        }
    ];

    assert_eq!(expected, actual);
}

#[test]
fn process_transactions_conflicts_in_queued_spanning_to_next_page() {
    let input_list: Vec<MultisigTransaction> = serde_json::from_str::<Page<MultisigTransaction>>(
        BACKEND_QUEUED_TRANSACTION_LIST_PAGE_CONFLICT_394,
    )
    .unwrap()
    .results;

    let bat_token_info = serde_json::from_str::<TokenInfo>(TOKEN_BAT).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(
        r#"{
              "address": "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
              "nonce": 393,
              "threshold": 3,
              "owners": [
                "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0",
                "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164",
                "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd"
              ],
              "masterCopy": "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F",
              "modules": [],
              "fallbackHandler": "0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44",
              "version": "1.1.1"
            }"#,
    )
    .unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(3)
        .returning(move |_| Ok(safe_info.clone()));
    mock_info_provider
        .expect_token_info()
        .times(3)
        .returning(move |_| Ok(bat_token_info.clone()));
    mock_info_provider
        .expect_full_address_info_search()
        .times(3)
        .returning(move |_| bail!("No address info"));

    let mut tx_iter = input_list.into_iter();

    let safe_nonce = 392;
    let previous_page_nonce = 393;
    let edge_nonce = 394;

    let actual = process_transactions(
        &mut mock_info_provider,
        safe_nonce,
        &mut tx_iter,
        previous_page_nonce,
        edge_nonce,
    );

    //The first item expected is just a a transaction because we are not in the first page of data
    let expected: Vec<TransactionListItem> = vec![
        TransactionListItem::Transaction {
            transaction: TransactionSummary {
                id: "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x0fe072e76498e0db46fc79113662026a4f8fb34e840491aefeff6dec21c766cb".to_string(),
                timestamp: 1607602242476,
                tx_status: TransactionStatus::AwaitingConfirmations,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                    sender_info: None,
                    recipient: "0xf2565317F3Ae8Ae9EA98E9Fe1e7FADC77F823cbD".to_string(),
                    recipient_info: None,
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(Erc20Transfer {
                        token_address: "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46".to_string(),
                        token_name: Some("BigAmount".to_string()),
                        token_symbol: Some("BA-T".to_string()),
                        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46.png".to_string()),
                        decimals: Some(1),
                        value: "10".to_string()
                    })
                }),
                execution_info: Some(ExecutionInfo{
                    nonce: 393,
                    confirmations_required: 3,
                    confirmations_submitted:1,
                    missing_signers: Some(vec!["0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(), "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164".to_string()])
                }),
                safe_app_info: None,
            },
            conflict_type: ConflictType::End,
        },
        TransactionListItem::ConflictHeader {
            nonce: 394
        },
        TransactionListItem::Transaction {
            transaction: TransactionSummary {
                id: "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x2e4af4b451a493470f38625c5f78f710f02303eb32780896cb55357c00d48faa".to_string(),
                timestamp: 1607602284354,
                tx_status: TransactionStatus::AwaitingConfirmations,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                    sender_info: None,
                    recipient: "0xF353eBBa77e5E71c210599236686D51cA1F88b84".to_string(),
                    recipient_info: None,
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(Erc20Transfer {
                        token_address: "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46".to_string(),
                        token_name: Some("BigAmount".to_string()),
                        token_symbol: Some("BA-T".to_string()),
                        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46.png".to_string()),
                        decimals: Some(1),
                        value: "20".to_string()
                    })
                }),
                execution_info: Some(ExecutionInfo{
                    nonce: 394,
                    confirmations_required: 3,
                    confirmations_submitted:1,
                    missing_signers: Some(vec!["0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(), "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164".to_string()])
                }),
                safe_app_info: None,
            },
            conflict_type: ConflictType::HasNext,
        },
        TransactionListItem::Transaction {
            transaction: TransactionSummary {
                id: "multisig_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0xca7a464a3479af396c2975b4b3f5f7b90fc56747404ebaad5ec838c2954d2f9c".to_string(),
                timestamp: 1607602424072,
                tx_status: TransactionStatus::AwaitingConfirmations,
                tx_info: TransactionInfo::Transfer(Transfer {
                    sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
                    sender_info: None,
                    recipient: "0xF353eBBa77e5E71c210599236686D51cA1F88b84".to_string(),
                    recipient_info: None,
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(Erc20Transfer {
                        token_address: "0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46".to_string(),
                        token_name: Some("BigAmount".to_string()),
                        token_symbol: Some("BA-T".to_string()),
                        logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD81F7D71ed570D121A1Ef9e3Bc0fc2bd6192De46.png".to_string()),
                        decimals: Some(1),
                        value: "20".to_string()
                    })
                }),
                execution_info: Some(ExecutionInfo{
                    nonce: 394,
                    confirmations_required: 3,
                    confirmations_submitted:1,
                    missing_signers: Some(vec!["0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(), "0x8bc9Ab35a2A8b20ad8c23410C61db69F2e5d8164".to_string()])
                }),
                safe_app_info: None,
            },
            conflict_type: ConflictType::HasNext,
        }
    ];

    assert_eq!(expected, actual);
}

fn get_multisig_tx(source: &str) -> MultisigTransaction {
    serde_json::from_str::<MultisigTransaction>(source).unwrap()
}
