use serde::Serialize;

/// About
///
/// <details>
/// <summary>Sample</summary>
///
/// ```json
/// {
///   "transactionServiceBaseUri": "https://safe-transaction.staging.gnosisdev.com/api/v1",
///   "name": "safe-client-gateway",
///   "version": "3.0.0",
///   "buildNumber": "48"
/// }
/// ```
/// </details>
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct About {
    /// base URI string used for backend requests
    pub transaction_service_base_uri: String,
    /// crate name
    pub name: String,
    /// env variable `VERSION`, defaults to crate version
    pub version: String,
    /// Build number from github action
    pub build_number: Option<String>,
}
