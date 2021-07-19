use crate::models::backend::safes::MasterCopy;
use crate::models::service::addresses::AddressEx;
use crate::models::service::safes::{ImplementationVersionState, SafeInfoEx};
use crate::providers::ext::InfoProviderExt;
use crate::providers::info::{InfoProvider, SafeInfo};
use semver::Version;
use std::cmp::Ordering;

// We need to add Sync as trait bound as info_provider moves across threads
impl SafeInfo {
    pub async fn to_safe_info_ex(
        &self,
        info_provider: &(impl InfoProvider + Sync),
        supported_master_copies: Vec<MasterCopy>,
    ) -> SafeInfoEx {
        let min_chain_version = info_provider
            .chain_info()
            .await
            .expect("ChainInfo must be available")
            .recommended_master_copy_version;
        let implementation_version_state =
            self.version
                .as_ref()
                .map_or(ImplementationVersionState::Unknown, |safe_version| {
                    calculate_version_state(
                        safe_version,
                        &supported_master_copies,
                        min_chain_version,
                    )
                });

        SafeInfoEx {
            address: AddressEx::address_only(&self.address),
            nonce: self.nonce,
            threshold: self.threshold,
            implementation: info_provider
                .address_ex_from_contracts_or_default(&self.master_copy)
                .await,
            owners: self
                .owners
                .iter()
                .map(|owner| AddressEx::address_only(&owner))
                .collect(),
            modules: info_provider
                .multiple_address_ex_from_contracts(&self.modules)
                .await,
            fallback_handler: info_provider
                .address_ex_from_contracts_optional(&self.fallback_handler)
                .await,
            guard: info_provider
                .address_ex_from_contracts_optional(&self.guard)
                .await,
            version: self.version.to_owned(),
            implementation_version_state,
        }
    }
}

pub(super) fn calculate_version_state(
    safe_version: &str,
    supported_master_copies: &Vec<MasterCopy>,
    min_chain_version: String,
) -> ImplementationVersionState {
    let sem_ver_safe = Version::parse(safe_version);
    let sem_ver_min = Version::parse(&min_chain_version);

    let supported_versions = supported_master_copies
        .iter()
        .filter_map(|it| Version::parse(&it.version).ok())
        .collect::<Vec<Version>>();

    if sem_ver_min.is_err() || sem_ver_safe.is_err() || supported_versions.is_empty() {
        return ImplementationVersionState::Unknown;
    }

    let sem_ver_safe = sem_ver_safe.unwrap();
    let sem_ver_min = sem_ver_min.unwrap();

    if !supported_versions.contains(&sem_ver_safe) {
        return ImplementationVersionState::Unknown;
    }

    match sem_ver_safe.cmp(&sem_ver_min) {
        Ordering::Less => ImplementationVersionState::Outdated,
        Ordering::Equal | Ordering::Greater => ImplementationVersionState::UpToDate,
    }
}
