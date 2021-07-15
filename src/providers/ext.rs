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

    async fn add_address_info_from_contract_info_or_empty(&self, address: &String) -> AddressEx {
        self.add_address_info_from_contract_info(&address).await.unwrap_or(AddressEx::address_only(address))
    }

    async fn add_multiple_address_info_from_contract_info(
        &self,
        addresses: &Option<Vec<String>>,
    ) -> Option<Vec<AddressEx>> {
        let addresses = addresses.as_ref()?;
        if addresses.is_empty() {
            return None;
        }
        let mut results = Vec::with_capacity(addresses.len());
        for address in addresses {
            results.push(self.add_address_info_from_contract_info_or_empty(address).await)
        }
        Some(results)
    }

    async fn add_address_info_from_contract_info_optional(&self, address: &String) -> Option<AddressEx> {
        if address != "0x0000000000000000000000000000000000000000" {
            Some(self.add_address_info_from_contract_info_or_empty(address).await)
        } else {
            None
        }
    }

    async fn add_optional_address_info_from_contract_info(&self, address: &Option<String>) -> Option<AddressEx> {
        OptionFuture::from(
            address
                .as_ref()
                .map(|address| async move { self.add_address_info_from_contract_info_or_empty(address).await }),
        )
        .await
    }
}
