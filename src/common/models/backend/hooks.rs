use serde::Deserialize;

#[derive(Deserialize, Debug, Hash)]
#[serde(tag = "type")]
pub struct Payload {
    pub address: String,
    #[serde(rename(deserialize = "chainId"))]
    pub chain_id: String,
    #[serde(flatten)]
    pub details: Option<PayloadDetails>,
}

#[derive(Deserialize, Debug, Hash)]
#[serde(tag = "type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum PayloadDetails {
    NewConfirmation(NewConfirmation),
    ExecutedMultisigTransaction(ExecutedMultisigTransaction),
    PendingMultisigTransaction(PendingMultisigTransaction),
    IncomingEther(IncomingEther),
    IncomingToken(IncomingToken),
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct NewConfirmation {
    pub owner: String,
    pub safe_tx_hash: String,
}

#[derive(Deserialize, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct ExecutedMultisigTransaction {
    pub safe_tx_hash: String,
    pub tx_hash: String,
}

#[derive(Deserialize, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct PendingMultisigTransaction {
    pub safe_tx_hash: String,
}

#[derive(Deserialize, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct IncomingEther {
    pub tx_hash: String,
    pub value: String,
}

#[derive(Deserialize, Debug, Hash)]
#[serde(rename_all = "camelCase")]
pub struct IncomingToken {
    pub tx_hash: String,
    pub token_address: String,
    pub token_id: Option<String>,
    pub value: Option<String>,
}
