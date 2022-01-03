use serde::Deserialize;
use std::fmt;

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafeApp {
    pub id: u64,
    pub url: String,
    pub name: String,
    pub icon_url: String,
    pub description: String,
    pub chain_ids: Vec<u64>,
    pub provider: Option<SafeAppProvider>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafeAppProvider {
    pub url: String,
    pub name: String,
}

pub enum SafeAppAccessPolicies {
    NoRestrictions,
    DomainAllowList,
}

impl fmt::Display for SafeAppAccessPolicies {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SafeAppAccessPolicies::NoRestrictions => write!(f, "NO_RESTRICTIONS"),
            SafeAppAccessPolicies::DomainAllowList => write!(f, "DOMAIN_ALLOWLIST"),
        }
    }
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafeAppNoRestrictionsPolicy {
    pub r#type: SafeAppAccessPolicies,
}
