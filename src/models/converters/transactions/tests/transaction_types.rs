use crate::models::backend::transactions::{ModuleTransaction, MultisigTransaction};
use crate::models::commons::ParamValue::SingleValue;
use crate::models::commons::{DataDecoded, Parameter};
use crate::models::service::addresses::AddressEx;
use crate::models::service::transactions::{
    Custom, Erc20Transfer, Erc721Transfer, NativeCoinTransfer, SettingsChange, SettingsInfo,
    TransactionInfo, Transfer, TransferDirection, TransferInfo,
};
use crate::providers::info::*;

#[rocket::async_test]
async fn transaction_operation_not_call() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let tx = serde_json::from_str::<MultisigTransaction>(
        crate::json::MULTISIG_TX_ERC20_TRANSFER_DELEGATE,
    )
    .unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: AddressEx::address_only("0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02"),
        data_size: "68".to_string(),
        value: "0".to_string(),
        method_name: Some("transfer".to_string()),
        action_count: None,
        is_cancellation: false,
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn transaction_data_size_and_value_greater_than_0() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let tx = serde_json::from_str::<MultisigTransaction>(
        crate::json::MULTISIG_TX_ERC20_TRANSFER_WITH_VALUE,
    )
    .unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: AddressEx::address_only("0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02"),
        data_size: "68".to_string(),
        value: "100000000000000000".to_string(),
        method_name: Some("transfer".to_string()),
        action_count: None,
        is_cancellation: false,
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn transaction_data_size_and_value_greater_than_0_with_address_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |address| {
            Ok(AddressEx {
                value: address.to_string(),
                name: Some("".to_string()),
                logo_uri: None,
            })
        });

    let tx = serde_json::from_str::<MultisigTransaction>(
        crate::json::MULTISIG_TX_ERC20_TRANSFER_WITH_VALUE,
    )
    .unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: AddressEx {
            value: "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02".to_string(),
            name: Some("".to_string()),
            logo_uri: None,
        },
        data_size: "68".to_string(),
        value: "100000000000000000".to_string(),
        method_name: Some("transfer".to_string()),
        action_count: None,
        is_cancellation: false,
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn transaction_data_size_0_value_greater_than_0() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ETHER_TRANSFER)
        .unwrap();
    let expected = TransactionInfo::Transfer(Transfer {
        sender: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        recipient: AddressEx::address_only("0x938bae50a210b80EA233112800Cd5Bc2e7644300"),
        direction: TransferDirection::Outgoing,
        transfer_info: TransferInfo::NativeCoin(NativeCoinTransfer {
            value: "100000000000000000".to_string(),
        }),
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn module_transaction_data_size_0_value_greater_than_0() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let tx =
        serde_json::from_str::<ModuleTransaction>(crate::json::MODULE_TX_ETHER_TRANSFER).unwrap();
    let expected = TransactionInfo::Transfer(Transfer {
        sender: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        recipient: AddressEx::address_only("0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02"),
        direction: TransferDirection::Outgoing,
        transfer_info: TransferInfo::NativeCoin(NativeCoinTransfer {
            value: "100000000000000000".to_string(),
        }),
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn transaction_data_size_greater_than_value_0_to_is_safe_is_settings_method() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(0);

    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_SETTINGS_CHANGE)
        .unwrap();
    let expected = TransactionInfo::SettingsChange(SettingsChange {
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
                    value: SingleValue("0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_string()),
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
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn transaction_data_size_greater_than_value_0_to_is_safe_is_settings_method_with_address_info(
) {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(0);

    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_SETTINGS_CHANGE)
        .unwrap();
    let expected = TransactionInfo::SettingsChange(SettingsChange {
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
                    value: SingleValue("0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_string()),
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
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn module_transaction_data_size_greater_than_value_0_to_is_safe_is_settings_method_with_address_info(
) {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    // We do not expect any address info loading as it is a call to the Safe itself
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(0);

    let tx =
        serde_json::from_str::<ModuleTransaction>(crate::json::MODULE_TX_SETTINGS_CHANGE).unwrap();
    let expected = TransactionInfo::SettingsChange(SettingsChange {
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
                    value: SingleValue("0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_string()),
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
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn transaction_data_size_greater_than_value_0_to_is_safe_is_not_settings_method() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    // We do not expect any address info loading as it is a call to the Safe itself
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(0);

    let tx = serde_json::from_str::<MultisigTransaction>(
        crate::json::MULTISIG_TX_UNKNOWN_SETTINGS_CHANGE,
    )
    .unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        data_size: "68".to_string(),
        value: "0".to_string(),
        method_name: Some("newAndDifferentAddOwnerWithThreshold".to_string()),
        action_count: None,
        is_cancellation: false,
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn module_transaction_data_size_greater_than_value_0_to_is_safe_is_not_settings_method() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    // We do not expect any address info loading as it is a call to the Safe itself
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(0);

    let tx =
        serde_json::from_str::<ModuleTransaction>(crate::json::MODULE_TX_UNKNOWN_SETTINGS_CHANGE)
            .unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        data_size: "68".to_string(),
        value: "0".to_string(),
        method_name: Some("newAndDifferentAddOwnerWithThreshold".to_string()),
        action_count: None,
        is_cancellation: false,
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn transaction_data_decoded_is_erc20_receiver_ok_transfer_method() {
    let token_info = serde_json::from_str::<TokenInfo>(crate::json::TOKEN_USDT).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ERC20_TRANSFER)
        .unwrap();
    let expected = TransactionInfo::Transfer(Transfer {
        sender: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        recipient: AddressEx::address_only("0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0"),
        direction: TransferDirection::Outgoing,
        transfer_info: TransferInfo::Erc20(
            Erc20Transfer {
                token_address: "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02".to_string(),
                token_name: Some("Compound USDT".to_string()),
                token_symbol: Some("USDT".to_string()),
                logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02.png".to_string()),
                decimals: Some(18),
                value: "50000000000000".to_string(),
            }),
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn module_transaction_data_decoded_is_erc20_receiver_ok_transfer_method() {
    let token_info = serde_json::from_str::<TokenInfo>(crate::json::TOKEN_USDT).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let tx =
        serde_json::from_str::<ModuleTransaction>(crate::json::MODULE_TX_ERC20_TRANSFER).unwrap();
    let expected = TransactionInfo::Transfer(Transfer {
        sender: AddressEx::address_only("0x1230B3d59858296A31053C1b8562Ecf89A2f888b"),
        recipient: AddressEx::address_only("0xF353eBBa77e5E71c210599236686D51cA1F88b84"),
        direction: TransferDirection::Outgoing,
        transfer_info: TransferInfo::Erc20(
            Erc20Transfer {
                token_address: "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02".to_string(),
                token_name: Some("Compound USDT".to_string()),
                token_symbol: Some("USDT".to_string()),
                logo_uri: Some("https://gnosis-safe-token-logos.s3.amazonaws.com/0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02.png".to_string()),
                decimals: Some(18),
                value: "100000000000000".to_string(),
            }),
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn transaction_data_decoded_is_erc721_receiver_ok_transfer_method() {
    let token_info = serde_json::from_str::<TokenInfo>(crate::json::TOKEN_CRYPTO_KITTIES).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ERC721_TRANSFER)
        .unwrap();
    let expected = TransactionInfo::Transfer(Transfer {
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
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn module_transaction_data_decoded_is_erc721_receiver_ok_transfer_method() {
    let token_info = serde_json::from_str::<TokenInfo>(crate::json::TOKEN_CRYPTO_KITTIES).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let tx =
        serde_json::from_str::<ModuleTransaction>(crate::json::MODULE_TX_ERC721_TRANSFER).unwrap();
    let expected = TransactionInfo::Transfer(Transfer {
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
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn transaction_data_decoded_is_erc20_receiver_not_ok_transfer_method() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("no address info"));

    let tx = serde_json::from_str::<MultisigTransaction>(
        crate::json::MULTISIG_TX_ERC20_TRANSFER_INVALID_TO_AND_FROM,
    )
    .unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: AddressEx::address_only("0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02"),
        data_size: "68".to_string(),
        value: "0".to_string(),
        method_name: Some("transferFrom".to_string()),
        action_count: None,
        is_cancellation: false,
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn transaction_data_decoded_is_erc721_receiver_not_ok_transfer_method() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let tx = serde_json::from_str::<MultisigTransaction>(
        crate::json::MULTISIG_TX_ERC721_TRANSFER_INVALID_TO_AND_FROM,
    )
    .unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: AddressEx::address_only("0x16baF0dE678E52367adC69fD067E5eDd1D33e3bF"),
        data_size: "68".to_string(),
        value: "0".to_string(),
        method_name: Some("safeTransferFrom".to_string()),
        action_count: None,
        is_cancellation: false,
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn transaction_data_decoded_is_transfer_method_receiver_ok_token_type_unknown() {
    let token_info = TokenInfo {
        token_type: TokenType::Unknown,
        address: "".to_string(),
        decimals: 0,
        symbol: "".to_string(),
        name: "".to_string(),
        logo_uri: None,
    };
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| Ok(token_info));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ERC721_TRANSFER)
        .unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: AddressEx::address_only("0x16baF0dE678E52367adC69fD067E5eDd1D33e3bF"),
        data_size: "68".to_string(),
        value: "0".to_string(),
        method_name: Some("transfer".to_string()),
        action_count: None,
        is_cancellation: false,
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn transaction_data_decoded_is_erc20_receiver_ok_token_fetch_error() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider
        .expect_token_info()
        .times(1)
        .return_once(move |_| bail!("No token info"));
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(1)
        .return_once(move |_| bail!("No address info"));

    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ERC721_TRANSFER)
        .unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: AddressEx::address_only("0x16baF0dE678E52367adC69fD067E5eDd1D33e3bF"),
        data_size: "68".to_string(),
        value: "0".to_string(),
        method_name: Some("transfer".to_string()),
        action_count: None,
        is_cancellation: false,
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn cancellation_transaction() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_safe_info().times(0);
    mock_info_provider.expect_token_info().times(0);
    // We do not expect any address info loading as it is a call to the Safe itself
    mock_info_provider
        .expect_address_ex_from_any_source()
        .times(0);

    let tx =
        serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_CANCELLATION).unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: AddressEx::address_only("0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67"),
        data_size: "0".to_string(),
        value: "0".to_string(),
        method_name: None,
        action_count: None,
        is_cancellation: true,
    });

    let actual = tx.transaction_info(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}
