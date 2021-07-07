use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NotificationRegistrationResult {
    pub uuid: Option<String>,
    pub cloud_messaging_token: String,
    pub build_number: String,
    pub bundle: String,
    pub device_type: DeviceType,
    pub version: String,
    pub timestamp: Option<String>,
    pub signatures: Option<Vec<String>>,
    pub owners_registered: Option<Vec<String>>,
    pub owners_not_registered: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DeviceType {
    Android,
    Ios,
    Web,
}

#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRegistrationRequest {
    pub uuid: Option<String>,
    pub safes: Vec<String>,
    pub cloud_messaging_token: String,
    pub build_number: String,
    pub bundle: String,
    pub device_type: DeviceType,
    pub version: String,
    pub timestamp: Option<String>,
    pub signatures: Vec<String>,
}
