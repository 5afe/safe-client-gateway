use crate::models::service::safes::{AddressEx, SafeInfoEx};
use crate::providers::ext::InfoProviderExt;
use crate::providers::info::{InfoProvider, SafeInfo};

// We need to add Sync as trait bound as info_provider moves across threads
impl SafeInfo {
    pub async fn to_safe_info_ex(&self, info_provider: &(impl InfoProvider + Sync)) -> SafeInfoEx {
        SafeInfoEx {
            address: AddressEx {
                value: self.address.to_owned(),
                name: None,
                logo_url: None,
            },
            nonce: self.nonce,
            threshold: self.threshold,
            implementation: info_provider.to_address_ex(&self.master_copy).await,
            owners: self
                .owners
                .iter()
                .map(|owner| AddressEx {
                    value: owner.to_owned(),
                    name: None,
                    logo_url: None,
                })
                .collect(),
            modules: info_provider.addresses_to_address_ex(&self.modules).await,
            fallback_handler: info_provider
                .optional_to_address_ex(&self.fallback_handler)
                .await,
            version: self.version.to_owned(),
        }
    }
}
