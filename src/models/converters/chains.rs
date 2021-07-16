use crate::models::backend::chains::{ChainInfo as BackendChainInfo, GasPrice};
use crate::models::service::chains::{
    ChainInfo as ServiceChainInfo, GasPrice as ServiceGasPrice,
    NativeCurrency as ServiceNativeCurrency, Theme as ServiceTheme,
};

impl From<BackendChainInfo> for ServiceChainInfo {
    fn from(chain_info: BackendChainInfo) -> Self {
        Self {
            transaction_service: chain_info.transaction_service,
            chain_id: chain_info.chain_id,
            chain_name: chain_info.chain_name,
            rpc_uri: chain_info.rpc_uri,
            block_explorer_uri: chain_info.block_explorer_uri,
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
            gas_price: match chain_info.gas_price {
                GasPrice::Oracle {
                    uri,
                    gas_parameter,
                    gwei_factor,
                } => ServiceGasPrice::Oracle {
                    uri,
                    gas_parameter,
                    gwei_factor,
                },
                GasPrice::Fixed { wei_value } => ServiceGasPrice::Fixed { wei_value },
                GasPrice::Unknown => ServiceGasPrice::Unknown,
            },
        }
    }
}
