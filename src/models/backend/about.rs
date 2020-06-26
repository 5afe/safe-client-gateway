use serde::Deserialize;
use ethereum_types::Address;

#[derive(Deserialize, Debug)]
pub struct About {
    pub name: String,
    pub version: String,
    pub api_version: String,
    pub secure: bool,
    pub settings: SettingsDto,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub struct SettingsDto {
    ethereum_node_url: String,
    ethereum_tracing_node_url: String,
    eth_internal_txs_block_process_limit: Option<usize>,
    eth_reorg_blocks: usize,
    eth_uniswap_factory_address: Address,
}