use serde::Serialize;

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct About {
    pub transaction_service_base_url: String,
    pub name: String,
    pub version: Option<String>,
    pub build_number: Option<String>,
}