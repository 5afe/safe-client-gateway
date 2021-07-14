use crate::models::service::safes::{AddressEx, ImplementationVersionState, SafeInfoEx};
use crate::providers::ext::InfoProviderExt;
use crate::providers::info::{InfoProvider, SafeInfo};

// We need to add Sync as trait bound as info_provider moves across threads
impl SafeInfo {
    pub async fn to_safe_info_ex(&self, info_provider: &(impl InfoProvider + Sync)) -> SafeInfoEx {
        let min_chain_version = info_provider
            .chain_info()
            .await
            .ok()
            .as_ref()
            .map(|chain_info| chain_info.min_implementation_version);
        let implementation_version_state = self
            .version
            .map_or(ImplementationVersionState::Unknown, |_| {
                calculate_version_state(&self.version, min_chain_version)
            });
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
                .to_address_ex_optional(&self.fallback_handler)
                .await,
            guard: info_provider.to_address_ex_optional(&self.guard).await,
            version: self.version.to_owned(),
            implementation_version_state,
        }
    }

    fn calculate_version_state(
        safe_version: &String,
        min_chain_version: &Option<String>,
    ) -> ImplementationVersionState {
        if min_chain_version.is_none() {
            ImplementationVersionState::Unknown
        }

        ImplementationVersionState::Unknown
    }
}
