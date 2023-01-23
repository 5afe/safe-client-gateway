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
    // We deserialize this for testing so it would break since the value wouldn't be present
    #[serde(default)]
    pub tags: Vec<String>,
    pub features: Vec<String>,
    pub developer_website: Option<String>,
    pub social_profiles: Vec<SafeAppSocialProfile>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct SafeAppSocialProfile {
    pub platform: String,
    pub url: String,
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
