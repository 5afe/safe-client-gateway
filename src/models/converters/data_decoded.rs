use crate::models::commons::DataDecoded;
use crate::models::service::transactions::SettingsInfo;

impl DataDecoded {
    pub(super) fn to_settings_info(&self) -> SettingsInfo {
        match self.method.as_str() {
            "setFallbackHandler" => {
                SettingsInfo::SetFallbackHandler {
                    handler: self.get_parameter_single_value_at(0).unwrap()
                }
            }
            "addOwnerWithThreshold" => {
                SettingsInfo::AddOwnerWithThreshold {
                    owner: self.get_parameter_single_value_at(0).unwrap(),
                    threshold: self.get_parameter_single_value_at(1).unwrap().parse().unwrap(),
                }
            }
            "removeOwner" => {
                SettingsInfo::RemoveOwner {
                    prev_owner: self.get_parameter_single_value_at(0).unwrap(),
                    owner: self.get_parameter_single_value_at(1).unwrap(),
                    threshold: self.get_parameter_single_value_at(2).unwrap().parse().unwrap(),
                }
            }
            "swapOwner" => {
                SettingsInfo::SwapOwner {
                    prev_owner: self.get_parameter_single_value_at(0).unwrap(),
                    old_owner: self.get_parameter_single_value_at(1).unwrap(),
                    new_owner: self.get_parameter_single_value_at(2).unwrap(),
                }
            }
            "changeThreshold" => {
                SettingsInfo::ChangeThreshold {
                    threshold: self.get_parameter_single_value_at(0).unwrap().parse().unwrap(),
                }
            }
            "changeMasterCopy" => {
                SettingsInfo::ChangeImplementation {
                    implementation: self.get_parameter_single_value_at(0).unwrap(),
                }
            }
            "enableModule" => {
                SettingsInfo::EnableModule {
                    module: self.get_parameter_single_value_at(0).unwrap()
                }
            }
            "disableModule" => {
                SettingsInfo::DisableModule {
                    prev_module: self.get_parameter_single_value_at(0).unwrap(),
                    module: self.get_parameter_single_value_at(1).unwrap(),
                }
            }
            _ => {
                SettingsInfo::Unknown
            }
        }
    }
}
