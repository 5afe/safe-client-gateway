use crate::models::converters::transactions::safe_app_info::{safe_app_info_from, OriginInternal};
use crate::models::service::transactions::summary::SafeAppInfo;
use crate::providers::info::*;
use crate::utils::errors::{ApiError, ErrorDetails};

#[test]
fn valid_full_origin_data() {
    let origin =
        "{\"url\":\"https://apps.gnosis-safe.io/walletConnect\",\"name\":\"WalletConnect\"}";

    let expected = OriginInternal {
        url: "https://apps.gnosis-safe.io/walletConnect".to_string(),
    };

    let actual = serde_json::from_str::<OriginInternal>(origin).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn valid_missing_name_origin_data() {
    let origin = "{\"url\":\"https://apps.gnosis-safe.io/walletConnect\"}";

    let expected = OriginInternal {
        url: "https://apps.gnosis-safe.io/walletConnect".to_string(),
    };

    let actual = serde_json::from_str::<OriginInternal>(origin).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn ellipsized_name_origin_data() {
    let origin =
        "{\"url\":\"https://apps.gnosis-safe.io/walletConnect\",\"name\":\"Walle...nect\"}";

    let expected = OriginInternal {
        url: "https://apps.gnosis-safe.io/walletConnect".to_string(),
    };

    let actual = serde_json::from_str::<OriginInternal>(origin).unwrap();

    assert_eq!(expected, actual);
}

#[test]
fn ellipsized_url_origin_data() {
    let origin = "{\"url\":\"https://apps.gnosis-safe.io/wallet...t\",\"name\":\"WalletConnect\"}";

    let expected = OriginInternal {
        url: "https://apps.gnosis-safe.io/wallet...t".to_string(),
    };

    let actual = serde_json::from_str::<OriginInternal>(origin).unwrap();

    assert_eq!(expected, actual);
}

#[test]
#[should_panic]
fn ellipsized_invalid_json_origin_data() {
    let origin =
        "{\"url\":\"https://apps.gnosis-safe.io/walletConnect\",...me\":\"WalletConnect\"}";

    serde_json::from_str::<OriginInternal>(origin).unwrap();
}

#[test]
fn to_safe_app_info_bad_url() {
    let origin = "{\"url\":\"https://apps.gnosis-safe.io/walletConnect\"}";
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_raw_request()
        .times(1)
        .return_once(move |_| {
            Err(ApiError {
                status: 404,
                details: ErrorDetails {
                    code: 42,
                    message: None,
                    arguments: None,
                },
            })
        });

    let actual = safe_app_info_from(origin, &mut mock_info_provider);
    assert!(actual.is_none());
}

#[test]
fn to_safe_app_info_correct() {
    let origin = "{\"url\":\"https://apps.gnosis-safe.io/walletConnect\"}";
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_raw_request()
        .times(1)
        .return_once(move |_| {
            Ok(json!({
                "name": "WalletConnect".to_string(),
                "description":
                    "Allows your Gnosis Safe Multisig to connect to dapps via WalletConnect."
                        .to_string(),
                "iconPath": "walletConnect.jpg".to_string(),
            })
            .to_string())
        });

    let expected = SafeAppInfo {
        name: "WalletConnect".to_string(),
        url: "https://apps.gnosis-safe.io/walletConnect".to_string(),
        logo_url: "https://apps.gnosis-safe.io/walletConnect/walletConnect.jpg".to_string(),
    };

    let actual = safe_app_info_from(origin, &mut mock_info_provider);
    assert!(actual.is_some());
    assert_eq!(expected, actual.unwrap());
}
