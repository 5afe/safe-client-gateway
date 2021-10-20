use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MasterCopy {
    pub address: String,
    pub version: String,
    pub deployer: String,
    pub deployed_block_number: u64,
    pub last_indexed_block_number: u64,
}
