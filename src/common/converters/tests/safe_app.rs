use crate::routes::safe_apps::backend::SafeApp as BackendSafeApp;
use crate::routes::safe_apps::models::{SafeApp, SafeAppProvider};

#[test]
fn safe_apps_empty() {
    let backend_json = "[]";
    let expected: Vec<SafeApp> = vec![];
    let response = serde_json::from_str::<Vec<BackendSafeApp>>(backend_json)
        .expect("SafeApp deserialization failed");

    let actual: Vec<SafeApp> = response
        .into_iter()
        .map(|safe_app| safe_app.into())
        .collect();

    assert_eq!(expected, actual);
}

#[test]
fn safe_apps_several_apps() {
    let response = serde_json::from_str::<Vec<BackendSafeApp>>(crate::json::POLYGON_SAFE_APPS)
        .expect("SafeApps deserialization failure");

    let actual = response
        .into_iter()
        .map(|safe_app| safe_app.into())
        .collect::<Vec<SafeApp>>();

    let expected = vec![
        SafeApp {
            id: 26,
            url: "https://curve.fi".to_string(),
            name: "Curve".to_string(),
            icon_url: "https://curve.fi/logo-square.svg".to_string(),
            description: "Decentralized exchange liquidity pool designed for extremely efficient stablecoin trading and low-risk income for liquidity providers".to_string(),
            chain_ids: vec!["1".to_string(), "137".to_string()],
            provider: None
        },
        SafeApp {
            id: 24,
            url: "https://cloudflare-ipfs.com/ipfs/QmdVaZxDov4bVARScTLErQSRQoxgqtBad8anWuw3YPQHCs".to_string(),
            name: "Transaction Builder".to_string(),
            icon_url: "https://cloudflare-ipfs.com/ipfs/QmdVaZxDov4bVARScTLErQSRQoxgqtBad8anWuw3YPQHCs/tx-builder.png".to_string(),
            description: "A Safe app to compose custom transactions".to_string(),
            chain_ids: vec!["1".to_string(), "4".to_string(),"56".to_string(),"100".to_string(),"137".to_string(),"246".to_string(), "73799".to_string()],
            provider: None
        },
        SafeApp {
            id: 11,
            url: "https://app.1inch.io".to_string(),
            name: "1inch.exchange".to_string(),
            icon_url: "https://app.1inch.io/assets/images/1inch.svg".to_string(),
            description: "The most efficient defi aggregator".to_string(),
            chain_ids: vec!["1".to_string(),"56".to_string(),"137".to_string()],
            provider: Some(SafeAppProvider {
                url: "https://1inch.exchange".to_string(),
                name: "1inch corporation".to_string(),
            })
        },
        SafeApp {
            id: 30,
            url: "https://paraswap.io".to_string(),
            name: "ParaSwap".to_string(),
            icon_url: "https://paraswap.io/paraswap.svg".to_string(),
            description: "ParaSwap allows dApps and traders to get the best DEX liquidity by aggregating multiple markets and offering the best rates".to_string(),
            chain_ids: vec!["1".to_string(),"56".to_string(),"137".to_string()],
            provider: None
        },
        SafeApp {
            id: 25,
            url: "https://cloudflare-ipfs.com/ipfs/QmTpLhxSiD1H94BFxeV2P6RfJf6EyCxxUCVYpcDffyMmmZ".to_string(),
            name: "WalletConnect".to_string(),
            icon_url: "https://cloudflare-ipfs.com/ipfs/QmTpLhxSiD1H94BFxeV2P6RfJf6EyCxxUCVYpcDffyMmmZ/wallet-connect.svg".to_string(),
            description: "Allows your Gnosis Safe Multisig to connect to dapps via WalletConnect.".to_string(),
            chain_ids: vec!["1".to_string(), "4".to_string(),"56".to_string(),"100".to_string(),"137".to_string(),"246".to_string(), "73799".to_string()],
            provider: None
        },
    ];

    assert_eq!(actual, expected);
}
