use crate::models::backend::transactions::{
    CreationTransaction, EthereumTransaction, ModuleTransaction, MultisigTransaction,
    Transaction as TransactionDto,
};
use crate::models::backend::transfers::{
    EtherTransfer as EtherTransferDto, Transfer as TransferDto,
};
use crate::models::commons::ParamValue::SingleValue;
use crate::models::commons::{DataDecoded, Operation, Parameter};
use crate::models::converters::transactions::data_size;
use crate::models::service::transactions::summary::{ExecutionInfo, TransactionSummary};
use crate::models::service::transactions::{
    Creation, Custom, Erc20Transfer, Erc721Transfer, EtherTransfer, SettingsChange, SettingsInfo,
    TransactionInfo, TransactionStatus, Transfer, TransferDirection, TransferInfo,
    ID_PREFIX_CREATION_TX, ID_PREFIX_ETHEREUM_TX, ID_PREFIX_MODULE_TX, ID_PREFIX_MULTISIG_TX,
};
use crate::providers::info::*;
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

#[test]
fn unknown_tx_to_summary_transaction() {
    let unknown_tx = TransactionDto::Unknown;
    let mut mock_info_provider = MockInfoProvider::new();

    let error = unknown_tx.to_transaction_summary(&mut mock_info_provider, &String::from(""));

    assert!(error.is_err());
}

#[test]
fn module_tx_to_summary_transaction() {
    let expected_to = String::from("0x12345789");
    let expected_date = Utc::now();
    let expected_date_in_millis = expected_date.timestamp_millis();
    let module_tx = ModuleTransaction {
        created: String::from("created"),
        execution_date: expected_date,
        block_number: 0,
        transaction_hash: String::from("tx_hash"),
        safe: String::from("safe"),
        module: String::from("module"),
        to: expected_to.clone(),
        value: None,
        data: None,
        data_decoded: None,
        operation: Operation::CALL,
    };

    let actual = ModuleTransaction::to_transaction_summary(&module_tx);
    let expected = vec![TransactionSummary {
        id: create_id!(
            ID_PREFIX_MODULE_TX,
            module_tx.safe,
            module_tx.transaction_hash,
            hex_hash(&module_tx)
        ),
        timestamp: expected_date_in_millis,
        tx_status: TransactionStatus::Success,
        execution_info: None,
        tx_info: TransactionInfo::Custom(Custom {
            to: expected_to,
            data_size: String::from("0"),
            value: String::from("0"),
            method_name: None,
        }),
    }];
    assert_eq!(actual, expected);
}

#[test]
fn ethereum_tx_to_summary_transaction_no_transfers() {
    let safe_address = String::from("0x2323");
    let mut mock_info_provider = MockInfoProvider::new();

    let ethereum_tx = EthereumTransaction {
        execution_date: Utc::now(),
        to: String::from("0x1234"),
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
    );
    assert_eq!(actual, Vec::new());
}

#[test]
fn ethereum_tx_to_summary_transaction_with_transfers() {
    let safe_address = String::from("0x2323");
    let mut mock_info_provider = MockInfoProvider::new();
    let timestamp = Utc::now();
    let timestamp_millis = timestamp.timestamp_millis();

    let transfers = vec![
        TransferDto::Ether(EtherTransferDto {
            execution_date: timestamp,
            block_number: 0,
            transaction_hash: "".to_string(),
            to: "".to_string(),
            value: String::from("1"),
            from: "".to_string(),
        }),
        TransferDto::Ether(EtherTransferDto {
            execution_date: timestamp,
            block_number: 0,
            transaction_hash: "".to_string(),
            to: "".to_string(),
            value: String::from("1"),
            from: "".to_string(),
        }),
    ];
    let ethereum_tx = EthereumTransaction {
        execution_date: timestamp,
        to: String::from("0x1234"),
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
    );
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
                sender: "".to_string(),
                recipient: "".to_string(),
                direction: TransferDirection::Unknown,
                transfer_info: TransferInfo::Ether(EtherTransfer {
                    value: "1".to_string(),
                }),
            }),
            execution_info: None,
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
                sender: "".to_string(),
                recipient: "".to_string(),
                direction: TransferDirection::Unknown,
                transfer_info: TransferInfo::Ether(EtherTransfer {
                    value: "1".to_string(),
                }),
            }),
            execution_info: None,
        },
    ];
    assert_eq!(actual, expected);
}

#[test]
fn creation_transaction_to_summary() {
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
            creator: creator,
            transaction_hash: transaction_hash,
            implementation: Some(master_copy),
            factory: Some(factory_address),
        }),
        execution_info: None,
    };

    let actual = creation_tx.to_transaction_summary(&safe_address);

    assert_eq!(expected, actual);
}

#[test]
fn multisig_transaction_to_erc20_transfer_summary() {
    let multisig_tx =
        serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ERC20_TRANSFER)
            .unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    let token_info = serde_json::from_str::<TokenInfo>(crate::json::TOKEN_USDT).unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));

    let expected = TransactionSummary {
        id: create_id!(ID_PREFIX_MULTISIG_TX, "0x1230B3d59858296A31053C1b8562Ecf89A2f888b", "0x95e32bb8cb88ecdc45732c0a551eae7b3744187cf1ba19cda1440eaaf7b4950c"),
        timestamp: multisig_tx.execution_date.unwrap().timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Transfer(Transfer {
            sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
            recipient: "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(),
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
        execution_info: Some(ExecutionInfo {
            nonce: 178,
            confirmations_required: 3,
            confirmations_submitted: 3,
            missing_signers: vec![],
        }),
    };

    let actual = MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider);

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}

#[test]
fn multisig_transaction_to_erc721_transfer_summary() {
    let multisig_tx =
        serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ERC721_TRANSFER)
            .unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    let token_info = serde_json::from_str::<TokenInfo>(crate::json::TOKEN_CRYPTO_KITTIES).unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));

    let expected = TransactionSummary {
        id: create_id!(ID_PREFIX_MULTISIG_TX, "0x1230B3d59858296A31053C1b8562Ecf89A2f888b", "0x9155f7741dd33572bc49c251eb4f4a5e9cf9653151417bdc4a2aca0767779603"),
        timestamp: multisig_tx.execution_date.unwrap().timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Transfer(Transfer {
            sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
            recipient: "0x938bae50a210b80EA233112800Cd5Bc2e7644300".to_string(),
            direction: TransferDirection::Outgoing,
            transfer_info: TransferInfo::Erc721(Erc721Transfer {
                token_address: "0x16baF0dE678E52367adC69fD067E5eDd1D33e3bF".to_string(),
                token_id: "1316".to_string(),
                token_name: Some("CryptoKitties".to_string()),
                token_symbol: Some("CK".to_string()),
                logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0x16baF0dE678E52367adC69fD067E5eDd1D33e3bF.png".to_string()),
            }),
        }),
        execution_info: Some(ExecutionInfo {
            nonce: 177,
            confirmations_required: 3,
            confirmations_submitted: 3,
            missing_signers: vec![],
        }),
    };

    let actual = MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider);

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}

#[test]
fn multisig_transaction_to_ether_transfer_summary() {
    let multisig_tx =
        serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ETHER_TRANSFER)
            .unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider.expect_token_info().times(0);

    let expected = TransactionSummary {
        id: create_id!(
            ID_PREFIX_MULTISIG_TX,
            "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
            "0x6e631d27c638458329ba95cc17961e74b8146c46886545cd1984bb2bcf4eccd3"
        ),
        timestamp: multisig_tx.execution_date.unwrap().timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Transfer(Transfer {
            sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
            recipient: "0x938bae50a210b80EA233112800Cd5Bc2e7644300".to_string(),
            direction: TransferDirection::Outgoing,
            transfer_info: TransferInfo::Ether(EtherTransfer {
                value: "100000000000000000".to_string(),
            }),
        }),
        execution_info: Some(ExecutionInfo {
            nonce: 147,
            confirmations_required: 2,
            confirmations_submitted: 2,
            missing_signers: vec![],
        }),
    };

    let actual = MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider);

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}

#[test]
fn multisig_transaction_to_settings_change_summary() {
    let multisig_tx =
        serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_SETTINGS_CHANGE)
            .unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
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
                owner: "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_string(),
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
        execution_info: Some(ExecutionInfo {
            nonce: 135,
            confirmations_required: 2,
            confirmations_submitted: 2,
            missing_signers: vec![],
        }),
    };

    let actual = MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider);

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}

#[test]
fn multisig_transaction_to_custom_summary() {
    let multisig_tx =
        serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_CUSTOM).unwrap();
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider.expect_token_info().times(0);

    let expected = TransactionSummary {
        id: create_id!(
            ID_PREFIX_MULTISIG_TX,
            "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
            "0x65df8a1e5a40703d9c67d5df6f9b552d3830faf0507c3d7350ba3764d3a68621"
        ),
        timestamp: multisig_tx.execution_date.unwrap().timestamp_millis(),
        tx_status: TransactionStatus::Success,
        tx_info: TransactionInfo::Custom(Custom {
            to: "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02".to_string(),
            data_size: "68".to_string(),
            value: "0".to_string(),
            method_name: Some("approve".to_string()),
        }),
        execution_info: Some(ExecutionInfo {
            nonce: 84,
            confirmations_required: 2,
            confirmations_submitted: 2,
            missing_signers: vec![],
        }),
    };

    let actual = MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider);

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}

#[test]
fn multisig_transaction_with_missing_signers() {
    let multisig_tx = serde_json::from_str::<MultisigTransaction>(
        crate::json::MULTISIG_TX_AWAITING_CONFIRMATIONS,
    )
    .unwrap();
    let mut safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    // Lower nonce so that transaction is pending again
    safe_info.nonce = 140;

    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(1)
        .return_once(move |_| Ok(safe_info));
    mock_info_provider.expect_token_info().times(0);

    let expected = TransactionSummary {
        id: create_id!(
            ID_PREFIX_MULTISIG_TX,
            "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
            "0x6e631d27c638458329ba95cc17961e74b8146c46886545cd1984bb2bcf4eccd3"
        ),
        timestamp: multisig_tx.submission_date.timestamp_millis(),
        tx_status: TransactionStatus::AwaitingConfirmations,
        tx_info: TransactionInfo::Transfer(Transfer {
            sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
            recipient: "0x938bae50a210b80EA233112800Cd5Bc2e7644300".to_string(),
            direction: TransferDirection::Outgoing,
            transfer_info: TransferInfo::Ether(EtherTransfer {
                value: "100000000000000000".to_string(),
            }),
        }),
        execution_info: Some(ExecutionInfo {
            nonce: 147,
            confirmations_required: 2,
            confirmations_submitted: 1,
            missing_signers: vec![
                "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23".to_owned(),
                "0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72".to_owned(),
                "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_owned(),
                "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_owned(),
            ],
        }),
    };

    let actual = MultisigTransaction::to_transaction_summary(&multisig_tx, &mut mock_info_provider);

    assert_eq!(&expected, actual.unwrap().get(0).unwrap());
}
