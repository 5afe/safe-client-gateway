use crate::models::commons::DataDecoded;
use crate::models::service::transactions::SettingsInfo;
use crate::providers::info::InfoProvider;
use crate::utils::{
    ADD_OWNER_WITH_THRESHOLD, CHANGE_MASTER_COPY, CHANGE_THRESHOLD, DISABLE_MODULE, ENABLE_MODULE,
    REMOVE_OWNER, SET_FALLBACK_HANDLER, SWAP_OWNER,
};

impl DataDecoded {
    pub(super) async fn to_settings_info(
        &self,
        info_provider: &impl InfoProvider,
    ) -> Option<SettingsInfo> {
        match self.method.as_str() {
            SET_FALLBACK_HANDLER => {
                let handler = self.get_parameter_single_value_at(0)?;
                Some(SettingsInfo::SetFallbackHandler {
                    handler_info: info_provider.contract_info(&handler).await.ok(),
                    handler,
                })
            }
            ADD_OWNER_WITH_THRESHOLD => {
                let owner = self.get_parameter_single_value_at(0)?;
                Some(SettingsInfo::AddOwner {
                    owner_info: None,
                    owner,
                    threshold: self.get_parameter_single_value_at(1)?.parse().ok()?,
                })
            }
            REMOVE_OWNER => {
                let owner = self.get_parameter_single_value_at(1)?;
                Some(SettingsInfo::RemoveOwner {
                    owner_info: None,
                    owner,
                    threshold: self.get_parameter_single_value_at(2)?.parse().ok()?,
                })
            }
            SWAP_OWNER => {
                let old_owner = self.get_parameter_single_value_at(1)?;
                let new_owner = self.get_parameter_single_value_at(2)?;
                Some(SettingsInfo::SwapOwner {
                    old_owner_info: None,
                    old_owner,
                    new_owner_info: None,
                    new_owner,
                })
            }
            CHANGE_MASTER_COPY => {
                let implementation = self.get_parameter_single_value_at(0)?;
                Some(SettingsInfo::ChangeImplementation {
                    implementation_info: info_provider.contract_info(&implementation).await.ok(),
                    implementation,
                })
            }
            ENABLE_MODULE => {
                let module = self.get_parameter_single_value_at(0)?;
                Some(SettingsInfo::EnableModule {
                    module_info: info_provider.contract_info(&module).await.ok(),
                    module,
                })
            }
            DISABLE_MODULE => {
                let module = self.get_parameter_single_value_at(1)?;
                Some(SettingsInfo::DisableModule {
                    module_info: info_provider.contract_info(&module).await.ok(),
                    module,
                })
            }
            CHANGE_THRESHOLD => Some(SettingsInfo::ChangeThreshold {
                threshold: self.get_parameter_single_value_at(0)?.parse().ok()?,
            }),
            _ => None,
        }
    }
}
