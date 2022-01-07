use crate::common::models::backend::safe_apps::SafeApp as BackendSafeApp;
use crate::routes::safe_apps::models::{
    SafeApp, SafeAppAccessControlPolicies, SafeAppDomainAllowlistPolicy, SafeAppProvider,
};

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
    let response =
        serde_json::from_str::<Vec<BackendSafeApp>>(crate::tests::json::POLYGON_SAFE_APPS)
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
            provider: None,
            access_control: SafeAppAccessControlPolicies::NoRestrictions,
        },
        SafeApp {
            id: 24,
            url: "https://safe-apps.dev.gnosisdev.com/tx-builder".to_string(),
            name: "Transaction Builder".to_string(),
            icon_url: "https://safe-apps.dev.gnosisdev.com/tx-builder/tx-builder.png".to_string(),
            description: "A Safe app to compose custom transactions".to_string(),
            chain_ids: vec!["1".to_string(), "4".to_string(),"10".to_string(),"56".to_string(),"100".to_string(),"137".to_string(),"246".to_string(), "42161".to_string(), "43114".to_string(), "73799".to_string()],
            provider: None,
            access_control: SafeAppAccessControlPolicies::NoRestrictions,
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
            }),
            access_control: SafeAppAccessControlPolicies::DomainAllowlist(SafeAppDomainAllowlistPolicy {
                value: vec!["https://gnosis-safe.io".to_string(), "https://dev.gnosis-safe.io".to_string()],
            }),
        },
        SafeApp {
            id: 30,
            url: "https://paraswap.io".to_string(),
            name: "ParaSwap".to_string(),
            icon_url: "https://paraswap.io/paraswap.svg".to_string(),
            description: "ParaSwap allows dApps and traders to get the best DEX liquidity by aggregating multiple markets and offering the best rates".to_string(),
            chain_ids: vec!["1".to_string(),"56".to_string(),"137".to_string()],
            provider: None,
            access_control: SafeAppAccessControlPolicies::NoRestrictions,
        },
        SafeApp {
            id: 25,
            url: "https://safe-apps.dev.gnosisdev.com/wallet-connect".to_string(),
            name: "WalletConnect".to_string(),
            icon_url: "https://safe-apps.dev.gnosisdev.com/wallet-connect/wallet-connect.svg".to_string(),
            description: "Connect your Safe to any dApp that supports WalletConnect".to_string(),
            chain_ids: vec!["1".to_string(), "4".to_string(), "10".to_string(),"56".to_string(),"100".to_string(),"137".to_string(),"246".to_string(), "73799".to_string(), "42161".to_string(), "43114".to_string()],
            provider: None,
            access_control: SafeAppAccessControlPolicies::NoRestrictions,
        },
    ];

    assert_eq!(actual, expected);
}
