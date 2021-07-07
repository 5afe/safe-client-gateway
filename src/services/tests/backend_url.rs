use crate::models::chains::{ChainInfo, NativeCurrency, Theme};
use crate::providers::info::*;
use crate::utils::errors::ApiResult;

#[rocket::async_test]
async fn core_uri_success_with_params() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let trusted = false;
    let exclude_spam = true;
    let chain_info = ChainInfo {
        transaction_service: "https://safe-transaction.mainnet.gnosis.io".to_string(),
        chain_id: "1".to_string(),
        chain_name: "".to_string(),
        rpc_url: "".to_string(),
        block_explorer_url: "".to_string(),
        native_currency: NativeCurrency {
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0,
            logo_url: "https://test.token.image.url".to_string(),
        },
        theme: Theme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
    };
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_chain_info()
        .times(1)
        .return_once(move || Ok(chain_info));
    let url = core_uri!(
        mock_info_provider,
        "/v1/safes/{}/balances/usd/?trusted={}&exclude_spam={}",
        safe_address,
        trusted,
        exclude_spam
    );

    assert_eq!(url.unwrap(), "https://safe-transaction.mainnet.gnosis.io/api/v1/safes/0x1230B3d59858296A31053C1b8562Ecf89A2f888b/balances/usd/?trusted=false&exclude_spam=true".to_string());
}

#[rocket::async_test]
async fn core_uri_success_without_params() {
    let chain_info = ChainInfo {
        transaction_service: "https://safe-transaction.mainnet.gnosis.io".to_string(),
        chain_id: "1".to_string(),
        chain_name: "".to_string(),
        rpc_url: "".to_string(),
        block_explorer_url: "".to_string(),
        native_currency: NativeCurrency {
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0,
            logo_url: "https://test.token.image.url".to_string(),
        },
        theme: Theme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
    };
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_chain_info()
        .times(1)
        .return_once(move || Ok(chain_info));
    let url = core_uri!(mock_info_provider, "/some/path");

    assert_eq!(
        "https://safe-transaction.mainnet.gnosis.io/api/some/path",
        url.unwrap()
    );
}

#[rocket::async_test]
#[should_panic]
async fn core_uri_error() {
    let mock_info_provider = MockInfoProvider::new();

    let url = core_uri!(mock_info_provider, "/nice/path");
    url.unwrap();
}
