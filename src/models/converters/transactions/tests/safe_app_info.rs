use crate::models::converters::transactions::safe_app_info::OriginInternal;
use crate::models::service::transactions::summary::SafeAppInfo;

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
    let expected = OriginInternal {
        url: "https://apps.gnosis-safe.io/walletConnect".to_string(),
    };

    serde_json::from_str::<OriginInternal>(origin).unwrap();
}
