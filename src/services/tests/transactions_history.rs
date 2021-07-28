use crate::json::BACKEND_HISTORY_TRANSACTION_LIST_PAGE;
use crate::models::backend::transactions::Transaction;
use crate::models::commons::{Page, PageMetadata};
use crate::models::service::addresses::AddressEx;
use crate::models::service::transactions::summary::{
    ConflictType, ExecutionInfo, ModuleExecutionInfo, TransactionListItem, TransactionSummary,
};
use crate::models::service::transactions::TransactionStatus::Success;
use crate::models::service::transactions::TransferDirection::{Incoming, Outgoing};
use crate::models::service::transactions::{Custom, TransactionInfo, Transfer};
use crate::models::service::transactions::{Erc20Transfer, TransferInfo};
use crate::providers::info::*;
use crate::services::transactions_history::{
    adjust_page_meta, backend_txs_to_summary_txs, get_day_timestamp_millis,
    peek_timestamp_and_remove_item, service_txs_to_tx_list_items,
};

#[test]
fn adjust_page_meta_offset_0() {
    let input = PageMetadata {
        offset: 0,
        limit: 50,
    };
    let expected = PageMetadata {
        offset: 0,
        limit: input.limit,
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
        limit: 51,
    };

    let actual = adjust_page_meta(&input);

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn backend_txs_to_summary_txs_empty() {
    let backend_txs = Page {
        next: None,
        previous: None,
        results: vec![],
    };
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);

    let mut back_end_txs_iter = backend_txs.results.into_iter();

    let actual = backend_txs_to_summary_txs(&mut back_end_txs_iter, &mut mock_info_provider, "")
        .await
        .unwrap();
    assert_eq!(actual.is_empty(), true);
}

#[rocket::async_test]
async fn backend_txs_to_summary_txs_with_values() {
    let backend_txs =
        serde_json::from_str::<Page<Transaction>>(BACKEND_HISTORY_TRANSACTION_LIST_PAGE).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(3)
        .returning(move |_| bail!("No contract info"));
    mock_info_provider
        .expect_token_info()
        .times(3)
        .returning(move |_| bail!("No token info"));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(6)
        .returning(move |_| bail!("No address info"));

    let mut back_end_txs_iter = backend_txs.results.into_iter();
    let expected = vec![
        TransactionSummary {
            id: "module_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0xcd10b23687bf336d0f4c0a3383590d3d1722aaa99a41fd0d289a5f69a8266c8f_0x53b6e88b578a6313".into(),
            timestamp: 1606845854000,
            tx_status: Success,
            tx_info: TransactionInfo::Custom(
                Custom {
                    to: AddressEx::address_only("0xc778417E063141139Fce010982780140Aa0cD5Ab"),
                    data_size: "68".into(),
                    value: "0".into(),
                    method_name: Some("transfer".into()),
                    action_count: None,
                    is_cancellation: false,
                },
            ),
            execution_info: Some(ExecutionInfo::Module(ModuleExecutionInfo {
                address: AddressEx::address_only("0xCFbFaC74C26F8647cBDb8c5caf80BB5b32E43134"),
            })),
            safe_app_info: None,
        },
        TransactionSummary {
            id: "module_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x1cf24abdb39bb7b156677a128e709cea55c6991b12708904d1f0f3664ad6646e_0x2e5157f6f782e36f".into(),
            timestamp: 1606845794000,
            tx_status: Success,
            tx_info: TransactionInfo::Custom(
                Custom {
                    to: AddressEx::address_only("0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02"),
                    data_size: "68".into(),
                    value: "0".into(),
                    method_name: Some("transfer".into()),
                    action_count: None,
                    is_cancellation: false,
                },
            ),
            execution_info: Some(ExecutionInfo::Module(ModuleExecutionInfo {
                address: AddressEx::address_only("0xCFbFaC74C26F8647cBDb8c5caf80BB5b32E43134"),
            })),
            safe_app_info: None,
        },
        TransactionSummary {
            id: "module_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x3f12bb74cd91ef09d553f66e3623bceaf879ba3dcb325227b1fbf2455757891a_0x15a0e5a089475db".into(),
            timestamp: 1606845070000,
            tx_status: Success,
            tx_info: TransactionInfo::Custom(
                Custom {
                    to: AddressEx::address_only("0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02"),
                    data_size: "68".into(),
                    value: "0".into(),
                    method_name: Some("transfer".into()),
                    action_count: None,
                    is_cancellation: false,
                },
            ),
            execution_info: Some(ExecutionInfo::Module(ModuleExecutionInfo {
                address: AddressEx::address_only("0xCFbFaC74C26F8647cBDb8c5caf80BB5b32E43134"),
            })),
            safe_app_info: None,
        },
        TransactionSummary {
            id: "ethereum_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x021d4d8cb68f3f772906b58f97b66c6ead228c252627c5b1aff4b496d4ff0c2d_0xfd0dbbc7700a140f".into(),
            timestamp: 1606744033000,
            tx_status: Success,
            tx_info: TransactionInfo::Transfer(
                Transfer {
                    sender: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
                    recipient: AddressEx::address_only("0xF353eBBa77e5E71c210599236686D51cA1F88b84"),
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(
                        Erc20Transfer {
                            token_address: "0x63704B63Ac04f3a173Dfe677C7e3D330c347CD88".into(),
                            token_name: Some(
                                "TEST AQER".into(),
                            ),
                            token_symbol: Some(
                                "AQER".into(),
                            ),
                            logo_uri: Some(
                                "https://gnosis-safe-token-logos.s3.amazonaws.com/0x63704B63Ac04f3a173Dfe677C7e3D330c347CD88.png".into(),
                            ),
                            decimals: Some(
                                18,
                            ),
                            value: "100000000000000000".into(),
                        },
                    ),
                },
            ),
            execution_info: None,
            safe_app_info: None,
        },
        TransactionSummary {
            id: "ethereum_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0x5f4b7555f8e977ae302ab4125de685ccfacf52ac70e6f0aa2939bcb347f9a732_0xb7ceaac0cd5a85c5".into(),
            timestamp: 1606743581000,
            tx_status: Success,
            tx_info: TransactionInfo::Transfer(
                Transfer {
                    sender: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
                    recipient: AddressEx::address_only("0xf2565317F3Ae8Ae9EA98E9Fe1e7FADC77F823cbD"),
                    direction: Outgoing,
                    transfer_info: TransferInfo::Erc20(
                        Erc20Transfer {
                            token_address: "0x63704B63Ac04f3a173Dfe677C7e3D330c347CD88".into(),
                            token_name: Some(
                                "TEST AQER".into(),
                            ),
                            token_symbol: Some(
                                "AQER".into(),
                            ),
                            logo_uri: Some(
                                "https://gnosis-safe-token-logos.s3.amazonaws.com/0x63704B63Ac04f3a173Dfe677C7e3D330c347CD88.png".into(),
                            ),
                            decimals: Some(18),
                            value: "100000000000000000".into(),
                        },
                    ),
                },
            ),
            execution_info: None,
            safe_app_info: None,
        },
        TransactionSummary {
            id: "ethereum_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_0xaafed95936f9d71eb8d9612e83f3f93f9decf33f11bbb4aa79cae98966ffa7fe_0x11bd3d64559a0af7".into(),
            timestamp: 1606739725000,
            tx_status: Success,
            tx_info: TransactionInfo::Transfer(
                Transfer {
                    sender: AddressEx::address_only("0xf2565317F3Ae8Ae9EA98E9Fe1e7FADC77F823cbD"),
                    recipient: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
                    direction: Incoming,
                    transfer_info: TransferInfo::Erc20(
                        Erc20Transfer {
                            token_address: "0x81D0FF4fE216fB6aC98ED609086A92d94dbfE666".into(),
                            token_name: Some(
                                "LS".into(),
                            ),
                            token_symbol: Some(
                                "LS".into(),
                            ),
                            logo_uri: Some(
                                "https://gnosis-safe-token-logos.s3.amazonaws.com/0x81D0FF4fE216fB6aC98ED609086A92d94dbfE666.png".into(),
                            ),
                            decimals: Some(
                                18,
                            ),
                            value: "400000000000000".into(),
                        },
                    ),
                },
            ),
            execution_info: None,
            safe_app_info: None,
        },
    ];
    let actual = backend_txs_to_summary_txs(
        &mut back_end_txs_iter,
        &mut mock_info_provider,
        "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
    )
    .await
    .unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn service_txs_to_tx_list_items_empty() {
    let service_tx: Vec<TransactionSummary> = vec![];
    let utc_timezone_offset = 0;

    let actual = service_txs_to_tx_list_items(service_tx, -1, utc_timezone_offset).unwrap();

    assert_eq!(actual.is_empty(), true);
}

#[rocket::async_test]
async fn service_txs_to_tx_list_items_last_timestamp_undefined() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(6)
        .returning(move |_| bail!("No contract info"));
    mock_info_provider
        .expect_token_info()
        .times(6)
        .returning(move |_| bail!("No token info"));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(12)
        .returning(move |_| bail!("No address info"));

    let service_txs = get_service_txs(&mut mock_info_provider).await;
    let service_txs_copy = get_service_txs(&mut mock_info_provider).await;
    let utc_timezone_offset = 0;

    let mut service_txs_inter = service_txs.into_iter();

    let expected = vec![
        TransactionListItem::DateLabel {
            timestamp: 1606780800000,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::DateLabel {
            timestamp: 1606694400000,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
    ];

    let actual = service_txs_to_tx_list_items(service_txs_copy, -1, utc_timezone_offset).unwrap();
    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn service_txs_to_tx_list_items_last_timestamp_defined_but_different() {
    let last_timestamp = 1606867200000;
    let utc_timezone_offset = 0;

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(6)
        .returning(move |_| bail!("No contract info"));
    mock_info_provider
        .expect_token_info()
        .times(6)
        .returning(move |_| bail!("No token info"));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(12)
        .returning(move |_| bail!("No address info"));

    let service_txs = get_service_txs(&mut mock_info_provider).await;
    let service_txs_copy = get_service_txs(&mut mock_info_provider).await;

    let mut service_txs_inter = service_txs.into_iter();

    let expected = vec![
        TransactionListItem::DateLabel {
            timestamp: 1606780800000,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::DateLabel {
            timestamp: 1606694400000,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
    ];

    let actual =
        service_txs_to_tx_list_items(service_txs_copy, last_timestamp, utc_timezone_offset)
            .unwrap();
    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn service_txs_to_tx_list_items_last_timestamp_defined_and_same() {
    let last_timestamp = 1606780800000;
    let utc_timezone_offset = 0;

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(6)
        .returning(move |_| bail!("No contract info"));
    mock_info_provider
        .expect_token_info()
        .times(6)
        .returning(move |_| bail!("No token info"));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(12)
        .returning(move |_| bail!("No address info"));

    let service_txs = get_service_txs(&mut mock_info_provider).await;
    let service_txs_copy = get_service_txs(&mut mock_info_provider).await;

    let mut service_txs_inter = service_txs.into_iter();

    let expected = vec![
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::DateLabel {
            timestamp: 1606694400000,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
    ];

    let actual =
        service_txs_to_tx_list_items(service_txs_copy, last_timestamp, utc_timezone_offset)
            .unwrap();
    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn service_txs_to_tx_list_items_date_label_berlin_timezone() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(6)
        .returning(move |_| bail!("No contract info"));
    mock_info_provider
        .expect_token_info()
        .times(6)
        .returning(move |_| bail!("No token info"));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(12)
        .returning(move |_| bail!("No address info"));

    let service_txs = get_service_txs(&mut mock_info_provider).await;
    let service_txs_copy = get_service_txs(&mut mock_info_provider).await;
    let berlin_timezone_offset = 3600; // + 1 hours Germany/Berlin

    let mut service_txs_inter = service_txs.into_iter();

    let expected = vec![
        TransactionListItem::DateLabel {
            timestamp: 1606777200000,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::DateLabel {
            timestamp: 1606690800000,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
    ];

    let actual =
        service_txs_to_tx_list_items(service_txs_copy, -1, berlin_timezone_offset).unwrap();
    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn service_txs_to_tx_list_items_date_label_melbourne_timezone() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(6)
        .returning(move |_| bail!("No contract info"));
    mock_info_provider
        .expect_token_info()
        .times(6)
        .returning(move |_| bail!("No token info"));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(12)
        .returning(move |_| bail!("No address info"));

    let service_txs = get_service_txs(&mut mock_info_provider).await;
    let service_txs_copy = get_service_txs(&mut mock_info_provider).await;
    let melbourne_timezone_offset = 39600; // + 11 hours Melbourne/Australia

    let mut service_txs_inter = service_txs.into_iter();

    let expected = vec![
        TransactionListItem::DateLabel {
            timestamp: 1606827600000, // 2020/12/02
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::DateLabel {
            timestamp: 1606741200000, // 2020/12/01
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::DateLabel {
            timestamp: 1606654800000, // 2020/11/30
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
    ];

    let actual =
        service_txs_to_tx_list_items(service_txs_copy, -1, melbourne_timezone_offset).unwrap();

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn service_txs_to_tx_list_items_date_label_buenos_aires_timezone() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(6)
        .returning(move |_| bail!("No contract info"));
    mock_info_provider
        .expect_token_info()
        .times(6)
        .returning(move |_| bail!("No token info"));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(12)
        .returning(move |_| bail!("No address info"));

    let service_txs = get_service_txs(&mut mock_info_provider).await;
    let service_txs_copy = get_service_txs(&mut mock_info_provider).await;
    let buenos_aires_timezone_offset = -10800; // -3 hours Argentina/Buenos Aires

    let mut service_txs_inter = service_txs.into_iter();

    let expected = vec![
        TransactionListItem::DateLabel {
            timestamp: 1606791600000,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::DateLabel {
            timestamp: 1606705200000,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
        TransactionListItem::Transaction {
            transaction: service_txs_inter.next().unwrap(),
            conflict_type: ConflictType::None,
        },
    ];

    let actual =
        service_txs_to_tx_list_items(service_txs_copy, -1, buenos_aires_timezone_offset).unwrap();

    assert_eq!(expected, actual);
}

#[rocket::async_test]
#[should_panic]
async fn peek_timestamp_and_remove_item_empty() {
    let utc_timezone_offset = 3600;
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);

    let backend_txs: Vec<Transaction> = vec![];
    let mut backend_txs_iter = backend_txs.into_iter();

    peek_timestamp_and_remove_item(
        &mut backend_txs_iter,
        &mut mock_info_provider,
        "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
        utc_timezone_offset,
    )
    .await
    .unwrap();
}

#[rocket::async_test]
async fn peek_timestamp_and_remove_item_with_items() {
    let expected_timestamp = 1606780800000;
    let utc_timezone_offset = 0;

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(1)
        .returning(move |_| bail!("No contract info"));
    mock_info_provider
        .expect_token_info()
        .times(1)
        .returning(move |_| bail!("No token info"));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let backend_txs =
        serde_json::from_str::<Page<Transaction>>(BACKEND_HISTORY_TRANSACTION_LIST_PAGE)
            .unwrap()
            .results;
    let mut backend_txs_iter = backend_txs.into_iter();

    let actual_timestamp = peek_timestamp_and_remove_item(
        &mut backend_txs_iter,
        &mut mock_info_provider,
        "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
        utc_timezone_offset,
    )
    .await
    .unwrap();

    assert_eq!(expected_timestamp, actual_timestamp);
}

#[test]
fn get_day_timestamp_millis_for_02_12_2020_00_00_01() {
    let input = 1606867201000; // 1 second past the 2nd of December 2020 UTC
    let utc_timezone_offset = 0;

    let actual = get_day_timestamp_millis(input, utc_timezone_offset);
    let expected = 1606867200000;

    assert_eq!(expected, actual);
}

async fn get_service_txs(mock_info_provider: &mut MockInfoProvider) -> Vec<TransactionSummary> {
    let backend_txs =
        serde_json::from_str::<Page<Transaction>>(BACKEND_HISTORY_TRANSACTION_LIST_PAGE).unwrap();

    let mut result = vec![];

    for tx in backend_txs.results {
        result.extend(
            tx.to_transaction_summary(
                mock_info_provider,
                "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
            )
            .await
            .unwrap_or_default(),
        )
    }

    result
}
