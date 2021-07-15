use crate::models::service::addresses::AddressEx;
use crate::providers::info::{InfoProvider, TokenInfo};
use rocket::futures::future::OptionFuture;

// Using the pattern here:
// use rocket::futures::stream::StreamExt;
impl<T: ?Sized> InfoProviderExt for T where T: InfoProvider {}

#[rocket::async_trait]
pub trait InfoProviderExt: InfoProvider {
    async fn address_to_token_info(&self, address: &Option<String>) -> Option<TokenInfo> {
        let address = address.as_ref()?;
        self.token_info(address).await.ok()
    }

    async fn address_ex_from_contracts_or_default(&self, address: &String) -> AddressEx {
        self.address_ex_from_contracts(&address)
            .await
            .unwrap_or(AddressEx::address_only(address))
    }

    async fn multiple_address_ex_from_contracts(
        &self,
        addresses: &Option<Vec<String>>,
    ) -> Option<Vec<AddressEx>> {
        let addresses = addresses.as_ref()?;
        if addresses.is_empty() {
            return None;
        }
        let mut results = Vec::with_capacity(addresses.len());
        for address in addresses {
            results.push(
                self.address_ex_from_contracts_or_default(address)
                    .await,
            )
        }
        Some(results)
    }

    async fn address_ex_from_contracts_optional(
        &self,
        address: &String,
    ) -> Option<AddressEx> {
        if address != "0x0000000000000000000000000000000000000000" {
            Some(
                self.address_ex_from_contracts_or_default(address)
                    .await,
            )
        } else {
            None
        }
    }

    async fn optional_address_ex_from_contracts(
        &self,
        address: &Option<String>,
    ) -> Option<AddressEx> {
        OptionFuture::from(address.as_ref().map(|address| async move {
            self.address_ex_from_contracts_or_default(address)
                .await
        }))
        .await
    }
}
