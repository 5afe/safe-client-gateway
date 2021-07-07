use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct NotificationRegistrationResult {
    #[serde(flatten)]
    pub notification_device_data: DeviceData,
    pub safes: Vec<String>,
    pub signatures: Option<Vec<String>>,
    pub owners_registered: Option<Vec<String>>,
    pub owners_not_registered: Option<Vec<String>>,
}

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRegistrationRequest {
    #[serde(flatten)]
    pub notification_device_data: DeviceData,
    pub safes: Vec<String>,
    pub signatures: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct DeviceData {
    pub uuid: Option<String>,
    pub cloud_messaging_token: String,
    pub build_number: String,
    pub bundle: String,
    pub device_type: DeviceType,
    pub version: String,
    pub timestamp: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum DeviceType {
    Android,
    Ios,
    Web,
}
