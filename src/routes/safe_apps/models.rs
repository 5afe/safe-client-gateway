use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::Serialize;

use crate::config::is_safe_apps_tags_feature_enabled;

#[derive(Serialize, Debug, PartialEq, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct SafeApp {
    pub id: u64,
    pub url: String,
    pub name: String,
    pub icon_url: String,
    pub description: String,
    pub chain_ids: Vec<String>,
    pub provider: Option<SafeAppProvider>,
    pub access_control: SafeAppAccessControlPolicies,
    #[serde(skip_serializing_if = "should_skip_serializing_tags")]
    // We deserialize this for testing so it would break since the value wouldn't be present
    #[serde(default)]
    pub tags: Vec<String>,
}

pub fn should_skip_serializing_tags(_tags: &Vec<String>) -> bool {
    return !is_safe_apps_tags_feature_enabled();
}

#[derive(Serialize, Debug, PartialEq, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct SafeAppProvider {
    pub url: String,
    pub name: String,
}

#[derive(Serialize, Debug, PartialEq, Clone, JsonSchema)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum SafeAppAccessControlPolicies {
    NoRestrictions,
    DomainAllowlist(SafeAppDomainAllowlistPolicy),
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Debug, PartialEq, Clone, JsonSchema)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct SafeAppDomainAllowlistPolicy {
    pub value: Vec<String>,
}
