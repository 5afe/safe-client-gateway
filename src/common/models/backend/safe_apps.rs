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
    // We set this value as a default since this feature might not be enabled yet. See SAFE_APPS_TAGS_FEATURE_ENABLED
    #[serde(default)]
    pub tags: Vec<String>,
    pub features: Vec<String>,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafeAppProvider {
    pub url: String,
    pub name: String,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(tag = "type")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum SafeAppAccessControlPolicies {
    NoRestrictions,
    DomainAllowlist(SafeAppDomainAllowlistPolicy),
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafeAppDomainAllowlistPolicy {
    pub value: Vec<String>,
}
