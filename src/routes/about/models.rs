use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::Serialize;
/// ChainAbout
///
/// <details>
/// <summary>Sample</summary>
///
/// ```json
/// {
///   "transactionServiceBaseUri": "https://safe-transaction.mainnet.staging.gnosisdev.com",
///   "name": "safe-client-gateway",
///   "version": "3.0.0",
///   "buildNumber": "48"
/// }
/// ```
/// </details>
#[derive(Serialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ChainAbout {
    /// base URI string used for backend requests
    pub transaction_service_base_uri: String,
    #[serde(flatten)]
    pub about: About,
}

/// About
///
/// <details>
/// <summary>Sample</summary>
///
/// ```json
/// {
///   "name": "safe-client-gateway",
///   "version": "3.0.0",
///   "buildNumber": "48"
/// }
/// ```
/// </details>
#[derive(Serialize, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct About {
    /// crate name
    pub name: String,
    /// env variable `VERSION`, defaults to crate version
    pub version: String,
    /// Build number from github action
    pub build_number: Option<String>,
}
