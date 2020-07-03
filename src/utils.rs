use crate::models::commons::DataDecoded;


//TODO think of a better impl, using enums as per Nick's suggestion
pub const ERC20_TRANSFER_METHODS: &[&str] = &["transfer", "transferFrom"];
pub const ERC721_TRANSFER_METHODS: &[&str] = &["transferFrom", "safeTransferFrom"];
pub const SETTINGS_CHANGE_METHODS: &[&str] = &["setFallbackHandler",
    "addOwnerWithThreshold",
    "removeOwner",
    "swapOwner",
    "changeThreshold",
    "changeMasterCopy",
    "enableModule",
    "disableModule"];

impl DataDecoded {
    pub fn contains_parameter(&self, parameter_name: &str) -> bool {
        self.parameters.as_ref()
            .map(|parameters| parameters.iter().any(|param| param.name == parameter_name))
            .is_some()
    }

    pub fn is_erc20_transfer_method(&self) -> bool {
        ERC20_TRANSFER_METHODS.iter().any(|&value| value == self.method)
    }

    pub fn is_erc721_transfer_method(&self) -> bool {
        ERC20_TRANSFER_METHODS.iter().any(|&value| value == self.method)
    }

    pub fn is_settings_change(&self) -> bool {
        SETTINGS_CHANGE_METHODS.iter().any(|&value| value == self.method)
    }
}
