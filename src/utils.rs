use crate::models::commons::DataDecoded;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rocket::http::uri::Absolute;

pub mod cors;
pub mod context;
pub mod cache;

//TODO think of a better impl, using enums as per Nick's suggestion
pub const ERC20_TRANSFER_METHODS: &[&str] = &["transfer", "transferFrom"];
pub const ERC721_TRANSFER_METHODS: &[&str] = &["transfer", "transferFrom", "safeTransferFrom"];
pub const SETTINGS_CHANGE_METHODS: &[&str] = &["setFallbackHandler",
    "addOwnerWithThreshold",
    "removeOwner",
    "swapOwner",
    "changeThreshold",
    "changeMasterCopy",
    "enableModule",
    "disableModule"];

impl DataDecoded {
    pub fn get_parameter_value(&self, name: &str) -> Option<String> {
        self.parameters.as_ref()?.iter()
            .find(|&param| param.name == name)
            .map(|result| result.value.clone())
    }

    pub fn contains_parameter(&self, name: &str) -> bool {
        self.parameters.as_ref()
            .map(|parameters| parameters.iter().any(|param| param.name == name))
            .is_some()
    }

    pub fn is_erc20_transfer_method(&self) -> bool {
        ERC20_TRANSFER_METHODS.iter().any(|&value| value == self.method)
    }

    pub fn is_erc721_transfer_method(&self) -> bool {
        ERC721_TRANSFER_METHODS.iter().any(|&value| value == self.method)
    }

    pub fn is_settings_change(&self) -> bool {
        SETTINGS_CHANGE_METHODS.iter().any(|&value| value == self.method)
    }
}

pub fn hex_hash<T: Hash>(t: &T) -> String {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    format!("{:#x}", s.finish())
}

//TODO verify we are only touching 'offset' and 'limit'
pub fn extract_query_string(raw_link: &String) -> Option<String> {
    let parsed = Absolute::parse(raw_link).ok()?;
    parsed.origin()?.query().map(|it| it.to_string())
}
