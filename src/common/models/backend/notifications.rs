use crate::routes::notifications::models::DeviceData;
use serde::Serialize;

#[derive(Serialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRegistrationRequest {
    #[serde(flatten)]
    pub notification_device_data: DeviceData,
    pub safes: Vec<String>,
    pub signatures: Vec<String>,
}
