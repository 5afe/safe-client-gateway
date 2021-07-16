use crate::models::converters::transactions::safe_app_info::{safe_app_info_from, OriginInternal};
use crate::providers::info::*;
use mockall::predicate::eq;

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

#[rocket::async_test]
async fn to_safe_app_info_bad_url() {
    let origin = "{\"url\":\"https://apps.gnosis-safe.io/walletConnect\"}";
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_app_info()
        .times(1)
        .return_once(move |_| bail!("Some http error"));

    let actual = safe_app_info_from(origin, &mut mock_info_provider).await;
    assert!(actual.is_none());
}

#[rocket::async_test]
async fn to_safe_app_info_correct() {
    let origin = "{\"url\":\"https://apps.gnosis-safe.io/walletConnect\"}";
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_app_info()
        .times(1)
        .return_once(move |_| {
            Ok(SafeAppInfo {
                name: "WalletConnect".to_string(),
                url: "https://apps.gnosis-safe.io/walletConnect".to_string(),
                logo_uri: "https://apps.gnosis-safe.io/walletConnect/walletConnect.jpg".to_string(),
            })
        });

    let expected = SafeAppInfo {
        name: "WalletConnect".to_string(),
        url: "https://apps.gnosis-safe.io/walletConnect".to_string(),
        logo_uri: "https://apps.gnosis-safe.io/walletConnect/walletConnect.jpg".to_string(),
    };

    let actual = safe_app_info_from(origin, &mut mock_info_provider).await;
    assert!(actual.is_some());
    assert_eq!(expected, actual.unwrap());
}

#[rocket::async_test]
async fn valid_ipfs_origin_gets_replaced() {
    let origin =
        "{\"url\":\"https://ipfs.io/ipfs/QmRWtuktjfU6WMAEJFgzBC4cUfqp3FF5uN9QoWb55SdGG5/manifest.json\",\"name\":\"WalletConnect\"}";
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_safe_app_info()
        .times(1)
        .with(eq("https://cloudflare-ipfs.com/ipfs/QmRWtuktjfU6WMAEJFgzBC4cUfqp3FF5uN9QoWb55SdGG5/manifest.json"))
        .return_once(move |_| {
            Ok(SafeAppInfo {
                name: "WalletConnect".to_string(),
                url: "https://ipfs.io/ipfs/QmRWtuktjfU6WMAEJFgzBC4cUfqp3FF5uN9QoWb55SdGG5/walletConnect".to_string(),
                logo_uri: "https://ipfs.io/ipfs/QmRWtuktjfU6WMAEJFgzBC4cUfqp3FF5uN9QoWb55SdGG5/walletConnect/walletConnect.jpg".to_string(),
            })
        });

    let expected = SafeAppInfo {
        name: "WalletConnect".to_string(),
        url: "https://ipfs.io/ipfs/QmRWtuktjfU6WMAEJFgzBC4cUfqp3FF5uN9QoWb55SdGG5/walletConnect".to_string(),
        logo_uri: "https://ipfs.io/ipfs/QmRWtuktjfU6WMAEJFgzBC4cUfqp3FF5uN9QoWb55SdGG5/walletConnect/walletConnect.jpg".to_string(),
    };

    let actual = safe_app_info_from(origin, &mut mock_info_provider).await;
    assert!(actual.is_some());
    assert_eq!(expected, actual.unwrap());
}
