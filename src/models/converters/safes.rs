use crate::models::service::safes::{AddressEx, SafeInfoEx};
use crate::providers::info::{InfoProvider, SafeInfo};

// AddressInfo for `address` and `owners` was deferred for a later version if necessary as it adds little value
impl SafeInfo {
    pub async fn to_safe_info_ex(&self, info_provider: &mut dyn InfoProvider) -> SafeInfoEx {
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
            modules: self.modules.as_ref().map(|modules| {
                modules
                    .iter()
                    .map(async move |module_address| {
                        to_address_ex(module_address, info_provider).await
                    })
                    .collect()
            }),
            fallback_handler: self
                .fallback_handler
                .as_ref()
                .map(async move |it| to_address_ex(&it, info_provider).await),
            version: self.version.to_owned(),
        }
    }
}

async fn to_address_ex(address: &str, info_provider: &mut dyn InfoProvider) -> AddressEx {
    let address_info = info_provider.contract_info(&address).await.ok();
    AddressEx {
        value: address.to_owned(),
        name: address_info.as_ref().map(|it| it.name.to_owned()),
        logo_url: address_info.map(|it| it.logo_uri).to_owned().flatten(),
    }
}
