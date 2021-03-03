use crate::models::service::safes::{AddressEx, SafeInfoEx};
use crate::providers::info::{InfoProvider, SafeInfo};

impl SafeInfo {
    pub fn to_safe_info_ex(&self, info_provider: &mut dyn InfoProvider) -> SafeInfoEx {
        SafeInfoEx {
            address: to_address_ex(&self.address, info_provider),
            nonce: self.nonce,
            threshold: self.threshold,
            implementation: to_address_ex(&self.master_copy, info_provider),
            owners: self
                .owners
                .iter()
                .map(|owner| to_address_ex(owner, info_provider))
                .collect(),
            modules: self.modules.as_ref().map(|modules| {
                modules
                    .iter()
                    .map(|module_address| to_address_ex(module_address, info_provider))
                    .collect()
            }),
            fallback_handler: self
                .fallback_handler
                .as_ref()
                .map(|it| to_address_ex(&it, info_provider)),
            version: self.version.to_owned(),
        }
    }
}

fn to_address_ex(address: &str, info_provider: &mut dyn InfoProvider) -> AddressEx {
    let address_info = info_provider.address_info(&address).ok();
    AddressEx {
        value: address.to_owned(),
        name: address_info.as_ref().map(|it| it.name.to_owned()),
        logo_url: address_info.map(|it| it.logo_uri).to_owned().flatten(),
    }
}
