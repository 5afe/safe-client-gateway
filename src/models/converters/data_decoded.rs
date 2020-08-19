use crate::models::commons::DataDecoded;
use crate::models::service::transactions::SettingsInfo;
use crate::utils::{SET_FALLBACK_HANDLER, ADD_OWNER_WITH_THRESHOLD, REMOVE_OWNER, SWAP_OWNER, CHANGE_THRESHOLD, CHANGE_MASTER_COPY, ENABLE_MODULE, DISABLE_MODULE};

impl DataDecoded {
    pub(super) fn to_settings_info(&self) -> Option<SettingsInfo> {
        match self.method.as_str() {
            SET_FALLBACK_HANDLER => {
                Some(SettingsInfo::SetFallbackHandler {
                    handler: self.get_parameter_single_value_at(0)?
                })
            }
            ADD_OWNER_WITH_THRESHOLD => {
                Some(SettingsInfo::AddOwner {
                    owner: self.get_parameter_single_value_at(0)?,
                    threshold: self.get_parameter_single_value_at(1)?.parse().ok()?,
                })
            }
            REMOVE_OWNER => {
                Some(SettingsInfo::RemoveOwner {
                    owner: self.get_parameter_single_value_at(1)?,
                    threshold: self.get_parameter_single_value_at(2)?.parse().ok()?,
                })
            }
            SWAP_OWNER => {
                Some(SettingsInfo::SwapOwner {
                    old_owner: self.get_parameter_single_value_at(1)?,
                    new_owner: self.get_parameter_single_value_at(2)?,
                })
            }
            CHANGE_THRESHOLD => {
                Some(SettingsInfo::ChangeThreshold {
                    threshold: self.get_parameter_single_value_at(0)?.parse().ok()?,
                })
            }
            CHANGE_MASTER_COPY => {
                Some(SettingsInfo::ChangeImplementation {
                    implementation: self.get_parameter_single_value_at(0)?,
                })
            }
            ENABLE_MODULE => {
                Some(SettingsInfo::EnableModule {
                    module: self.get_parameter_single_value_at(0)?
                })
            }
            DISABLE_MODULE => {
                Some(SettingsInfo::DisableModule {
                    module: self.get_parameter_single_value_at(1)?,
                })
            }
            _ => None
        }
    }
}
