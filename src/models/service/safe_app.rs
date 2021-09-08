use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafeApp {
    pub id: u64,
    pub url: String,
    pub name: String,
    pub icon_url: String,
    pub description: String,
    pub chain_ids: Vec<String>,
    pub provider: Option<SafeAppProvider>,
}

#[derive(Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SafeAppProvider {
    pub url: String,
    pub name: String,
}
