use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Collectible {
    address: String,
    token_name: String,
    token_symbol: String,
    logo_uri: String,
    id: String,
    uri: String,
    name: String,
    description: String,
    image_uri: String,
    metadata: Metadata,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    description: String,
    external_url: String,
    name: String,
    attributes: Vec<Option<Atributes>>,
}
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Atributes {
    trait_type: String,
    value: Option<String>,
}
