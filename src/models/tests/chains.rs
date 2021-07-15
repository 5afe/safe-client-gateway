use crate::models::backend::chains::{ChainInfo, NativeCurrency, Theme};
use crate::models::service::chains::{
    ChainInfo as ServiceChainInfo, NativeCurrency as ServiceNativeCurrency, Theme as ServiceTheme,
};

#[test]
fn chain_info_json() {
    let expected = ChainInfo {
        recommended_master_copy_version: "1.1.1".to_string(),
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        rpc_url: "https://someurl.com/rpc".to_string(),
        block_explorer_url: "https://blockexplorer.com".to_string(),
        native_currency: NativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_url: "https://test.token.image.url".to_string(),
        },
        theme: Theme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
    };

    let actual = serde_json::from_str::<ChainInfo>(crate::json::CHAIN_INFO_RINKEBY);

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_to_service_chain_info() {
    let expected = ServiceChainInfo {
        transaction_service: "https://safe-transaction.rinkeby.staging.gnosisdev.com".to_string(),
        chain_id: "4".to_string(),
        chain_name: "Rinkeby".to_string(),
        rpc_url: "https://someurl.com/rpc".to_string(),
        block_explorer_url: "https://blockexplorer.com".to_string(),
        native_currency: ServiceNativeCurrency {
            name: "Ether".to_string(),
            symbol: "ETH".to_string(),
            decimals: 18,
            logo_url: "https://test.token.image.url".to_string(),
        },
        theme: ServiceTheme {
            text_color: "#fff".to_string(),
            background_color: "#000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
    };

    let from_json = serde_json::from_str::<ChainInfo>(crate::json::CHAIN_INFO_RINKEBY).unwrap();
    let actual: ServiceChainInfo = from_json.into();

    assert_eq!(expected, actual);
}
