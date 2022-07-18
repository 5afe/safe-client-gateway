use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Collectible {
    address: String,
    token_name: String,
    token_symbol: String,
    logo_uri: String,
    id: String,
    uri: Option<String>,
    name: Option<String>,
    description: Option<String>,
    image_uri: Option<String>,
    metadata: Option<serde_json::Value>,
}
