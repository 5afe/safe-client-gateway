use crate::models::commons::DataDecoded;
use crate::models::service::transactions::SettingsInfo;

impl DataDecoded {
    pub(super) fn to_settings_info(&self) -> SettingsInfo {
        match self.method.as_str() {
            "setFallbackHandler" => {
                SettingsInfo::SetFallbackHandler {
                    handler: self.get_parameter_single_value("handler").unwrap()
                }
            }
            "addOwnerWithThreshold" => {
                SettingsInfo::AddOwnerWithThreshold {
                    owner: self.get_parameter_single_value("owner").unwrap(),
                    threshold: self.get_parameter_single_value("_threshold").unwrap().parse().unwrap(),
                }
            }
            "removeOwner" => {
                SettingsInfo::RemoveOwner {
                    prev_owner: self.get_parameter_single_value("prevOwner").unwrap(),
                    owner: self.get_parameter_single_value("owner").unwrap(),
                    threshold: self.get_parameter_single_value("_threshold").unwrap().parse().unwrap(),
                }
            }
            "swapOwner" => {
                SettingsInfo::SwapOwner {
                    prev_owner: self.get_parameter_single_value("prevOwner").unwrap(),
                    new_owner: self.get_parameter_single_value("newOwner").unwrap(),
                    old_owner: self.get_parameter_single_value("oldOwner").unwrap(),
                }
            }
            "changeThreshold" => {
                SettingsInfo::ChangeThreshold {
                    threshold: self.get_parameter_single_value("_threshold").unwrap().parse().unwrap(),
                }
            }
            "changeMasterCopy" => {
                SettingsInfo::ChangeImplementation {
                    implementation: self.get_parameter_single_value("_masterCopy").unwrap(),
                }
            }
            "enableModule" => {
                SettingsInfo::EnableModule {
                    module: self.get_parameter_single_value("module").unwrap()
                }
            }
            "disableModule" => {
                SettingsInfo::DisableModule {
                    prev_module: self.get_parameter_single_value("prevModule").unwrap(),
                    module: self.get_parameter_single_value("module").unwrap(),
                }
            }
            _ => {
                SettingsInfo::Unknown
            }
        }
    }
}
