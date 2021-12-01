use crate::common::models::backend::chains::{
    ChainInfo as BackendChainInfo, GasPrice, RpcAuthentication, RpcUri as BackendRpcUri,
};
use crate::routes::chains::models::{
    BlockExplorerUriTemplate as ServiceBlockExplorerUriTemplate, ChainInfo as ServiceChainInfo,
    GasPrice as ServiceGasPrice, NativeCurrency as ServiceNativeCurrency,
    RpcAuthentication as ServiceRpcAuthentication, RpcUri as ServiceRpcUri, Theme as ServiceTheme,
};

impl From<BackendChainInfo> for ServiceChainInfo {
    fn from(chain_info: BackendChainInfo) -> Self {
        Self {
            transaction_service: chain_info.transaction_service,
            chain_id: chain_info.chain_id,
            chain_name: chain_info.chain_name,
            short_name: chain_info.short_name,
            l2: chain_info.l2,
            description: chain_info.description,
            rpc_uri: ServiceRpcUri::from(chain_info.rpc_uri),
            safe_apps_rpc_uri: ServiceRpcUri::from(chain_info.safe_apps_rpc_uri),
            public_rpc_uri: ServiceRpcUri::from(chain_info.public_rpc_uri),
            block_explorer_uri_template: ServiceBlockExplorerUriTemplate {
                address: chain_info.block_explorer_uri_template.address,
                tx_hash: chain_info.block_explorer_uri_template.tx_hash,
                api: chain_info.block_explorer_uri_template.api,
            },
            native_currency: ServiceNativeCurrency {
                name: chain_info.native_currency.name,
                symbol: chain_info.native_currency.symbol,
                decimals: chain_info.native_currency.decimals,
                logo_uri: chain_info.native_currency.logo_uri,
            },
            theme: ServiceTheme {
                text_color: chain_info.theme.text_color,
                background_color: chain_info.theme.background_color,
            },
            ens_registry_address: chain_info.ens_registry_address,
            gas_price: chain_info
                .gas_price
                .iter()
                .map(|gas_price| match gas_price {
                    GasPrice::Oracle {
                        uri,
                        gas_parameter,
                        gwei_factor,
                    } => ServiceGasPrice::Oracle {
                        uri: uri.to_string(),
                        gas_parameter: gas_parameter.to_string(),
                        gwei_factor: gwei_factor.to_string(),
                    },
                    GasPrice::Fixed { wei_value } => ServiceGasPrice::Fixed {
                        wei_value: wei_value.to_string(),
                    },
                    GasPrice::Unknown => ServiceGasPrice::Unknown,
                })
                .collect::<Vec<ServiceGasPrice>>(),
            disabled_wallets: chain_info.disabled_wallets,
            features: chain_info.features,
        }
    }
}

impl From<BackendRpcUri> for ServiceRpcUri {
    fn from(rpc_uri: BackendRpcUri) -> Self {
        ServiceRpcUri {
            authentication: match rpc_uri.authentication {
                RpcAuthentication::ApiKeyPath => ServiceRpcAuthentication::ApiKeyPath,
                RpcAuthentication::NoAuthentication => ServiceRpcAuthentication::NoAuthentication,
                RpcAuthentication::Unknown => ServiceRpcAuthentication::Unknown,
            },
            value: rpc_uri.value,
        }
    }
}
