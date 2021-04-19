use crate::models::service::safes::{AddressEx, SafeInfoEx};
use crate::providers::info::{InfoProvider, SafeInfo};
use rocket::futures::future::OptionFuture;
use rocket::futures::stream::{self, StreamExt as _};
use std::borrow::BorrowMut;

// AddressInfo for `address` and `owners` was deferred for a later version if necessary as it adds little value
impl SafeInfo {
    pub async fn to_safe_info_ex(&self, info_provider: &mut impl InfoProvider) -> SafeInfoEx {
        SafeInfoEx {
            address: AddressEx {
                value: self.address.to_owned(),
                name: None,
                logo_url: None,
            },
            nonce: self.nonce,
            threshold: self.threshold,
            implementation: to_address_ex(&self.master_copy, info_provider).await,
            owners: self
                .owners
                .iter()
                .map(|owner| AddressEx {
                    value: owner.to_owned(),
                    name: None,
                    logo_url: None,
                })
                .collect(),
            // "move" forces us to have a function that borrows info_provider
            modules: map_modules_address_ex(&self.modules, info_provider).await,
            //This one can use async move as it is the last use of the info_provider and can be moved
            fallback_handler: OptionFuture::from(
                self.fallback_handler
                    .as_ref()
                    .map(|it| async move { to_address_ex(&it, info_provider).await }),
            )
            .await,
            version: self.version.to_owned(),
        }
    }
}

async fn map_modules_address_ex(
    modules: &Option<Vec<String>>,
    info_provider: &mut impl InfoProvider,
) -> Option<Vec<AddressEx>> {
    // early return if modules are None
    let modules = modules.as_ref()?;
    let mut results = Vec::with_capacity(modules.len());
    for module in modules {
        results.push(to_address_ex(module, info_provider).await)
    }
    Some(results)
}

async fn to_address_ex(address: &str, info_provider: &mut impl InfoProvider) -> AddressEx {
    let address_info = info_provider.contract_info(&address).await.ok();
    AddressEx {
        value: address.to_owned(),
        name: address_info.as_ref().map(|it| it.name.to_owned()),
        logo_url: address_info.map(|it| it.logo_uri).to_owned().flatten(),
    }
}
