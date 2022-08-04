mod preview;
mod routes;

const POST_CONFIRMATION_RESULT: &str = include_str!("json/post_confirmation_result.json");
const MULTISIG_TX_DETAILS: &str = include_str!("json/multisig_tx_details.json");
const CONTRACTS_RESPONSE: &str = include_str!("json/contracts_response.json");
const PREVIEW_RESPONSE: &str = include_str!("json/preview_response.json");
const PREVIEW_DATA_DECODED_ERROR_RESPONSE: &str =
    include_str!("json/preview_response_data_decoded_error.json");
const CHAIN_RESPONSE: &str = include_str!("json/chain_response.json");
const CONTRACT_INFO: &str = include_str!("json/contract_info_BID.json");
