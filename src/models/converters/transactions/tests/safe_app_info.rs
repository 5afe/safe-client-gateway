#[test]
fn valid_full_origin_data() {
    let origin =
        "{\"url\":\"https://apps.gnosis-safe.io/walletConnect\",\"name\":\"WalletConnect\"}";
}

#[test]
fn valid_missing_name_origin_data() {
    let origin = "{\"url\":\"https://apps.gnosis-safe.io/walletConnect\"}";
}

#[test]
fn ellipsized_name_origin_data() {
    let origin =
        "{\"url\":\"https://apps.gnosis-safe.io/walletConnect\",\"name\":\"Walle...nect\"}";
}

#[test]
fn ellipsized_url_origin_data() {
    let origin = "{\"url\":\"https://apps.gnosis-safe.io/wallet...t\",\"name\":\"WalletConnect\"}";
}

#[test]
fn ellipsized_invalid_json_origin_data() {
    let origin =
        "{\"url\":\"https://apps.gnosis-safe.io/walletConnect\",...me\":\"WalletConnect\"}";
}
