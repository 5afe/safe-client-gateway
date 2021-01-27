use serde::Serialize;

#[derive(Serialize, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AddressInfo {
    pub name: String,
    pub logo_uri: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename = "camelCase")]
pub struct ContractInfo {
    pub address: String,
    pub name: Option<String>,
    pub display_name: Option<String>,
    pub logo_uri: Option<String>,
    // pub contract_abi: Option<ContractAbi>, //Ignored for now
}
