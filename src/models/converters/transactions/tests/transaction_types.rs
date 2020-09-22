use crate::models::backend::transactions::MultisigTransaction;
use crate::models::service::transactions::{TransactionInfo, Custom, Transfer, EtherTransfer, TransferInfo, TransferDirection, SettingsChange, SettingsInfo};
use crate::providers::info::*;
use crate::models::commons::{DataDecoded, Parameter};
use crate::models::commons::ParamValue::SingleValue;

#[test]
fn transaction_operation_not_call() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(0);
    mock_info_provider
        .expect_token_info()
        .times(0);
    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ERC20_TRANSFER_DELEGATE).unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02".to_string(),
        data_size: "68".to_string(),
        value: "0".to_string(),
        method_name: Some("transfer".to_string()),
    });

    let actual = tx.transaction_info(&mut mock_info_provider);

    assert_eq!(expected, actual);
}

#[test]
fn transaction_data_size_and_value_greater_than_0() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(0);
    mock_info_provider
        .expect_token_info()
        .times(0);
    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ERC20_TRANSFER_WITH_VALUE).unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: "0xD9BA894E0097f8cC2BBc9D24D308b98e36dc6D02".to_string(),
        data_size: "68".to_string(),
        value: "100000000000000000".to_string(),
        method_name: Some("transfer".to_string()),
    });

    let actual = tx.transaction_info(&mut mock_info_provider);

    assert_eq!(expected, actual);
}

#[test]
fn transaction_data_size_0_value_greater_than_0() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(0);
    mock_info_provider
        .expect_token_info()
        .times(0);
    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_ETHER_TRANSFER).unwrap();
    let expected = TransactionInfo::Transfer(Transfer {
        sender: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        recipient: "0x938bae50a210b80EA233112800Cd5Bc2e7644300".to_string(),
        direction: TransferDirection::Outgoing,
        transfer_info: TransferInfo::Ether(EtherTransfer { value: "100000000000000000".to_string() }),
    });

    let actual = tx.transaction_info(&mut mock_info_provider);

    assert_eq!(expected, actual);
}

#[test]
fn transaction_data_size_greater_than_value_0_to_is_safe_is_settings_method() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(0);
    mock_info_provider
        .expect_token_info()
        .times(0);
    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_SETTINGS_CHANGE).unwrap();
    let expected = TransactionInfo::SettingsChange(SettingsChange {
        settings_info: Some(SettingsInfo::AddOwner {
            owner: "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_string(),
            threshold: 2,
        }),
        data_decoded: DataDecoded {
            method: "addOwnerWithThreshold".to_string(),
            parameters: Some(vec!(
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
                })),
        },
    });

    let actual = tx.transaction_info(&mut mock_info_provider);

    assert_eq!(expected, actual);
}

#[test]
fn transaction_data_size_greater_than_value_0_to_is_safe_is_not_settings_method() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_info()
        .times(0);
    mock_info_provider
        .expect_token_info()
        .times(0);
    let tx = serde_json::from_str::<MultisigTransaction>(crate::json::MULTISIG_TX_UNKNOWN_SETTINGS_CHANGE).unwrap();
    let expected = TransactionInfo::Custom(Custom {
        to: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
        data_size: "68".to_string(),
        value: "0".to_string(),
        method_name: Some("newAndDifferentAddOwnerWithThreshold".to_string()),
    });

    let actual = tx.transaction_info(&mut mock_info_provider);

    assert_eq!(expected, actual);
}

#[test]
fn transaction_data_decoded_is_erc20_receiver_ok_transfer_method() {}

#[test]
fn transaction_data_decoded_is_erc721_receiver_ok_transfer_method() {}

#[test]
fn transaction_data_decoded_is_erc20_receiver_not_ok_transfer_method() {}

#[test]
fn transaction_data_decoded_is_erc721_receiver_not_ok_transfer_method() {}

#[test]
fn transaction_data_decoded_is_transfer_method_receiver_ok_token_type_unknown() {}

#[test]
fn transaction_data_decoded_is_erc20_receiver_ok_token_not_found() {}

#[test]
fn transaction_data_decoded_is_erc721_receiver_ok_token_not_found() {}

#[test]
fn transaction_data_decoded_is_erc20_receiver_ok_token_fetch_error() {}

#[test]
fn transaction_data_decoded_is_other_method_method() {}
