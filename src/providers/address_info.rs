use serde::Serialize;

#[derive(Serialize, PartialEq, Debug)]
pub struct AddressInfo {
    pub name: String,
    pub logo_uri: String,
}
