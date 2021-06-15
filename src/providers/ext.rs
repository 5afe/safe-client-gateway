use crate::models::service::safes::AddressEx;
use crate::providers::address_info::AddressInfo;
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

    async fn addresses_to_address_ex(
        &self,
        addresses: &Option<Vec<String>>,
    ) -> Option<Vec<AddressEx>> {
        let addresses = addresses.as_ref()?;
        if addresses.is_empty() {
            return None;
        }
        let mut results = Vec::with_capacity(addresses.len());
        for address in addresses {
            results.push(self.to_address_ex(address).await)
        }
        Some(results)
    }

    async fn to_address_info(&self, address: &Option<String>) -> Option<AddressInfo> {
        OptionFuture::from(
            address
                .as_ref()
                .map(|address| async move { self.contract_info(address).await.ok() }),
        )
        .await
        .flatten()
    }

    async fn to_address_ex(&self, address: &String) -> AddressEx {
        let address_info = self.contract_info(&address).await.ok();
        AddressEx {
            value: address.to_owned(),
            name: address_info.as_ref().map(|it| it.name.to_owned()),
            logo_url: address_info.map(|it| it.logo_uri).to_owned().flatten(),
        }
    }

    async fn to_address_ex_optional(&self, address: &String) -> Option<AddressEx> {
        if address != "0x0000000000000000000000000000000000000000" {
            Some(self.to_address_ex(address).await)
        } else {
            None
        }
    }

    async fn optional_to_address_ex(&self, address: &Option<String>) -> Option<AddressEx> {
        OptionFuture::from(
            address
                .as_ref()
                .map(|address| async move { self.to_address_ex(address).await }),
        )
        .await
    }
}
