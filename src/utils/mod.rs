use crate::models::commons::{DataDecoded, ParamValue};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use rocket::http::uri::Absolute;

pub mod cors;
pub mod context;
pub mod cache;
pub mod json;

//TODO think of a better impl, using enums as per Nick's suggestion
pub const TRANSFER_METHOD: &str = "transfer";
pub const ERC20_TRANSFER_METHODS: &[&str] = &[TRANSFER_METHOD, "transferFrom"];
pub const ERC721_TRANSFER_METHODS: &[&str] = &[TRANSFER_METHOD, "transferFrom", "safeTransferFrom"];

pub const SET_FALLBACK_HANDLER: &'static str = "setFallbackHandler";
pub const ADD_OWNER_WITH_THRESHOLD: &'static str = "addOwnerWithThreshold";
pub const REMOVE_OWNER: &'static str = "removeOwner";
pub const SWAP_OWNER: &'static str = "swapOwner";
pub const CHANGE_THRESHOLD: &'static str = "changeThreshold";
pub const CHANGE_MASTER_COPY: &'static str = "changeMasterCopy";
pub const ENABLE_MODULE: &'static str = "enableModule";
pub const DISABLE_MODULE: &'static str = "disableModule";
pub const SETTINGS_CHANGE_METHODS: &[&str] = &[
    SET_FALLBACK_HANDLER,
    ADD_OWNER_WITH_THRESHOLD,
    REMOVE_OWNER,
    SWAP_OWNER,
    CHANGE_THRESHOLD,
    CHANGE_MASTER_COPY,
    ENABLE_MODULE,
    DISABLE_MODULE];

impl DataDecoded {
    pub fn get_parameter_single_value(&self, some_name: &str) -> Option<String> {
        self.parameters.as_ref()?.iter()
            .find_map(|param| {
                match &param.value {
                    ParamValue::SingleValue(value) => {
                        if param.name == some_name { Some(value.clone()) } else { None }
                    }
                    _ => None
                }
            })
    }

    pub fn get_parameter_single_value_at(&self, position: usize) -> Option<String> {
        self.parameters.as_ref().and_then(|parameters|
            parameters.get(position).and_then(|parameter|
                match &parameter.value {
                    ParamValue::SingleValue(value) => {
                        Some(value.clone())
                    }
                    _ => None
                }
            ))
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
