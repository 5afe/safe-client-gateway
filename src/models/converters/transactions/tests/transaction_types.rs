use crate::models::backend::transactions::MultisigTransaction;
use crate::models::service::transactions::{TransactionInfo, Custom};
use crate::providers::info::*;

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
fn transaction_data_size_0_value_greater_than_0() {}

#[test]
fn transaction_data_size_greater_than_value_0_to_is_safe_is_settings_method() {}

#[test]
fn transaction_data_size_greater_than_value_0_to_is_safe_is_not_settings_method() {}

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
