mod routes;

const SAFE_STATE: &str = include_str!("json/safe_state.json");
const LAST_COLLECTIBLE_TRANSFER: &str = include_str!("json/last_collectible_transfer.json");
const LAST_QUEUED_TX: &str = include_str!("json/last_queued_tx.json");
const LAST_HISTORY_TX: &str = include_str!("json/last_history_tx.json");
