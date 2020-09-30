use crate::utils::errors::ApiResult;
use crate::models::service::balances::Balance;
use crate::models::backend::balances::Balance as BalanceDto;
use crate::providers::info::{TokenInfo, TokenType};

impl BalanceDto {
    pub fn to_balance(&self, conversion_rate: u64) -> ApiResult<Balance> {
        Ok(Balance {
            token_info: TokenInfo {
                token_type: TokenType::Erc20,
                address: self.token_address.to_owned(),
                decimals: self.token.decimals,
                symbol: self.token.symbol.to_owned(),
                name: self.token.name.to_owned(),
                logo_uri: Some(self.token.logo_uri.to_owned()),
            },
            amount: self.balance.to_owned(),
            fiat_amount: self.balance_usd.to_string(),
        })
    }
}
