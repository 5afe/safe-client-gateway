use crate::models::commons::DataDecoded;
use crate::models::service::transactions::{SettingsChange, SettingsInfo};

#[test]
fn data_decoded_set_fallback_handler_to_settings_info() {
    let data_decoded = serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_SET_FALLBACK_HANDLER).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: SettingsInfo::SetFallbackHandler {
            handler: "0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44".to_string(),
        },
    };

    let actual = DataDecoded::to_settings_info(&data_decoded);

    assert_eq!(expected.settings_info, actual);
}

#[test]
fn data_decoded_add_owner_with_threshold_to_settings_info() {
    let data_decoded = serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_ADD_OWNER_WITH_THRESHOLD).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: SettingsInfo::AddOwnerWithThreshold {
            owner: "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23".to_string(),
            threshold: 1,
        },
    };

    let actual = DataDecoded::to_settings_info(&data_decoded);

    assert_eq!(expected.settings_info, actual);
}

#[test]
fn data_decoded_remove_owner_to_settings_info() {
    let data_decoded = serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_REMOVE_OWNER).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: SettingsInfo::RemoveOwner {
            prev_owner: "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(),
            owner: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
            threshold: 2,
        },
    };

    let actual = DataDecoded::to_settings_info(&data_decoded);

    assert_eq!(expected.settings_info, actual);
}

#[test]
fn data_decoded_swap_owner_to_settings_info() {
    let data_decoded = serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_SWAP_OWNER).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: SettingsInfo::SwapOwner {
            prev_owner: "0x0000000000000000000000000000000000000001".to_string(),
            old_owner: "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_string(),
            new_owner: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
        },
    };

    let actual = DataDecoded::to_settings_info(&data_decoded);

    assert_eq!(expected.settings_info, actual);
}

#[test]
fn data_decoded_change_threshold_to_settings_info() {
    let data_decoded = serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_CHANGE_THRESHOLD).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: SettingsInfo::ChangeThreshold {
            threshold: 2
        },
    };

    let actual = DataDecoded::to_settings_info(&data_decoded);

    assert_eq!(expected.settings_info, actual);
}

#[test]
fn data_decoded_change_implementation_to_settings_info() {
    let data_decoded = serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_CHANGE_MASTER_COPY).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: SettingsInfo::ChangeImplementation {
            implementation: "0xb6029EA3B2c51D09a50B53CA8012FeEB05bDa35A".to_string()
        },
    };

    let actual = DataDecoded::to_settings_info(&data_decoded);

    assert_eq!(expected.settings_info, actual);
}

#[test]
fn data_decoded_enable_module_to_settings_info() {
    let data_decoded = serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_ENABLE_MODULE).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: SettingsInfo::EnableModule {
            module: "0xF5dC3718EEbC5b003F1672A499F2ACBE77Ba790d".to_string()
        },
    };

    let actual = DataDecoded::to_settings_info(&data_decoded);

    assert_eq!(expected.settings_info, actual);
}

#[test]
fn data_decoded_disable_module_to_settings_info() {
    let data_decoded = serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_DISABLE_MODULE).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: SettingsInfo::DisableModule {
            prev_module: "0xF5dC3718EEbC5b003F1672A499F2ACBE77Ba790d".to_string(),
            module: "0x25F73b24B866963B0e560fFF9bbA7908be0263E8".to_string(),
        },
    };

    let actual = DataDecoded::to_settings_info(&data_decoded);

    assert_eq!(expected.settings_info, actual);
}

#[test]
fn data_decoded_unknown_to_settings_info() {
    let data_decoded = serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_EXEC_TRANSACTION_FROM_MODULE).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: SettingsInfo::Unknown,
    };

    let actual = DataDecoded::to_settings_info(&data_decoded);

    assert_eq!(expected.settings_info, actual);
}
