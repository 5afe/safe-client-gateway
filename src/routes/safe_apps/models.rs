use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
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
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct SafeAppProvider {
    pub url: String,
    pub name: String,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub enum SafeAppAccessControlPolicies {
    NoRestrictions,
    DomainAllowlist(SafeAppDomainAllowlistPolicy),
    #[serde(other)]
    Unknown,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct SafeAppDomainAllowlistPolicy {
    pub value: Vec<String>,
}
