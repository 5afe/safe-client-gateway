use crate::models::backend::notifications::DeviceData;
use serde::{Deserialize, Serialize};

/// NotificationRegistrationRequest
///
/// <details>
/// <summary>Example body of NotificationRegistrationRequest registering a device for push notifications</summary>
///
/// ```json
///
/// {
///  "uuid": "c50750df-700c-4b17-98ca-b95a5c27ca18",
///  "cloudMessagingToken": "eWv4Ya6OSaiuDI91S0_C6D:APA91bGpprbGOCa1Qev0h3vlMu2nXa9nWpaL7N9fEcX2G4byZ3TSKXircrMtuWg1H4nSG9Ugu7a7rgY1eDKAR9UaxgaP1egTRj3taqAfAQblApuiWFfRRkyxdD3N23t7wYi9ZBIXZ88Z",
///  "bundle": "io.gnosis.safe.debug",
///  "version": "2.13.0",
///  "deviceType": "ANDROID",
///  "buildNumber": "703",
///  "timestamp": "1618906387",
///  "safeRegistrations": [
///      {
///        "chainId": "1",
///        "safes": [
///          "0x00E17aA063fbDB3BFdEfc2c3b2c13173d2711a35"
///        ],
///        "signatures": "0x4b574e7c729db54b427dd17a6b2ae3481221642a9d61c52a53f77500d98ddc1d739c39dfb117619fb09a20e3f5070d018e62c37f89fb622ae10b56a6be9af5c11b"
///      },
///      {
///        "chainId": "4",
///        "safes": [
///          "0x00E17aA063fbDB3BFdEfc2c3b2c13173d2711a35",
///          "0x00e17Aa063FbDB3bFdEfC2c3b2C13173D2711A36"
///        ],
///        "signatures": "0x4b574e7c729db54b427dd17a6b2ae3481221642a9d61c52a53f77500d98ddc1d739c39dfb117619fb09a20e3f5070d018e62c37f89fb622ae10b56a6be9af5c11b"
///      }
///    ]
///  }
/// ```
/// </details>
#[derive(Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct NotificationRegistrationRequest {
    #[serde(flatten)]
    pub device_data: DeviceData,
    pub safe_registrations: Vec<SafeRegistration>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct SafeRegistration {
    pub chain_id: String,
    pub safes: Vec<String>,
    pub signatures: Vec<String>,
}
