use crate::models::backend::chains::ChainInfo;
use crate::models::converters::safes::calculate_version_state;
use crate::models::service::addresses::AddressEx;
use crate::models::service::safes::{ImplementationVersionState, SafeInfoEx};
use crate::providers::info::*;
use rocket::serde::json::json;

#[rocket::async_test]
async fn to_safe_info_ex_no_address_info() {
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_add_address_info_from_contract_info()
        .times(5)
        .returning(move |_| bail!("No safe info"));
    mock_info_provider
        .expect_chain_info()
        .times(1)
        .returning(move || bail!("No chain info"));

    let expected = SafeInfoEx {
        address: AddressEx {
            value: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
            name: None,
            logo_url: None,
        },
        nonce: 180,
        threshold: 3,
        owners: vec![
            AddressEx {
                value: "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(),
                name: None,
                logo_url: None,
            },
        ],
        implementation: AddressEx {
            value: "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F".to_string(),
            name: None,
            logo_url: None,
        },
        modules: Some(vec![
            AddressEx {
                value: "0x25F73b24B866963B0e560fFF9bbA7908be0263E8".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0x10A7EC8D10CD175dC33781fB9Cf3394220Fac78c".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0xF5dC3718EEbC5b003F1672A499F2ACBE77Ba790d".to_string(),
                name: None,
                logo_url: None,
            },
        ]),
        fallback_handler: Some(AddressEx {
            value: "0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44".to_string(),
            name: None,
            logo_url: None,
        }),
        guard: None,
        version: Some("1.1.1".to_string()),
        implementation_version_state: ImplementationVersionState::Unknown,
    };

    let actual = safe_info.to_safe_info_ex(&mut mock_info_provider).await;

    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn to_safe_info_ex_no_address_info_up_to_date() {
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_add_address_info_from_contract_info()
        .times(5)
        .returning(move |_| bail!("No safe info"));
    mock_info_provider
        .expect_chain_info()
        .times(1)
        .returning(move || {
            Ok(serde_json::from_str::<ChainInfo>(crate::json::CHAIN_INFO_RINKEBY).unwrap())
        });

    let expected = SafeInfoEx {
        address: AddressEx {
            value: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
            name: None,
            logo_url: None,
        },
        nonce: 180,
        threshold: 3,
        owners: vec![
            AddressEx {
                value: "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(),
                name: None,
                logo_url: None,
            },
        ],
        implementation: AddressEx {
            value: "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F".to_string(),
            name: None,
            logo_url: None,
        },
        modules: Some(vec![
            AddressEx {
                value: "0x25F73b24B866963B0e560fFF9bbA7908be0263E8".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0x10A7EC8D10CD175dC33781fB9Cf3394220Fac78c".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0xF5dC3718EEbC5b003F1672A499F2ACBE77Ba790d".to_string(),
                name: None,
                logo_url: None,
            },
        ]),
        fallback_handler: Some(AddressEx {
            value: "0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44".to_string(),
            name: None,
            logo_url: None,
        }),
        guard: None,
        version: Some("1.1.1".to_string()),
        implementation_version_state: ImplementationVersionState::UpToDate,
    };

    let actual = safe_info.to_safe_info_ex(&mut mock_info_provider).await;

    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn to_safe_info_ex_address_info() {
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_add_address_info_from_contract_info()
        .times(5)
        .returning(move |address| {
            Ok(AddressEx {
                value: address.to_string(),
                name: Some(format!("name_{}", &address)),
                logo_url: Some(format!("logo_uri_{}", &address)),
            })
        });
    mock_info_provider
        .expect_chain_info()
        .times(1)
        .returning(move || bail!("No chain info"));

    let expected = SafeInfoEx {
        address: AddressEx {
            value: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
            name: None,
            logo_url: None,
        },
        nonce: 180,
        threshold: 3,
        owners: vec![
            AddressEx {
                value: "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0x37e9F140A9Df5DCBc783C6c220660a4E15CBFe72".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0xA3DAa0d9Ae02dAA17a664c232aDa1B739eF5ae8D".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0xF2CeA96575d6b10f51d9aF3b10e3e4E5738aa6bd".to_string(),
                name: None,
                logo_url: None,
            },
            AddressEx {
                value: "0x65F8236309e5A99Ff0d129d04E486EBCE20DC7B0".to_string(),
                name: None,
                logo_url: None,
            },
        ],
        implementation: AddressEx {
            value: "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F".to_string(),
            name: Some("name_0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F".to_string()),
            logo_url: Some("logo_uri_0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F".to_string()),
        },
        modules: Some(vec![
            AddressEx {
                value: "0x25F73b24B866963B0e560fFF9bbA7908be0263E8".to_string(),
                name: Some("name_0x25F73b24B866963B0e560fFF9bbA7908be0263E8".to_string()),
                logo_url: Some("logo_uri_0x25F73b24B866963B0e560fFF9bbA7908be0263E8".to_string()),
            },
            AddressEx {
                value: "0x10A7EC8D10CD175dC33781fB9Cf3394220Fac78c".to_string(),
                name: Some("name_0x10A7EC8D10CD175dC33781fB9Cf3394220Fac78c".to_string()),
                logo_url: Some("logo_uri_0x10A7EC8D10CD175dC33781fB9Cf3394220Fac78c".to_string()),
            },
            AddressEx {
                value: "0xF5dC3718EEbC5b003F1672A499F2ACBE77Ba790d".to_string(),
                name: Some("name_0xF5dC3718EEbC5b003F1672A499F2ACBE77Ba790d".to_string()),
                logo_url: Some("logo_uri_0xF5dC3718EEbC5b003F1672A499F2ACBE77Ba790d".to_string()),
            },
        ]),
        fallback_handler: Some(AddressEx {
            value: "0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44".to_string(),
            name: Some("name_0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44".to_string()),
            logo_url: Some("logo_uri_0xd5D82B6aDDc9027B22dCA772Aa68D5d74cdBdF44".to_string()),
        }),
        guard: None,
        version: Some("1.1.1".to_string()),
        implementation_version_state: ImplementationVersionState::Unknown,
    };

    let actual = safe_info.to_safe_info_ex(&mut mock_info_provider).await;

    assert_eq!(actual, expected);
}

#[rocket::async_test]
async fn to_safe_info_ex_nullable_fields_are_all_null() {
    let safe_info = serde_json::from_str::<SafeInfo>(
        &json!({
            "address": "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
            "nonce" : 180,
            "threshold" : 3,
            "owners" : ["0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23"],
            "masterCopy": "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F",
            "fallbackHandler": "0x0000000000000000000000000000000000000000",
            "guard": "0x0000000000000000000000000000000000000000"
        })
        .to_string(),
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_add_address_info_from_contract_info()
        .times(1)
        .return_once(move |_| bail!("No address info"));
    mock_info_provider
        .expect_chain_info()
        .times(1)
        .returning(move || bail!("No chain info"));

    let expected = SafeInfoEx {
        address: AddressEx {
            value: "0x1230B3d59858296A31053C1b8562Ecf89A2f888b".to_string(),
            name: None,
            logo_url: None,
        },
        nonce: 180,
        threshold: 3,
        owners: vec![AddressEx {
            value: "0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23".to_string(),
            name: None,
            logo_url: None,
        }],
        implementation: AddressEx {
            value: "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F".to_string(),
            name: None,
            logo_url: None,
        },
        modules: None,
        fallback_handler: None,
        guard: None,
        version: None,
        implementation_version_state: ImplementationVersionState::Unknown,
    };

    let actual = safe_info.to_safe_info_ex(&mut mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[rocket::async_test]
async fn to_safe_info_guard_and_fallback_handler_defined() {
    let safe_info =
        serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_GUARD_SAFE_V130).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_add_address_info_from_contract_info()
        .times(3)
        .returning(move |address| {
            Ok(AddressEx {
                value: address.to_string(),
                name: Some(format!("name_{}", &address)),
                logo_url: Some(format!("logo_uri_{}", &address)),
            })
        });
    mock_info_provider
        .expect_chain_info()
        .times(1)
        .returning(move || bail!("No chain info"));

    let expected = SafeInfoEx {
        address: AddressEx {
            value: "0x4cb09344de5bCCD45F045c5Defa0E0452869FF0f".to_string(),
            name: None,
            logo_url: None,
        },
        nonce: 7,
        threshold: 1,
        owners: vec![AddressEx {
            value: "0x5aC255889882aCd3da2aA939679E3f3d4cea221e".to_string(),
            name: None,
            logo_url: None,
        }],
        implementation: AddressEx {
            value: "0x3E5c63644E683549055b9Be8653de26E0B4CD36E".to_string(),
            name: Some("name_0x3E5c63644E683549055b9Be8653de26E0B4CD36E".to_string()),
            logo_url: Some("logo_uri_0x3E5c63644E683549055b9Be8653de26E0B4CD36E".to_string()),
        },
        modules: None,
        fallback_handler: Some(AddressEx {
            value: "0xf48f2B2d2a534e402487b3ee7C18c33Aec0Fe5e4".to_string(),
            name: Some("name_0xf48f2B2d2a534e402487b3ee7C18c33Aec0Fe5e4".to_string()),
            logo_url: Some("logo_uri_0xf48f2B2d2a534e402487b3ee7C18c33Aec0Fe5e4".to_string()),
        }),
        guard: Some(AddressEx {
            value: "0x40A2aCCbd92BCA938b02010E17A5b8929b49130D".to_string(),
            name: Some("name_0x40A2aCCbd92BCA938b02010E17A5b8929b49130D".to_string()),
            logo_url: Some("logo_uri_0x40A2aCCbd92BCA938b02010E17A5b8929b49130D".to_string()),
        }),
        version: Some("1.3.0".to_string()),
        implementation_version_state: ImplementationVersionState::Unknown,
    };

    let actual = safe_info.to_safe_info_ex(&mock_info_provider).await;

    assert_eq!(expected, actual);
}

#[test]
fn calculate_version_state_up_to_date() {
    let actual_equal = calculate_version_state("1.1.1", &Some("1.1.1".to_string()));
    let actual_newer = calculate_version_state("1.3.0", &Some("1.1.1".to_string()));

    assert_eq!(actual_equal, ImplementationVersionState::UpToDate);
    assert_eq!(actual_newer, ImplementationVersionState::UpToDate);
}

#[test]
fn calculate_version_state_outdated() {
    let actual = calculate_version_state("1.1.1", &Some("1.3.0".to_string()));

    assert_eq!(actual, ImplementationVersionState::Outdated);
}

#[test]
fn calculate_version_state_unknown() {
    let actual = calculate_version_state("1.1.1", &None);

    assert_eq!(actual, ImplementationVersionState::Unknown);
}
