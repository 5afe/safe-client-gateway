use serde::Deserialize;

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
    pub access_control: SafeAppAccessControlPolicies,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafeAppProvider {
    pub url: String,
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
pub enum SafeAppAccessControlPolicies {
    #[serde(rename(deserialize = "NO_RESTRICTIONS"))]
    NoRestrictions(SafeAppNoRestrictionsPolicy),
    #[serde(rename(deserialize = "DOMAIN_ALLOWLIST"))]
    DomainAllowList(SafeAppDomainAllowlistPolicy),
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafeAppDomainAllowlistPolicy {
    pub value: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafeAppNoRestrictionsPolicy;
