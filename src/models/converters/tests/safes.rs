use crate::models::service::safes::{AddressEx, SafeInfoEx};
use crate::providers::address_info::AddressInfo;
use crate::providers::info::*;

#[test]
fn to_safe_info_ex_no_address_info() {
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_contract_info()
        .times(5)
        .returning(move |_| bail!("No safe info"));
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
        version: Some("1.1.1".to_string()),
    };

    let actual = safe_info.to_safe_info_ex(&mut mock_info_provider);

    assert_eq!(actual, expected);
}

#[test]
fn to_safe_info_ex_address_info() {
    let safe_info = serde_json::from_str::<SafeInfo>(crate::json::SAFE_WITH_MODULES).unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_contract_info()
        .times(5)
        .returning(move |address| {
            Ok(AddressInfo {
                name: format!("name_{}", &address),
                logo_uri: Some(format!("logo_uri_{}", &address)),
            })
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
        version: Some("1.1.1".to_string()),
    };

    let actual = safe_info.to_safe_info_ex(&mut mock_info_provider);

    assert_eq!(actual, expected);
}

#[test]
fn to_safe_info_ex_nullable_fields_are_all_null() {
    let safe_info = serde_json::from_str::<SafeInfo>(
        &json!({
            "address": "0x1230B3d59858296A31053C1b8562Ecf89A2f888b",
            "nonce" : 180,
            "threshold" : 3,
            "owners" : ["0xBEA2F9227230976d2813a2f8b922c22bE1DE1B23"],
            "masterCopy": "0x34CfAC646f301356fAa8B21e94227e3583Fe3F5F",
        })
        .to_string(),
    )
    .unwrap();
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_contract_info()
        .times(1)
        .return_once(move |_| bail!("No address info"));

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
        version: None,
    };

    let actual = safe_info.to_safe_info_ex(&mut mock_info_provider);

    assert_eq!(expected, actual);
}
