use serde_repr::{Deserialize_repr, Serialize_repr};
use serde::{Deserialize, Serialize};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy, Hash)]
#[repr(u8)]
pub enum Operation {
    CALL = 0,
    DELEGATE = 1,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DataDecoded {
    pub method: String,
    pub parameters: Option<Vec<Parameter>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Parameter {
    pub name: String,
    #[serde(rename = "type")]
    pub param_type: String,
    pub value: ParamValue,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename(deserialize = "decodedValue"))]
    pub value_decoded: Option<Vec<ValueDecoded>>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq)]
#[serde(untagged)]
pub enum ParamValue {
    SingleValue(String),
    ArrayValue(Vec<ParamValue>),
}

#[derive(Serialize, Deserialize, Debug, Clone, Hash, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ValueDecoded {
    pub operation: String, //TODO use operation, usually this is received as 0 or 1 but here it is `CALL`, etc. as a string
    pub to: String,
    pub value: Option<u64>,
    pub data: Option<String>,
    #[serde(rename(deserialize = "decodedData"))] //TODO maybe we can also open an issue for this to have names consistent across structs
    pub data_decoded: Option<DataDecoded>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Page<T> {
    pub next: Option<String>,
    pub previous: Option<String>,
    pub results: Vec<T>,
}

impl From<String> for ParamValue {
    fn from(item: String) -> Self {
        ParamValue::SingleValue(item)
    }
}
