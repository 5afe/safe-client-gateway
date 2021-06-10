use crate::models::chains::{ChainInfo, NativeCurrency};
use crate::providers::info::*;
use crate::services::core_backend_url;

#[rocket::async_test]
async fn balances_backend_url_success() {
    let safe_address = "0x1230B3d59858296A31053C1b8562Ecf89A2f888b";
    let trusted = false;
    let exclude_spam = true;
    let chain_id = "1";
    let chain_info = ChainInfo {
        tx_service_url: "https://safe-transaction.mainnet.gnosis.io".to_string(),
        chain_id: "1".to_string(),
        chain_name: "".to_string(),
        rpc_url: "".to_string(),
        block_explorer_url: "".to_string(),
        native_currency: NativeCurrency {
            name: "".to_string(),
            symbol: "".to_string(),
            decimals: 0,
        },
    };
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_chain_info()
        .times(1)
        .return_once(move |_| Ok(chain_info));
    let url = core_backend_url(chain_id, mock_info_provider, || {
        format!(
            "/v1/safes/{}/balances/usd/?trusted={}&exclude_spam={}",
            safe_address, trusted, exclude_spam
        )
    })
    .await;

    assert_eq!(url.unwrap(),"https://safe-transaction.mainnet.gnosis.io/v1/safes/0x1230B3d59858296A31053C1b8562Ecf89A2f888b/balances/usd/?trusted=false&exclude_spam=true".to_string())
}

#[rocket::async_test]
#[should_panic]
async fn backend_url_error() {
    let mut mock_info_provider = MockInfoProvider::new();
    mock_info_provider
        .expect_chain_info()
        .times(1)
        .returning(move |_| bail!("Unsupported net"));
    let url = core_backend_url("1", mock_info_provider, || String::from("nice/path")).await;
    url.expect("An error");
}
