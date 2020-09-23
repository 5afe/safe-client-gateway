use crate::utils::{SETTINGS_CHANGE_METHODS, ERC20_TRANSFER_METHODS, ERC721_TRANSFER_METHODS};
use crate::models::commons::DataDecoded;

#[test]
fn is_settings_method() {
    SETTINGS_CHANGE_METHODS.iter().for_each(|&item| {
        let data_decoded = DataDecoded { method: String::from(item), parameters: None };
        assert!(data_decoded.is_settings_change())
    });

    let unknown_setting_change = DataDecoded { method: String::from("unknownSettingChange"), parameters: None };
    assert!(!unknown_setting_change.is_settings_change())
}

#[test]
fn is_erc20_transfer_method() {
    ERC20_TRANSFER_METHODS.iter().for_each(|&item| {
        let data_decoded = DataDecoded { method: String::from(item), parameters: None };
        assert!(data_decoded.is_erc20_transfer_method())
    });

    let unknown_method = DataDecoded { method: String::from("unknownTransferMethod"), parameters: None };
    assert!(!unknown_method.is_erc20_transfer_method())
}

#[test]
fn is_erc721_transfer_method() {
    ERC721_TRANSFER_METHODS.iter().for_each(|&item| {
        let data_decoded = DataDecoded { method: String::from(item), parameters: None };
        assert!(data_decoded.is_erc721_transfer_method())
    });

    let unknown_method = DataDecoded { method: String::from("unknownTransferMethod"), parameters: None };
    assert!(!unknown_method.is_erc721_transfer_method())
}