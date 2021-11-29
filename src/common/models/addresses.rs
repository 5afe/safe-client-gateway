use serde::Serialize;

#[derive(Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
#[cfg_attr(test, derive(serde::Deserialize))]
pub struct AddressEx {
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logo_uri: Option<String>,
}

impl AddressEx {
    pub fn zero() -> Self {
        AddressEx {
            value: "0x0000000000000000000000000000000000000000".to_owned(),
            name: None,
            logo_uri: None,
        }
    }
    pub fn address_only(address: &str) -> Self {
        AddressEx {
            value: address.to_owned(),
            name: None,
            logo_uri: None,
        }
    }
}
