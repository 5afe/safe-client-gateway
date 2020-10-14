use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmationRequest {
    pub signer: String,
    pub signature: String,
}
