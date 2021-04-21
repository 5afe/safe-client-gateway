use crate::models::commons::{DataDecoded, ParamValue, Parameter};
use crate::models::service::transactions::{SettingsChange, SettingsInfo};
use crate::providers::address_info::AddressInfo;
use crate::providers::info::*;
use mockall::predicate::eq;
use mockall::Sequence;
use std::collections::HashMap;

#[rocket::async_test]
async fn data_decoded_set_fallback_handler_to_settings_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_contract_info()
        .times(1)
        .return_once(move |_| bail!("Some http error"));

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_SET_FALLBACK_HANDLER)
            .unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::SetFallbackHandler {
            handler: "0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44".to_string(),
            handler_info: None,
        }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_set_fallback_handler_to_settings_info_with_address_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_contract_info()
        .times(1)
        .return_once(move |_| {
            Ok(AddressInfo {
                name: "Address name".to_string(),
                logo_uri: Some("logo.url".to_string()),
            })
        });

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_SET_FALLBACK_HANDLER)
            .unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::SetFallbackHandler {
            handler: "0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44".to_string(),
            handler_info: Some(AddressInfo {
                name: "Address name".to_string(),
                logo_uri: Some("logo.url".to_string()),
            }),
        }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_add_owner_with_threshold_to_settings_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_contract_info().times(0);

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_ADD_OWNER_WITH_THRESHOLD)
            .unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::AddOwner {
            owner: "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23".to_string(),
            owner_info: None,
            threshold: 1,
        }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_add_owner_with_threshold_to_settings_info_with_address_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_contract_info().times(0);

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_ADD_OWNER_WITH_THRESHOLD)
            .unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::AddOwner {
            owner: "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23".to_string(),
            owner_info: None,
            threshold: 1,
        }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_remove_owner_to_settings_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_contract_info().times(0);

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_REMOVE_OWNER).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::RemoveOwner {
            owner: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
            owner_info: None,
            threshold: 2,
        }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_swap_owner_to_settings_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_contract_info().times(0);

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_SWAP_OWNER).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::SwapOwner {
            old_owner: "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_string(),
            old_owner_info: None,
            new_owner: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
            new_owner_info: None,
        }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_change_threshold_to_settings_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider.expect_contract_info().times(0);

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_CHANGE_THRESHOLD).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::ChangeThreshold { threshold: 2 }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_change_implementation_to_settings_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_contract_info()
        .times(1)
        .return_once(move |_| bail!("Some http error"));

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_CHANGE_MASTER_COPY).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::ChangeImplementation {
            implementation: "0xb6029EA3B2c51D09a50B53CA8012FeEB05bDa35A".to_string(),
            implementation_info: None,
        }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_change_implementation_to_settings_info_with_address_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_contract_info()
        .times(1)
        .return_once(move |_| {
            Ok(AddressInfo {
                name: "Address name".to_string(),
                logo_uri: Some("logo.url".to_string()),
            })
        });

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_CHANGE_MASTER_COPY).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::ChangeImplementation {
            implementation: "0xb6029EA3B2c51D09a50B53CA8012FeEB05bDa35A".to_string(),
            implementation_info: Some(AddressInfo {
                name: "Address name".to_string(),
                logo_uri: Some("logo.url".to_string()),
            }),
        }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_enable_module_to_settings_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_contract_info()
        .times(1)
        .return_once(move |_| bail!("Some http error"));

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_ENABLE_MODULE).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::EnableModule {
            module: "0xF5dC3718EEbC5b003F1672A499F2ACBE77Ba790d".to_string(),
            module_info: None,
        }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_enable_module_to_settings_info_with_address_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_contract_info()
        .times(1)
        .return_once(move |_| {
            Ok(AddressInfo {
                name: "Address name".to_string(),
                logo_uri: Some("logo.url".to_string()),
            })
        });

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_ENABLE_MODULE).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::EnableModule {
            module: "0xF5dC3718EEbC5b003F1672A499F2ACBE77Ba790d".to_string(),
            module_info: Some(AddressInfo {
                name: "Address name".to_string(),
                logo_uri: Some("logo.url".to_string()),
            }),
        }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_disable_module_to_settings_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_contract_info()
        .times(1)
        .return_once(move |_| bail!("Some http error"));

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_DISABLE_MODULE).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::DisableModule {
            module: "0x25F73b24B866963B0e560fFF9bbA7908be0263E8".to_string(),
            module_info: None,
        }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_disable_module_to_settings_info_with_address_info() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_contract_info()
        .times(1)
        .return_once(move |_| {
            Ok(AddressInfo {
                name: "Address name".to_string(),
                logo_uri: Some("logo.url".to_string()),
            })
        });

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_DISABLE_MODULE).unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: Some(SettingsInfo::DisableModule {
            module: "0x25F73b24B866963B0e560fFF9bbA7908be0263E8".to_string(),
            module_info: Some(AddressInfo {
                name: "Address name".to_string(),
                logo_uri: Some("logo.url".to_string()),
            }),
        }),
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[rocket::async_test]
async fn data_decoded_unknown_to_settings_info() {
    let mut mock_info_provider = MockInfoProvider::new();

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_EXEC_TRANSACTION_FROM_MODULE)
            .unwrap();

    let expected = SettingsChange {
        data_decoded: data_decoded.clone(),
        settings_info: None,
    };

    let actual = DataDecoded::to_settings_info(&data_decoded, &mut mock_info_provider).await;

    assert_eq!(expected.settings_info, actual);
}

#[test]
fn data_decoded_with_nested_safe_transaction() {
    let data_decoded = serde_json::from_str::<DataDecoded>(
        crate::json::DATA_DECODED_EXEC_TRANSACTION_WITH_VALUE_DECODED,
    )
    .unwrap();

    let expected = DataDecoded {
        method: "execTransaction".to_string(),
        parameters: Some(vec![
            Parameter {
                name: "to".to_string(),
                param_type: "address".to_string(),
                value: ParamValue::SingleValue("0x441E604Ad49602c0B9C0B08D0781eCF96740786a".to_string()),
                value_decoded: None,
            },
            Parameter {
                name: "value".to_string(),
                param_type: "uint256".to_string(),
                value: ParamValue::SingleValue("0".to_string()),
                value_decoded: None,
            },
            Parameter {
                name: "data".to_string(),
                param_type: "bytes".to_string(),
                value: ParamValue::SingleValue("0x610b592500000000000000000000000034cfac646f301356faa8b21e94227e3583fe3f5f".to_string()),
                value_decoded: None,
            },
            Parameter {
                name: "operation".to_string(),
                param_type: "uint8".to_string(),
                value: ParamValue::SingleValue("0".to_string()),
                value_decoded: None,
            },
            Parameter {
                name: "safeTxGas".to_string(),
                param_type: "uint256".to_string(),
                value: ParamValue::SingleValue("53036".to_string()),
                value_decoded: None,
            },
            Parameter {
                name: "baseGas".to_string(),
                param_type: "uint256".to_string(),
                value: ParamValue::SingleValue("0".to_string()),
                value_decoded: None,
            },
            Parameter {
                name: "gasPrice".to_string(),
                param_type: "uint256".to_string(),
                value: ParamValue::SingleValue("0".to_string()),
                value_decoded: None,
            },
            Parameter {
                name: "gasToken".to_string(),
                param_type: "address".to_string(),
                value: ParamValue::SingleValue("0x0000000000000000000000000000000000000000".to_string()),
                value_decoded: None,
            },
            Parameter {
                name: "refundReceiver".to_string(),
                param_type: "address".to_string(),
                value: ParamValue::SingleValue("0x0000000000000000000000000000000000000000".to_string()),
                value_decoded: None,
            },
            Parameter {
                name: "signatures".to_string(),
                param_type: "bytes".to_string(),
                value: ParamValue::SingleValue("0x0000000000000000000000000e24b6e3beff0b44b773f068343bc2cb56cb37690000000000000000000000000000000000000000000000000000000000000000017e86d3185b70c297e33c7691d537fb9f11601ceb3a34f3c7b50fc7a3086380451c0924eac2e1bdd9cab77a96ced513f4c9df0432a19e9b61859261cdfb7dd6b41b".to_string()),
                value_decoded: None,
            },
        ]),
    };

    assert_eq!(expected, data_decoded);
}

#[test]
fn address_info_index_not_multi_send_address_single_value() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0xb6029EA3B2c51D09a50B53CA8012FeEB05bDa35A"))
        .times(1)
        .return_once(move |_| {
            Ok(AddressInfo {
                name: "Master Copy".to_string(),
                logo_uri: Some("url.de".to_string()),
            })
        });

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_CHANGE_MASTER_COPY).unwrap();

    let expected = {
        let mut map = HashMap::new();
        map.insert(
            "0xb6029EA3B2c51D09a50B53CA8012FeEB05bDa35A".to_string(),
            AddressInfo {
                name: "Master Copy".to_string(),
                logo_uri: Some("url.de".to_string()),
            },
        );
        map
    };

    let actual = data_decoded.build_address_info_index(&mut mock_info_provider);

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn address_info_index_not_multi_send_address_array_value() {
    // expected address in json, one will not return to test behaviour of that too
    // 1) "0x4FB84d2dFc50017aFa759107a389759c8fD077DE" -> returns
    // 2) "0x111111111117dC0aa78b770fA6A738034120C302" -> returns
    // 3) "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE" -> bails
    // 4) "0x4FB84d2dFc50017aFa759107a389759c8fD077DE" -> skip duplicate
    // 5) "0xBc79855178842FDBA0c353494895DEEf509E26bB" -> bails
    // 6) "0x991c44331f0E59510Bcff76edBA06C3f552Eef8B" -> returns
    // we expect the index to contain 4 values

    let mut sequence = Sequence::new();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0x4FB84d2dFc50017aFa759107a389759c8fD077DE"))
        .times(1)
        .return_once(move |address| {
            Ok(AddressInfo {
                name: format!("{}_name", &address),
                logo_uri: Some(format!("{}_url", &address)),
            })
        })
        .in_sequence(&mut sequence);

    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0x111111111117dC0aa78b770fA6A738034120C302"))
        .times(1)
        .return_once(move |address| {
            Ok(AddressInfo {
                name: format!("{}_name", &address),
                logo_uri: Some(format!("{}_url", &address)),
            })
        })
        .in_sequence(&mut sequence);

    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"))
        .times(1)
        .return_once(move |_| bail!("no address"))
        .in_sequence(&mut sequence);

    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0xBc79855178842FDBA0c353494895DEEf509E26bB"))
        .times(1)
        .return_once(move |_| bail!("no address"))
        .in_sequence(&mut sequence);

    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0x991c44331f0E59510Bcff76edBA06C3f552Eef8B"))
        .times(1)
        .return_once(move |address| {
            Ok(AddressInfo {
                name: format!("{}_name", &address),
                logo_uri: Some(format!("{}_url", &address)),
            })
        })
        .in_sequence(&mut sequence);

    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DATA_DECODED_SWAP_ARRAY_VALUES).unwrap();

    let expected = {
        let mut map = HashMap::new();
        map.insert(
            "0x4FB84d2dFc50017aFa759107a389759c8fD077DE".to_string(),
            AddressInfo {
                name: "0x4FB84d2dFc50017aFa759107a389759c8fD077DE_name".to_string(),
                logo_uri: Some("0x4FB84d2dFc50017aFa759107a389759c8fD077DE_url".to_string()),
            },
        );

        map.insert(
            "0x111111111117dC0aa78b770fA6A738034120C302".to_string(),
            AddressInfo {
                name: "0x111111111117dC0aa78b770fA6A738034120C302_name".to_string(),
                logo_uri: Some("0x111111111117dC0aa78b770fA6A738034120C302_url".to_string()),
            },
        );

        map.insert(
            "0x991c44331f0E59510Bcff76edBA06C3f552Eef8B".to_string(),
            AddressInfo {
                name: "0x991c44331f0E59510Bcff76edBA06C3f552Eef8B_name".to_string(),
                logo_uri: Some("0x991c44331f0E59510Bcff76edBA06C3f552Eef8B_url".to_string()),
            },
        );

        map
    };

    let actual = data_decoded.build_address_info_index(&mut mock_info_provider);

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn address_info_index_multi_send_single_level_of_nesting() {
    let mut sequence = Sequence::new();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0x111111125434b319222CdBf8C261674aDB56F3ae"))
        .times(1)
        .return_once(move |address| {
            Ok(AddressInfo {
                name: format!("{}_name", &address),
                logo_uri: Some(format!("{}_url", &address)),
            })
        })
        .in_sequence(&mut sequence);

    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0xd47140F6Ab73f6d6B6675Fb1610Bb5E9B5d96FE5"))
        .times(1)
        .return_once(move |address| {
            Ok(AddressInfo {
                name: format!("{}_name", &address),
                logo_uri: Some(format!("{}_url", &address)),
            })
        })
        .in_sequence(&mut sequence);

    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"))
        .times(1)
        .return_once(move |address| {
            Ok(AddressInfo {
                name: format!("{}_name", &address),
                logo_uri: Some(format!("{}_url", &address)),
            })
        })
        .in_sequence(&mut sequence);

    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2"))
        .times(1)
        .return_once(move |address| {
            Ok(AddressInfo {
                name: format!("{}_name", &address),
                logo_uri: Some(format!("{}_url", &address)),
            })
        })
        .in_sequence(&mut sequence);

    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0xBc79855178842FDBA0c353494895DEEf509E26bB"))
        .times(1)
        .return_once(move |address| {
            Ok(AddressInfo {
                name: format!("{}_name", &address),
                logo_uri: Some(format!("{}_url", &address)),
            })
        })
        .in_sequence(&mut sequence);

    let data_decoded = serde_json::from_str::<DataDecoded>(
        crate::json::DATA_DECODED_MULTI_SEND_SINGLE_INNER_TRANSACTION,
    )
    .unwrap();

    let expected = {
        let mut map = HashMap::new();
        map.insert(
            "0x111111125434b319222CdBf8C261674aDB56F3ae".to_string(),
            AddressInfo {
                name: "0x111111125434b319222CdBf8C261674aDB56F3ae_name".to_string(),
                logo_uri: Some("0x111111125434b319222CdBf8C261674aDB56F3ae_url".to_string()),
            },
        );

        map.insert(
            "0xd47140F6Ab73f6d6B6675Fb1610Bb5E9B5d96FE5".to_string(),
            AddressInfo {
                name: "0xd47140F6Ab73f6d6B6675Fb1610Bb5E9B5d96FE5_name".to_string(),
                logo_uri: Some("0xd47140F6Ab73f6d6B6675Fb1610Bb5E9B5d96FE5_url".to_string()),
            },
        );

        map.insert(
            "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE".to_string(),
            AddressInfo {
                name: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE_name".to_string(),
                logo_uri: Some("0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE_url".to_string()),
            },
        );

        map.insert(
            "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2".to_string(),
            AddressInfo {
                name: "0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2_name".to_string(),
                logo_uri: Some("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2_url".to_string()),
            },
        );

        map.insert(
            "0xBc79855178842FDBA0c353494895DEEf509E26bB".to_string(),
            AddressInfo {
                name: "0xBc79855178842FDBA0c353494895DEEf509E26bB_name".to_string(),
                logo_uri: Some("0xBc79855178842FDBA0c353494895DEEf509E26bB_url".to_string()),
            },
        );

        map
    };

    let actual = data_decoded.build_address_info_index(&mut mock_info_provider);

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn address_info_index_multi_send_two_levels_of_nesting() {
    let mut sequence = Sequence::new();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE"))
        .times(1)
        .return_once(move |address| {
            Ok(AddressInfo {
                name: format!("{}_name", &address),
                logo_uri: Some(format!("{}_url", &address)),
            })
        })
        .in_sequence(&mut sequence);

    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0x991c44331f0E59510Bcff76edBA06C3f552Eef8B"))
        .times(1)
        .return_once(move |address| {
            Ok(AddressInfo {
                name: format!("{}_name", &address),
                logo_uri: Some(format!("{}_url", &address)),
            })
        })
        .in_sequence(&mut sequence);

    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0x68881260bd04E9dAc7F77a314360ce05435B4818"))
        .times(1)
        .return_once(move |address| {
            Ok(AddressInfo {
                name: format!("{}_name", &address),
                logo_uri: Some(format!("{}_url", &address)),
            })
        })
        .in_sequence(&mut sequence);

    // Had to doctor the json in order to have different address in the nested calls and verify that we
    // don't call them, not because they are duplicate, but because they are 1 level further nested
    let data_decoded =
        serde_json::from_str::<DataDecoded>(crate::json::DOCTORED_DATA_DECODED_NESTED_MULTI_SENDS)
            .unwrap();

    let expected = {
        let mut map = HashMap::new();
        map.insert(
            "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE".to_string(),
            AddressInfo {
                name: "0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE_name".to_string(),
                logo_uri: Some("0xEeeeeEeeeEeEeeEeEeEeeEEEeeeeEeeeeeeeEEeE_url".to_string()),
            },
        );

        map.insert(
            "0x991c44331f0E59510Bcff76edBA06C3f552Eef8B".to_string(),
            AddressInfo {
                name: "0x991c44331f0E59510Bcff76edBA06C3f552Eef8B_name".to_string(),
                logo_uri: Some("0x991c44331f0E59510Bcff76edBA06C3f552Eef8B_url".to_string()),
            },
        );

        map.insert(
            "0x68881260bd04E9dAc7F77a314360ce05435B4818".to_string(),
            AddressInfo {
                name: "0x68881260bd04E9dAc7F77a314360ce05435B4818_name".to_string(),
                logo_uri: Some("0x68881260bd04E9dAc7F77a314360ce05435B4818_url".to_string()),
            },
        );

        map
    };

    let actual = data_decoded.build_address_info_index(&mut mock_info_provider);

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn address_info_index_skip_address_info_for_0x0() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0x441E604Ad49602c0B9C0B08D0781eCF96740786a"))
        .times(1)
        .return_once(move |address| {
            Ok(AddressInfo {
                name: format!("{}_name", &address),
                logo_uri: Some(format!("{}_url", &address)),
            })
        });

    let data_decoded = serde_json::from_str::<DataDecoded>(
        crate::json::DATA_DECODED_EXEC_TRANSACTION_WITH_VALUE_DECODED,
    )
    .unwrap();

    let expected = {
        let mut map = HashMap::new();
        map.insert(
            "0x441E604Ad49602c0B9C0B08D0781eCF96740786a".to_string(),
            AddressInfo {
                name: "0x441E604Ad49602c0B9C0B08D0781eCF96740786a_name".to_string(),
                logo_uri: Some("0x441E604Ad49602c0B9C0B08D0781eCF96740786a_url".to_string()),
            },
        );

        map
    };

    let actual = data_decoded.build_address_info_index(&mut mock_info_provider);

    assert_eq!(expected, actual.unwrap());
}

#[test]
fn address_info_index_no_results_returns_none() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_full_address_info_search()
        .with(eq("0x441E604Ad49602c0B9C0B08D0781eCF96740786a"))
        .times(1)
        .return_once(move |_| bail!("no address info"));

    let data_decoded = serde_json::from_str::<DataDecoded>(
        crate::json::DATA_DECODED_EXEC_TRANSACTION_WITH_VALUE_DECODED,
    )
    .unwrap();

    let expected = None;

    let actual = data_decoded.build_address_info_index(&mut mock_info_provider);

    assert_eq!(expected, actual);
}
