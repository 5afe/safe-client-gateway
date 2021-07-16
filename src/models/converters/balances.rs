use crate::models::backend::balances::Balance as BalanceDto;
use crate::models::backend::chains::NativeCurrency;
use crate::models::service::balances::Balance;
use crate::providers::info::{TokenInfo, TokenType};

impl BalanceDto {
    pub fn to_balance(&self, usd_to_fiat: f64, native_coin: &NativeCurrency) -> Balance {
        let fiat_conversion = self.fiat_conversion.parse::<f64>().unwrap_or(0.0) * usd_to_fiat;
        let fiat_balance = self.fiat_balance.parse::<f64>().unwrap_or(0.0) * usd_to_fiat;
        let token_type = self
            .token_address
            .as_ref()
            .map(|_| TokenType::Erc20)
            .unwrap_or(TokenType::NativeToken);

        let logo_uri = if token_type == TokenType::NativeToken {
            Some(native_coin.logo_uri.to_string())
        } else {
            self.token.as_ref().map(|it| it.logo_uri.to_string())
        };
        Balance {
            token_info: TokenInfo {
                token_type,
                address: self
                    .token_address
                    .to_owned()
                    .unwrap_or(String::from("0x0000000000000000000000000000000000000000")),
                decimals: self
                    .token
                    .as_ref()
                    .map(|it| it.decimals)
                    .unwrap_or(native_coin.decimals),
                symbol: self
                    .token
                    .as_ref()
                    .map(|it| it.symbol.to_string())
                    .unwrap_or(native_coin.symbol.to_string()),
                name: self
                    .token
                    .as_ref()
                    .map(|it| it.name.to_string())
                    .unwrap_or(native_coin.name.to_string()),
                logo_uri,
            },
            balance: self.balance.to_owned(),
            fiat_balance: fiat_balance.to_string(),
            fiat_conversion: fiat_conversion.to_string(),
        }
    }
}
