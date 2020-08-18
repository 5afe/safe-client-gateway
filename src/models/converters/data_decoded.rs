use crate::models::commons::DataDecoded;
use crate::models::service::transactions::SettingsInfo;
use crate::utils::SETTINGS_CHANGE_METHODS;

impl DataDecoded {
    pub(super) fn to_settings_info(&self) -> Option<SettingsInfo> {
        match self.method.as_str() {
            method_name if method_name == SETTINGS_CHANGE_METHODS[0] => {
                Some(SettingsInfo::SetFallbackHandler {
                    handler: self.get_parameter_single_value_at(0)?
                })
            }
            method_name if method_name == SETTINGS_CHANGE_METHODS[1] => {
                Some(SettingsInfo::AddOwner {
                    owner: self.get_parameter_single_value_at(0)?,
                    threshold: self.get_parameter_single_value_at(1)?.parse().ok()?,
                })
            }
            method_name if method_name == SETTINGS_CHANGE_METHODS[2] => {
                Some(SettingsInfo::RemoveOwner {
                    owner: self.get_parameter_single_value_at(1)?,
                    threshold: self.get_parameter_single_value_at(2)?.parse().ok()?,
                })
            }
            method_name if method_name == SETTINGS_CHANGE_METHODS[3] => {
                Some(SettingsInfo::SwapOwner {
                    old_owner: self.get_parameter_single_value_at(1)?,
                    new_owner: self.get_parameter_single_value_at(2)?,
                })
            }
            method_name if method_name == SETTINGS_CHANGE_METHODS[4] => {
                Some(SettingsInfo::ChangeThreshold {
                    threshold: self.get_parameter_single_value_at(0)?.parse().ok()?,
                })
            }
            method_name if method_name == SETTINGS_CHANGE_METHODS[5] => {
                Some(SettingsInfo::ChangeImplementation {
                    implementation: self.get_parameter_single_value_at(0)?,
                })
            }
            method_name if method_name == SETTINGS_CHANGE_METHODS[6] => {
                Some(SettingsInfo::EnableModule {
                    module: self.get_parameter_single_value_at(0)?
                })
            }
            method_name if method_name == SETTINGS_CHANGE_METHODS[7] => {
                Some(SettingsInfo::DisableModule {
                    module: self.get_parameter_single_value_at(1)?,
                })
            }
            _ => None
        }
    }
}
