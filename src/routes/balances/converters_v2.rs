use crate::common::models::backend::balances_v2::Balance as BalanceDto;
use crate::common::models::backend::chains::NativeCurrency;
use crate::providers::info::{TokenInfo, TokenType};
use crate::routes::balances::models::Balance;
use bigdecimal::num_bigint::BigInt;
use bigdecimal::{BigDecimal, ToPrimitive, Zero};
use std::str::FromStr;

impl BalanceDto {
    pub fn to_balance_v2(
        &self,
        token_to_usd: &BigDecimal,
        usd_to_fiat: &BigDecimal,
        native_coin: &NativeCurrency,
    ) -> Balance {
        let token_decimals = self
            .token
            .as_ref()
            .and_then(|token| Some(token.decimals))
            .and_then(|decimals| decimals.to_i64())
            .unwrap_or(native_coin.decimals.to_i64().unwrap());

        let balance = BigInt::from_str(&self.balance).unwrap_or(Zero::zero());
        let token_balance = BigDecimal::new(balance, token_decimals);
        let fiat_conversion = token_to_usd * usd_to_fiat;
        let fiat_balance = (token_balance * token_to_usd * usd_to_fiat).with_scale(5);

        let (token_type, logo_uri) = if self.token_address.is_some() {
            (
                TokenType::Erc20,
                self.token.as_ref().map(|it| it.logo_uri.to_string()),
            )
        } else {
            (
                TokenType::NativeToken,
                Some(native_coin.logo_uri.to_string()),
            )
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
