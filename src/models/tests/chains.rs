use crate::models::backend::chains::{ChainInfo, GasPrice, NativeCurrency, Theme};
use crate::models::service::chains::{
    ChainInfo as ServiceChainInfo, GasPrice as ServiceGasPrice,
    NativeCurrency as ServiceNativeCurrency, Theme as ServiceTheme,
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
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: GasPrice::Oracle {
            url: "https://gaspriceoracle.com/".to_string(),
            gas_parameter: "average".to_string(),
            gwei_factor: "10".to_string(),
        },
    };

    let actual = serde_json::from_str::<ChainInfo>(crate::json::CHAIN_INFO_RINKEBY);

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_with_fixed_gas_price() {
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
        gas_price: GasPrice::Fixed {
            wei_value: "1000000000".to_string(),
        },
    };

    let actual = serde_json::from_str::<ChainInfo>(crate::json::CHAIN_INFO_RINKEBY_FIXED_GAS_PRICE);

    assert!(actual.is_ok());
    assert_eq!(expected, actual.unwrap());
}

#[test]
fn chain_info_json_with_unkown_gas_price_type() {
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
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: GasPrice::Unknown,
    };

    let actual =
        serde_json::from_str::<ChainInfo>(crate::json::CHAIN_INFO_RINKEBY_UNKNOWN_GAS_PRICE);

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
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: ServiceGasPrice::Oracle {
            url: "https://gaspriceoracle.com/".to_string(),
            gas_parameter: "average".to_string(),
            gwei_factor: "10".to_string(),
        },
    };

    let from_json = serde_json::from_str::<ChainInfo>(crate::json::CHAIN_INFO_RINKEBY).unwrap();
    let actual: ServiceChainInfo = from_json.into();

    assert_eq!(expected, actual);
}

#[test]
fn unknown_gas_price_type_to_service_chain_info() {
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
            text_color: "#ffffff".to_string(),
            background_color: "#000000".to_string(),
        },
        ens_registry_address: Some("0xFFfFfFffFFfffFFfFFfFFFFFffFFFffffFfFFFfF".to_string()),
        gas_price: ServiceGasPrice::Unknown,
    };

    let from_json =
        serde_json::from_str::<ChainInfo>(crate::json::CHAIN_INFO_RINKEBY_UNKNOWN_GAS_PRICE)
            .unwrap();
    let actual: ServiceChainInfo = from_json.into();

    assert_eq!(expected, actual);
}
