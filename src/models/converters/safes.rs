use crate::models::service::addresses::AddressEx;
use crate::models::service::safes::{ImplementationVersionState, SafeInfoEx};
use crate::providers::ext::InfoProviderExt;
use crate::providers::info::{InfoProvider, SafeInfo};
use semver::Version;
use std::cmp::Ordering;

// We need to add Sync as trait bound as info_provider moves across threads
impl SafeInfo {
    pub async fn to_safe_info_ex(&self, info_provider: &(impl InfoProvider + Sync)) -> SafeInfoEx {
        let min_chain_version = info_provider
            .chain_info()
            .await
            .ok()
            .as_ref()
            .map(|chain_info| chain_info.recommended_master_copy_version.to_string());
        let implementation_version_state = self
            .version
            .as_ref()
            .map_or(ImplementationVersionState::Unknown, |it| {
                calculate_version_state(it, &min_chain_version)
            });
        SafeInfoEx {
            address: AddressEx::address_only(&self.address),
            nonce: self.nonce,
            threshold: self.threshold,
            implementation: info_provider.add_address_info_from_contract_info_or_empty(&self.master_copy).await,
            owners: self
                .owners
                .iter()
                .map(|owner| AddressEx::address_only(&owner))
                .collect(),
            modules: info_provider.add_multiple_address_info_from_contract_info(&self.modules).await,
            fallback_handler: info_provider
                .add_address_info_from_contract_info_optional(&self.fallback_handler)
                .await,
            guard: info_provider.add_address_info_from_contract_info_optional(&self.guard).await,
            version: self.version.to_owned(),
            implementation_version_state,
        }
    }
}

pub(super) fn calculate_version_state(
    safe_version: &str,
    min_chain_version: &Option<String>,
) -> ImplementationVersionState {
    if let Some(min_chain_version) = min_chain_version {
        let sem_ver_safe = Version::parse(safe_version);
        let sem_ver_min = Version::parse(min_chain_version);

        if sem_ver_min.is_err() || sem_ver_safe.is_err() {
            return ImplementationVersionState::Unknown;
        }

        match sem_ver_safe.unwrap().cmp(&sem_ver_min.unwrap()) {
            Ordering::Less => ImplementationVersionState::Outdated,
            Ordering::Equal => ImplementationVersionState::UpToDate,
            Ordering::Greater => ImplementationVersionState::UpToDate,
        }
    } else {
        ImplementationVersionState::Unknown
    }
}
