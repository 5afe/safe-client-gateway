use crate::common::models::addresses::AddressEx;
use crate::common::models::backend::transactions::{
    CreationTransaction, EthereumTransaction, ModuleTransaction, MultisigTransaction,
    SafeTransaction, Transaction as TransactionDto,
};
use crate::common::models::backend::transfers::{
    EtherTransfer as EtherTransferDto, Transfer as TransferDto,
};
use crate::common::models::data_decoded::ParamValue::SingleValue;
use crate::common::models::data_decoded::{DataDecoded, Operation, Parameter};
use crate::providers::info::*;
use crate::routes::transactions::converters::data_size;
use crate::routes::transactions::models::summary::{
    ExecutionInfo, ModuleExecutionInfo, MultisigExecutionInfo, TransactionSummary,
};
use crate::routes::transactions::models::{
    Creation, Custom, Erc20Transfer, Erc721Transfer, NativeCoinTransfer, SettingsChange,
    SettingsInfo, TransactionInfo, TransactionStatus, Transfer, TransferDirection, TransferInfo,
    ID_PREFIX_CREATION_TX, ID_PREFIX_ETHEREUM_TX, ID_PREFIX_MODULE_TX, ID_PREFIX_MULTISIG_TX,
};
use crate::utils::hex_hash;
use chrono::Utc;

#[test]
fn data_size_calculation() {
    assert_eq!(data_size(&None), 0);
    assert_eq!(data_size(&Some(String::from(""))), 0);
    assert_eq!(data_size(&Some(String::from("0x"))), 0);
    assert_eq!(
        data_size(&Some(String::from("0x8d80ff0a000000000000000000000000000000000000000000000000000000000000002000000000000000000000000000000000000000000000000000000000000000f2001230b3d59858296a31053c1b8562ecf89a2f888b000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000247de7edef00000000000000000000000034cfac646f301356faa8b21e94227e3583fe3f5f001230b3d59858296a31053c1b8562ecf89a2f888b00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000024f08a0323000000000000000000000000d5d82b6addc9027b22dca772aa68d5d74cdbdf440000000000000000000000000000"))),
        324
    );
}

#[rocket::async_test]
async fn unknown_tx_to_summary_transaction() {
    let unknown_tx = TransactionDto::Unknown;
    let mut mock_info_provider = MockInfoProvider::new();

    let error = unknown_tx
        .to_transaction_summary(&mut mock_info_provider, &String::from(""))
        .await;

    assert!(error.is_err());
}

#[rocket::async_test]
async fn module_tx_to_summary_transaction_success() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(1)
        .returning(move |_| bail!("No contract info"));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .returning(move |_| bail!("No address info"));

    let expected_to = String::from("0x12345789");
    let expected_date = Utc::now();
    let expected_date_in_millis = expected_date.timestamp_millis();
    let module_tx = ModuleTransaction {
        safe_transaction: SafeTransaction {
            safe: String::from("safe"),
            to: expected_to.clone(),
            value: None,
            data: None,
            data_decoded: None,
            operation: Operation::CALL,
        },
        created: String::from("created"),
        execution_date: expected_date,
        block_number: 0,
        is_successful: true,
        transaction_hash: String::from("tx_hash"),
        module: String::from("module"),
    };

    let actual =
        ModuleTransaction::to_transaction_summary(&module_tx, &mut mock_info_provider).await;
    let expected = vec![TransactionSummary {
        id: create_id!(
            ID_PREFIX_MODULE_TX,
            module_tx.safe_transaction.safe,
            module_tx.transaction_hash,
            hex_hash(&module_tx)
        ),
        timestamp: expected_date_in_millis,
        tx_status: TransactionStatus::Success,
        execution_info: Some(ExecutionInfo::Module(ModuleExecutionInfo {
            address: AddressEx::address_only("module"),
        })),
        tx_info: TransactionInfo::Custom(Custom {
            to: AddressEx::address_only(&expected_to),
            data_size: String::from("0"),
            value: String::from("0"),
            method_name: None,
            action_count: None,
            is_cancellation: false,
        }),
        safe_app_info: None,
    }];
    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn module_tx_to_summary_transaction_failed() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(1)
        .returning(move |_| bail!("No contract info"));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .returning(move |_| bail!("No address info"));

    let expected_to = String::from("0x12345789");
    let expected_date = Utc::now();
    let expected_date_in_millis = expected_date.timestamp_millis();
    let module_tx = ModuleTransaction {
        safe_transaction: SafeTransaction {
            safe: String::from("safe"),
            to: expected_to.clone(),
            value: None,
            data: None,
            data_decoded: None,
            operation: Operation::CALL,
        },
        created: String::from("created"),
        execution_date: expected_date,
        block_number: 0,
        is_successful: false,
        transaction_hash: String::from("tx_hash"),
        module: String::from("module"),
    };

    let actual =
        ModuleTransaction::to_transaction_summary(&module_tx, &mut mock_info_provider).await;
    let expected = vec![TransactionSummary {
        id: create_id!(
            ID_PREFIX_MODULE_TX,
            module_tx.safe_transaction.safe,
            module_tx.transaction_hash,
            hex_hash(&module_tx)
        ),
        timestamp: expected_date_in_millis,
        tx_status: TransactionStatus::Failed,
        execution_info: Some(ExecutionInfo::Module(ModuleExecutionInfo {
            address: AddressEx::address_only("module"),
        })),
        tx_info: TransactionInfo::Custom(Custom {
            to: AddressEx::address_only(&expected_to),
            data_size: String::from("0"),
            value: String::from("0"),
            method_name: None,
            action_count: None,
            is_cancellation: false,
        }),
        safe_app_info: None,
    }];
    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn module_transaction_to_custom_summary_and_module_info() {
    let module_tx =
        serde_json::from_str::<ModuleTransaction>(crate::tests::json::MODULE_TX).unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(1)
        .return_once(move |address| {
            Ok(AddressEx {
                value: address.to_string(),
                name: Some(format!("{}_name", address)),
                logo_uri: None,
            })
        });
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let expected = TransactionSummary {
        id: create_id!(
            ID_PREFIX_MODULE_TX,
            module_tx.safe_transaction.safe,
            module_tx.transaction_hash,
            hex_hash(&module_tx)
        ),
        timestamp: module_tx.execution_date.timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Custom(Custom {
            to: AddressEx::address_only("0xaAEb2035FF394fdB2C879190f95e7676f1A9444B"),
            data_size: "132".to_string(),
            value: "0".to_string(),
            method_name: None,
            action_count: None,
            is_cancellation: false,
        }),
        execution_info: Some(ExecutionInfo::Module(ModuleExecutionInfo {
            address: AddressEx {
                value: "0xfa559f0932b7B60d90B4af0b8813d4088465096b".to_string(),
                name: Some("0xfa559f0932b7B60d90B4af0b8813d4088465096b_name".to_string()),
                logo_uri: None,
            },
        })),
        safe_app_info: None,
    };

    let actual =
        ModuleTransaction::to_transaction_summary(&module_tx, &mut mock_info_provider).await;

    assert_eq!(&expected, actual.get(0).unwrap());
}

#[rocket::async_test]
async fn ethereum_tx_to_summary_transaction_no_transfers() {
    let safe_address = String::from("0x2323");
    let mut mock_info_provider = MockInfoProvider::new();

    let ethereum_tx = EthereumTransaction {
        execution_date: Utc::now(),
        data: None,
        tx_hash: String::from("0x4321"),
        block_number: 0,
        transfers: None,
        from: String::from("0x6789"),
    };

    let actual = EthereumTransaction::to_transaction_summary(
        &ethereum_tx,
        &mut mock_info_provider,
        &safe_address,
    )
    .await;
    assert_eq!(actual, Vec::new());
}

#[rocket::async_test]
async fn ethereum_tx_to_summary_transaction_with_transfers() {
    let safe_address = String::from("0x2323");
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(4)
        .returning(move |_| bail!("No address info"));
    let timestamp = Utc::now();
    let timestamp_millis = timestamp.timestamp_millis();

    let transfers = vec![
        TransferDto::Ether(EtherTransferDto {
            execution_date: timestamp,
            block_number: 0,
            transaction_hash: "0x4321".to_string(),
            to: "".to_string(),
            value: String::from("1"),
            from: "".to_string(),
        }),
        TransferDto::Ether(EtherTransferDto {
            execution_date: timestamp,
            block_number: 0,
            transaction_hash: "0x4321".to_string(),
            to: "".to_string(),
            value: String::from("1"),
            from: "".to_string(),
        }),
    ];
    let ethereum_tx = EthereumTransaction {
        execution_date: timestamp,
        data: None,
        tx_hash: String::from("0x4321"),
        block_number: 0,
        transfers: Some(transfers.to_vec()),
        from: String::from("0x6789"),
    };

    let actual = EthereumTransaction::to_transaction_summary(
        &ethereum_tx,
        &mut mock_info_provider,
        &safe_address,
    )
    .await;
    let expected = vec![
        TransactionSummary {
            id: create_id!(
                ID_PREFIX_ETHEREUM_TX,
                safe_address,
                ethereum_tx.tx_hash,
                hex_hash(&ethereum_tx.transfers.as_ref().unwrap().get(0).unwrap())
            ),
            timestamp: timestamp_millis,
            tx_status: TransactionStatus::Success,
            tx_info: TransactionInfo::Transfer(Transfer {
                sender: AddressEx::address_only(""),
                recipient: AddressEx::address_only(""),
                direction: TransferDirection::Unknown,
                transfer_info: TransferInfo::NativeCoin(NativeCoinTransfer {
                    value: "1".to_string(),
                }),
            }),
            execution_info: None,
            safe_app_info: None,
        },
        TransactionSummary {
            id: create_id!(
                ID_PREFIX_ETHEREUM_TX,
                safe_address,
                ethereum_tx.tx_hash,
                hex_hash(&ethereum_tx.transfers.as_ref().unwrap().get(1).unwrap())
            ),
            timestamp: timestamp_millis,
            tx_status: TransactionStatus::Success,
            tx_info: TransactionInfo::Transfer(Transfer {
                sender: AddressEx::address_only(""),
                recipient: AddressEx::address_only(""),
                direction: TransferDirection::Unknown,
                transfer_info: TransferInfo::NativeCoin(NativeCoinTransfer {
                    value: "1".to_string(),
                }),
            }),
            execution_info: None,
            safe_app_info: None,
        },
    ];
    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn creation_transaction_to_summary_no_address_info_available() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(3)
        .returning(move |_| bail!("No address info"));

    let created_date = Utc::now();
    let safe_address = String::from("0x38497");
    let creator = String::from("0x123");
    let transaction_hash = String::from("0x2232");
    let factory_address = String::from("0x123");
    let master_copy = String::from("0x987");
    let creation_tx = CreationTransaction {
        created: created_date,
        creator: creator.clone(),
        transaction_hash: transaction_hash.clone(),
        factory_address: Some(factory_address.clone()),
        master_copy: Some(master_copy.clone()),
        setup_data: None,
        data_decoded: None,
    };
    let expected = TransactionSummary {
        id: create_id!(ID_PREFIX_CREATION_TX, safe_address),
        timestamp: created_date.timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Creation(Creation {
            creator: AddressEx::address_only(&creator),
            transaction_hash,
            implementation: Some(AddressEx::address_only(&master_copy)),
            factory: Some(AddressEx::address_only(&factory_address)),
        }),
        execution_info: None,
        safe_app_info: None,
    };

    let actual = creation_tx
        .to_transaction_summary(&safe_address, &mut mock_info_provider)
        .await;

    assert_eq!(expected, actual);
}

// TODO test with addresses returned
#[rocket::async_test]
async fn creation_transaction_to_summary_address_info_available() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(3)
        .returning(move |address| {
            Ok(AddressEx {
                value: address.to_string(),
                name: Some("".to_string()),
                logo_uri: None,
            })
        });

    let created_date = Utc::now();
    let safe_address = String::from("0x38497");
    let creator = String::from("0x123");
    let transaction_hash = String::from("0x2232");
    let factory_address = String::from("0x123");
    let master_copy = String::from("0x987");
    let creation_tx = CreationTransaction {
        created: created_date,
        creator: creator.clone(),
        transaction_hash: transaction_hash.clone(),
        factory_address: Some(factory_address.clone()),
        master_copy: Some(master_copy.clone()),
        setup_data: None,
        data_decoded: None,
    };
    let expected = TransactionSummary {
        id: create_id!(ID_PREFIX_CREATION_TX, safe_address),
        timestamp: created_date.timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Creation(Creation {
            creator: AddressEx {
                value: creator,
                name: Some("".to_string()),
                logo_uri: None,
            },
            transaction_hash,
            implementation: Some(AddressEx {
                value: master_copy,
                name: Some("".to_string()),
                logo_uri: None,
            }),
            factory: Some(AddressEx {
                value: factory_address,
                name: Some("".to_string()),
                logo_uri: None,
            }),
        }),
        execution_info: None,
        safe_app_info: None,
    };

    let actual = creation_tx
        .to_transaction_summary(&safe_address, &mut mock_info_provider)
        .await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn multisig_transaction_to_erc20_transfer_summary() {
    let multisig_tx =
        serde_json::from_str::<MultisigTransaction>(crate::tests::json::MULTISIG_TX_ERC20_TRANSFER)
            .unwrap();
    let safe_info =
        serde_json::from_str::<SafeInfo>(crate::tests::json::SAFE_WITH_MODULES).unwrap();
    let token_info = serde_json::from_str::<TokenInfo>(crate::tests::json::TOKEN_USDT).unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let expected = TransactionSummary {
        id: create_id!(ID_PREFIX_MULTISIG_TX, "0x1230B3d59858296A31053C1b8562Ecf89A2f888b", "0x95e32bb8cb88ecdc45732c0a551eae7b3744187cf1ba19cda1440eaaf7b4950c"),
        timestamp: multisig_tx.execution_date.unwrap().timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Transfer(Transfer {
            sender: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
            recipient: AddressEx::address_only("0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"),
            direction: TransferDirection::Outgoing,
            transfer_info: TransferInfo::Erc20(Erc20Transfer {
                token_address: "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02".to_string(),
                token_name: Some("Compound USDT".to_string()),
                token_symbol: Some("USDT".to_string()),
                logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02.png".to_string()),
                decimals: Some(18),
                value: "50000000000000".to_string(),
            }),
        }),
        execution_info: Some(ExecutionInfo::Multisig(MultisigExecutionInfo {
            nonce: 178,
            confirmations_required: 3,
            confirmations_submitted: 3,
            missing_signers: None,
        })),
        safe_app_info: None,
    };

    let actual =
        MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider).await;

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}

#[rocket::async_test]
async fn multisig_transaction_to_erc721_transfer_summary() {
    let multisig_tx = serde_json::from_str::<MultisigTransaction>(
        crate::tests::json::MULTISIG_TX_ERC721_TRANSFER,
    )
    .unwrap();
    let safe_info =
        serde_json::from_str::<SafeInfo>(crate::tests::json::SAFE_WITH_MODULES).unwrap();
    let token_info =
        serde_json::from_str::<TokenInfo>(crate::tests::json::TOKEN_CRYPTO_KITTIES).unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let expected = TransactionSummary {
        id: create_id!(ID_PREFIX_MULTISIG_TX, "0x1230B3d59858296A31053C1b8562Ecf89A2f888b", "0x9155f7741dd33572bc49c251eb4f4a5e9cf9653151417bdc4a2aca0767779603"),
        timestamp: multisig_tx.execution_date.unwrap().timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Transfer(Transfer {
            sender: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
            recipient: AddressEx::address_only("0x938bae50a210b80EA233112800Cd5Bc2e7644300"),
            direction: TransferDirection::Outgoing,
            transfer_info: TransferInfo::Erc721(Erc721Transfer {
                token_address: "0x16baF0dE678E52367adC69fD067E5eDd1D33e3bF".to_string(),
                token_id: "1316".to_string(),
                token_name: Some("CryptoKitties".to_string()),
                token_symbol: Some("CK".to_string()),
                logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x16baF0dE678E52367adC69fD067E5eDd1D33e3bF.png".to_string()),
            }),
        }),
        execution_info: Some(ExecutionInfo::Multisig(MultisigExecutionInfo {
            nonce: 177,
            confirmations_required: 3,
            confirmations_submitted: 3,
            missing_signers: None,
        })),
        safe_app_info: None,
    };

    let actual =
        MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider).await;

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}

#[rocket::async_test]
async fn multisig_transaction_to_ether_transfer_summary() {
    let multisig_tx =
        serde_json::from_str::<MultisigTransaction>(crate::tests::json::MULTISIG_TX_ETHER_TRANSFER)
            .unwrap();
    let safe_info =
        serde_json::from_str::<SafeInfo>(crate::tests::json::SAFE_WITH_MODULES).unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let expected = TransactionSummary {
        id: create_id!(
            ID_PREFIX_MULTISIG_TX,
            "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
            "0x6e631d27c638458329ba95cc17961e74b8146c46886545cd1984bb2bcf4eccd3"
        ),
        timestamp: multisig_tx.execution_date.unwrap().timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Transfer(Transfer {
            sender: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
            recipient: AddressEx::address_only("0x938bae50a210b80EA233112800Cd5Bc2e7644300"),
            direction: TransferDirection::Outgoing,
            transfer_info: TransferInfo::NativeCoin(NativeCoinTransfer {
                value: "100000000000000000".to_string(),
            }),
        }),
        execution_info: Some(ExecutionInfo::Multisig(MultisigExecutionInfo {
            nonce: 147,
            confirmations_required: 2,
            confirmations_submitted: 2,
            missing_signers: None,
        })),
        safe_app_info: None,
    };

    let actual =
        MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider).await;

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}

#[rocket::async_test]
async fn multisig_transaction_to_settings_change_summary() {
    let multisig_tx = serde_json::from_str::<MultisigTransaction>(
        crate::tests::json::MULTISIG_TX_SETTINGS_CHANGE,
    )
    .unwrap();
    let safe_info =
        serde_json::from_str::<SafeInfo>(crate::tests::json::SAFE_WITH_MODULES).unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider
        .expect_address_ex_from_contracts()
        .times(0);
    mock_info_provider.expect_token_info().times(0);

    let expected = TransactionSummary {
        id: create_id!(
            ID_PREFIX_MULTISIG_TX,
            "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
            "0x57d94fe21bbee8f6646c420ee23126cd1ba1b9a53a6c9b10099a043da8f32eea"
        ),
        timestamp: multisig_tx.execution_date.unwrap().timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::SettingsChange(SettingsChange {
            settings_info: Some(SettingsInfo::AddOwner {
                owner: AddressEx::address_only("0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D"),
                threshold: 2,
            }),
            data_decoded: DataDecoded {
                method: "addOwnerWithThreshold".to_string(),
                parameters: Some(vec![
                    Parameter {
                        name: "owner".to_string(),
                        param_type: "address".to_string(),
                        value: SingleValue(
                            "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_string(),
                        ),
                        value_decoded: None,
                    },
                    Parameter {
                        name: "_threshold".to_string(),
                        param_type: "uint256".to_string(),
                        value: SingleValue("2".to_string()),
                        value_decoded: None,
                    },
                ]),
            },
        }),
        execution_info: Some(ExecutionInfo::Multisig(MultisigExecutionInfo {
            nonce: 135,
            confirmations_required: 2,
            confirmations_submitted: 2,
            missing_signers: None,
        })),
        safe_app_info: None,
    };

    let actual =
        MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider).await;

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}

#[rocket::async_test]
async fn multisig_transaction_to_custom_summary() {
    let multisig_tx =
        serde_json::from_str::<MultisigTransaction>(crate::tests::json::MULTISIG_TX_CUSTOM)
            .unwrap();
    let safe_info =
        serde_json::from_str::<SafeInfo>(crate::tests::json::SAFE_WITH_MODULES).unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let expected = TransactionSummary {
        id: create_id!(
            ID_PREFIX_MULTISIG_TX,
            "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
            "0x65df8a1e5a40703d9c67d5df6f9b552d3830faf0507c3d7350ba3764d3a68621"
        ),
        timestamp: multisig_tx.execution_date.unwrap().timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Custom(Custom {
            to: AddressEx::address_only("0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02"),
            data_size: "68".to_string(),
            value: "0".to_string(),
            method_name: Some("approve".to_string()),
            action_count: None,
            is_cancellation: false,
        }),
        execution_info: Some(ExecutionInfo::Multisig(MultisigExecutionInfo {
            nonce: 84,
            confirmations_required: 2,
            confirmations_submitted: 2,
            missing_signers: None,
        })),
        safe_app_info: None,
    };

    let actual =
        MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider).await;

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}

#[rocket::async_test]
async fn multisig_transaction_with_missing_signers() {
    let multisig_tx = serde_json::from_str::<MultisigTransaction>(
        crate::tests::json::MULTISIG_TX_AWAITING_CONFIRMATIONS,
    )
    .unwrap();
    let mut safe_info =
        serde_json::from_str::<SafeInfo>(crate::tests::json::SAFE_WITH_MODULES).unwrap();
    // Lower nonce so that transaction is pending again
    safe_info.nonce = 140;

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let expected = TransactionSummary {
        id: create_id!(
            ID_PREFIX_MULTISIG_TX,
            "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
            "0x6e631d27c638458329ba95cc17961e74b8146c46886545cd1984bb2bcf4eccd3"
        ),
        timestamp: multisig_tx.submission_date.timestamp_millis(),
        tx_status: TransactionStatus::AwaitingConfirmations,
        tx_info: TransactionInfo::Transfer(Transfer {
            sender: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
            recipient: AddressEx::address_only("0x938bae50a210b80EA233112800Cd5Bc2e7644300"),
            direction: TransferDirection::Outgoing,
            transfer_info: TransferInfo::NativeCoin(NativeCoinTransfer {
                value: "100000000000000000".to_string(),
            }),
        }),
        execution_info: Some(ExecutionInfo::Multisig(MultisigExecutionInfo {
            nonce: 147,
            confirmations_required: 2,
            confirmations_submitted: 1,
            missing_signers: Some(vec![
                AddressEx::address_only("0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23"),
                AddressEx::address_only("0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72"),
                AddressEx::address_only("0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D"),
                AddressEx::address_only("0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"),
            ]),
        })),
        safe_app_info: None,
    };

    let actual =
        MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider).await;

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}

#[rocket::async_test]
async fn ethereum_transaction_with_inconsistent_token_types() {
    let ethereum_tx = serde_json::from_str::<EthereumTransaction>(
        crate::tests::json::ETHEREUM_TX_INCONSISTENT_TOKEN_TYPES,
    )
    .unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let actual = EthereumTransaction::to_transaction_summary(
        &ethereum_tx,
        &mut mock_info_provider,
        "0xBc79855178842FDBA0c353494895DEEf509E26bB",
    )
    .await;
    let expected = TransactionSummary {
        id: create_id!(
            ID_PREFIX_ETHEREUM_TX,
            "0xBc79855178842FDBA0c353494895DEEf509E26bB",
            ethereum_tx.tx_hash,
            hex_hash(ethereum_tx.transfers.unwrap().first().unwrap())
        ),
        timestamp: ethereum_tx.execution_date.timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Transfer(Transfer {
            sender: AddressEx::address_only("0xd31e655bC4Eb5BCFe25A47d636B25bb4aa4041B2"),
            recipient: AddressEx::address_only("0xBc79855178842FDBA0c353494895DEEf509E26bB"),
            direction: TransferDirection::Incoming,
            transfer_info: TransferInfo::Erc721(Erc721Transfer {
                token_address: "0xb07de4b2989E180F8907B8C7e617637C26cE2776".to_string(),
                token_id: "856420144564".to_string(),
                token_name: Some("A! WWW.SPACESWAP.APP ! TOP DEFI AGGREGATOR !".to_string()),
                token_symbol: Some("A! WWW.SPACESWAP.APP ! TOP DEFI AGGREGATOR !".to_string()),
                logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xb07de4b2989E180F8907B8C7e617637C26cE2776.png".to_string()),
            }),
        }),
        execution_info: None,
        safe_app_info: None,
    };

    assert_eq!(1, actual.len());
    assert_eq!(&expected, actual.first().unwrap());
}

#[rocket::async_test]
async fn multisig_transaction_with_origin() {
    let multisig_tx =
        serde_json::from_str::<MultisigTransaction>(crate::tests::json::MULTISIG_TX_WITH_ORIGIN)
            .unwrap();
    let mut safe_info =
        serde_json::from_str::<SafeInfo>(crate::tests::json::SAFE_WITH_MODULES).unwrap();
    // Lower nonce so that transaction is pending again
    safe_info.nonce = 140;

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));
    mock_info_provider
        .expect_safe_app_info()
        .times(1)
        .return_once(move |_| {
            Ok(SafeAppInfo {
                name: "WalletConnect".to_string(),
                url: "https://apps.gnosis-safe.io/walletConnect".to_string(),
                logo_uri: "https://apps.gnosis-safe.io/walletConnect/walletConnect.jpg".to_string(),
            })
        });
    mock_info_provider.expect_token_info().times(0);

    let expected = TransactionSummary {
        id: create_id!(
            ID_PREFIX_MULTISIG_TX,
            "0xBc79855178842FDBA0c353494895DEEf509E26bB",
            "0x728e6dec56dc61523b56dc440e34c1c4c39c66895df8e5d3499ed1f7d4fcfe80"
        ),
        timestamp: multisig_tx.execution_date.unwrap().timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Custom(Custom {
            to: AddressEx::address_only("0x8D29bE29923b68abfDD21e541b9374737B49cdAD"),
            data_size: "3108".to_string(),
            value: "0".to_string(),
            method_name: Some("multiSend".to_string()),
            action_count: Some(1),
            is_cancellation: false,
        }),
        execution_info: Some(ExecutionInfo::Multisig(MultisigExecutionInfo {
            nonce: 160,
            confirmations_required: 2,
            confirmations_submitted: 2,
            missing_signers: None,
        })),
        safe_app_info: Some(SafeAppInfo {
            name: "WalletConnect".to_string(),
            url: "https://apps.gnosis-safe.io/walletConnect".to_string(),
            logo_uri: "https://apps.gnosis-safe.io/walletConnect/walletConnect.jpg".to_string(),
        }),
    };

    let actual =
        MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider).await;

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}
