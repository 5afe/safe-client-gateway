pub const MULTISIG_TX_ERC20_TRANSFER: &str =
    include_str!("transactions/multisig_erc20_transfer.json");
pub const MULTISIG_TX_ERC20_TRANSFER_DELEGATE: &str =
    include_str!("transactions/multisig_erc20_transfer_delegate.json");
pub const MULTISIG_TX_ERC20_TRANSFER_WITH_VALUE: &str =
    include_str!("transactions/multisig_erc20_transfer_with_value.json");
pub const MULTISIG_TX_ERC20_TRANSFER_INVALID_TO_AND_FROM: &str =
    include_str!("transactions/multisig_erc20_transfer_invalid_to_and_from.json");
pub const MULTISIG_TX_ERC721_TRANSFER: &str =
    include_str!("transactions/multisig_erc721_transfer.json");
pub const MULTISIG_TX_ERC721_TRANSFER_CANCELLED: &str =
    include_str!("transactions/multisig_erc721_transfer_cancelled.json");
pub const MULTISIG_TX_ERC721_TRANSFER_INVALID_TO_AND_FROM: &str =
    include_str!("transactions/multisig_erc721_transfer_invalid_to_and_from.json");
pub const MULTISIG_TX_ETHER_TRANSFER: &str =
    include_str!("transactions/multisig_ether_transfer.json");
pub const MULTISIG_TX_SETTINGS_CHANGE: &str =
    include_str!("transactions/multisig_addOwnerWithThreshold_settings_change.json");
pub const MULTISIG_TX_UNKNOWN_SETTINGS_CHANGE: &str =
    include_str!("transactions/multisig_newAndDifferentAddOwnerWithThreshold_settings_change.json");
pub const MULTISIG_TX_CUSTOM: &str = include_str!("transactions/multisig_approve_custom.json");
pub const MULTISIG_TX_FAILED: &str = include_str!("transactions/multisig_failed_transfer.json");
pub const MULTISIG_TX_AWAITING_EXECUTION: &str =
    include_str!("transactions/multisig_awaiting_execution.json");
pub const MULTISIG_TX_AWAITING_CONFIRMATIONS: &str =
    include_str!("transactions/multisig_awaiting_confirmations.json");
pub const MULTISIG_TX_AWAITING_CONFIRMATIONS_EMPTY: &str =
    include_str!("transactions/multisig_awaiting_confirmations_empty.json");
pub const MULTISIG_TX_AWAITING_CONFIRMATIONS_NULL: &str =
    include_str!("transactions/multisig_awaiting_confirmations_null.json");
pub const MULTISIG_TX_AWAITING_CONFIRMATIONS_REQUIRED_NULL: &str =
    include_str!("transactions/multisig_awaiting_confirmations_required_null.json");
pub const MULTISIG_TX_CONFIRMATIONS_NULL: &str =
    include_str!("transactions/multisig_confirmations_null.json");
pub const MULTISIG_TX_WITH_ORIGIN: &str = include_str!("transactions/multisig_with_origin.json");
pub const MULTISIG_TX_CANCELLATION: &str =
    include_str!("transactions/multisig_cancellation_transaction.json");

pub const MODULE_TX: &str = include_str!("transactions/module_transaction.json");
pub const MODULE_TX_FAILED: &str = include_str!("transactions/module_transaction_failed.json");
pub const MODULE_TX_ERC20_TRANSFER: &str = include_str!("transactions/module_erc20_transfer.json");
pub const MODULE_TX_ERC721_TRANSFER: &str =
    include_str!("transactions/module_erc721_transfer.json");
pub const MODULE_TX_ETHER_TRANSFER: &str = include_str!("transactions/module_ether_transfer.json");
pub const MODULE_TX_UNKNOWN_SETTINGS_CHANGE: &str =
    include_str!("transactions/module_newAndDifferentAddOwnerWithThreshold_settings_change.json");
pub const MODULE_TX_SETTINGS_CHANGE: &str =
    include_str!("transactions/module_addOwnerWithThreshold_settings_change.json");

pub const ETHEREUM_TX_INCONSISTENT_TOKEN_TYPES: &str =
    include_str!("transactions/ethereum_inconsistent_token_types.json");

pub const SAFE_WITH_MODULES: &str = include_str!("safes/with_modules.json");
pub const SAFE_WITH_THRESHOLD_TWO: &str = include_str!("safes/with_threshold_two.json");
pub const SAFE_WITH_MODULES_AND_HIGH_NONCE: &str =
    include_str!("safes/with_modules_and_high_nonce.json");
pub const SAFE_WITH_GUARD_SAFE_V130_L2: &str = include_str!("safes/with_guard_safe_v130_l2.json");
pub const _SAFE_WITH_MODULE_TXS: &str = include_str!("safes/with_module_transactions.json");

pub const ETHER_TRANSFER_INCOMING: &str = include_str!("transfers/ether_transfer_incoming.json");
pub const ETHER_TRANSFER_OUTGOING: &str = include_str!("transfers/ether_transfer_outgoing.json");
pub const ERC_20_TRANSFER_WITH_ERC721_TOKEN_INFO: &str =
    include_str!("transfers/erc20_transfer_with_erc721_token_info.json");
pub const ERC_20_TRANSFER_WITHOUT_TOKEN_INFO: &str =
    include_str!("transfers/erc_20_transfer_without_token_info.json");
pub const ERC_20_TRANSFER_WITH_TOKEN_INFO_INCOMING: &str =
    include_str!("transfers/erc_20_transfer_with_token_info_incoming.json");
pub const ERC_20_TRANSFER_WITH_TOKEN_INFO_OUTGOING: &str =
    include_str!("transfers/erc_20_transfer_with_token_info_outgoing.json");
pub const ERC_20_TRANSFER_UNEXPECTED_PARAM_NAMES: &str =
    include_str!("transfers/erc_20_transfer_unexpected_param_names.json");
pub const ERC_721_TRANSFER_WITHOUT_TOKEN_INFO: &str =
    include_str!("transfers/erc_721_transfer_without_token_info.json");
pub const ERC_721_TRANSFER_WITH_TOKEN_INFO_INCOMING: &str =
    include_str!("transfers/erc_721_transfer_with_token_info_incoming.json");
pub const ERC_721_TRANSFER_WITH_TOKEN_INFO_OUTGOING: &str =
    include_str!("transfers/erc_721_transfer_with_token_info_outgoing.json");

pub const CREATION_TX: &str = include_str!("transactions/creation_transaction.json");

pub const BACKEND_MULTISIG_TRANSFER_TX: &str =
    include_str!("transactions/backend_multisig_transfer_tx.json");
pub const BACKEND_HISTORY_TRANSACTION_LIST_PAGE: &str =
    include_str!("transactions/backend_history_transaction_list_page.json");
pub const BACKEND_QUEUED_TRANSACTION_LIST_PAGE_NO_CONFLICTS: &str =
    include_str!("transactions/backend_queued_transaction_list_page_no_conflicts.json");
pub const BACKEND_QUEUED_TRANSACTION_LIST_PAGE_CONFLICT_393: &str =
    include_str!("transactions/backend_queued_transaction_list_page_conflicts_393.json");
pub const BACKEND_QUEUED_TRANSACTION_LIST_PAGE_CONFLICT_394: &str =
    include_str!("transactions/backend_queued_transaction_list_page_conflicts_394.json");

pub const TOKEN_USDT: &str = include_str!("tokens/usdt.json");
pub const TOKEN_CRYPTO_KITTIES: &str = include_str!("tokens/crypto_kitties.json");
pub const TOKEN_DAI: &str = include_str!("tokens/dai.json");
pub const TOKEN_PV_MEMORIAL_TOKEN: &str = include_str!("tokens/pv_memorial_token.json");
pub const TOKEN_BAT: &str = include_str!("tokens/bat.json");

pub const DATA_DECODED_APPROVE: &str = include_str!("commons/data_decoded_approve.json");
pub const DATA_DECODED_MULTI_SEND: &str = include_str!("commons/data_decoded_multi_send.json");
pub const DATA_DECODED_ADD_OWNER_WITH_THRESHOLD: &str =
    include_str!("commons/data_decoded_add_owner_with_threshold.json");
pub const DATA_DECODED_CHANGE_MASTER_COPY: &str =
    include_str!("commons/data_decoded_change_master_copy.json");
pub const DATA_DECODED_CHANGE_THRESHOLD: &str =
    include_str!("commons/data_decoded_change_threshold.json");
pub const DATA_DECODED_DISABLE_MODULE: &str =
    include_str!("commons/data_decoded_disable_module.json");
pub const DATA_DECODED_ENABLE_MODULE: &str =
    include_str!("commons/data_decoded_enable_module.json");
pub const DATA_DECODED_REMOVE_OWNER: &str = include_str!("commons/data_decoded_remove_owner.json");
pub const DATA_DECODED_SET_FALLBACK_HANDLER: &str =
    include_str!("commons/data_decoded_set_fallback_handler.json");
pub const DATA_DECODED_SWAP_OWNER: &str = include_str!("commons/data_decoded_swap_owner.json");
pub const DATA_DECODED_EXEC_TRANSACTION_FROM_MODULE: &str =
    include_str!("commons/data_decoded_exec_transaction_from_module.json");
pub const DATA_DECODED_EXEC_TRANSACTION_WITH_VALUE_DECODED: &str =
    include_str!("commons/data_decoded_nested_safe_interaction.json");
pub const DATA_DECODED_SWAP_ARRAY_VALUES: &str =
    include_str!("commons/data_decoded_swap_array_values.json");
pub const DATA_DECODED_MULTI_SEND_SINGLE_INNER_TRANSACTION: &str =
    include_str!("commons/data_decoded_multi_send_single_inner_transaction.json");
pub const DOCTORED_DATA_DECODED_NESTED_MULTI_SENDS: &str =
    include_str!("commons/DOCTORED_data_decoded_nested_multi_sends.json");
pub const DOCTORED_DATA_DECODED_MULTI_SEND_NESTED_DELEGATE: &str =
    include_str!("commons/DOCTORED_data_decoded_multi_send_nested_delegate.json");
pub const DATA_DECODED_SET_GUARD: &str = include_str!("commons/data_decoded_set_guard.json");
pub const DATA_DECODED_DELETE_GUARD: &str = include_str!("commons/data_decoded_delete_guard.json");

pub const BALANCE_ETHER: &str = include_str!("balances/balance_ether.json");
pub const BALANCE_COMPOUND_ETHER: &str = include_str!("balances/balance_compound_ether.json");

pub const TX_DETAILS_WITH_ORIGIN: &str = include_str!("results/tx_details_with_origin.json");

pub const CHAIN_INFO_RINKEBY: &str = include_str!("chains/rinkeby.json");
pub const CHAIN_INFO_POLYGON: &str = include_str!("chains/polygon.json");
pub const CHAIN_INFO_RINKEBY_FIXED_GAS_PRICE: &str =
    include_str!("chains/rinkeby_fixed_gas_price.json");
pub const CHAIN_INFO_RINKEBY_MULTIPLE_GAS_PRICE: &str =
    include_str!("chains/rinkeby_multiple_gas_price.json");
pub const CHAIN_INFO_RINKEBY_NO_GAS_PRICE: &str = include_str!("chains/rinkeby_no_gas_price.json");
pub const CHAIN_INFO_RINKEBY_UNKNOWN_GAS_PRICE: &str =
    include_str!("chains/rinkeby_unknown_gas_price.json");
pub const CHAIN_INFO_RINKEBY_RPC_NO_AUTHENTICATION: &str =
    include_str!("chains/rinkeby_rpc_no_auth.json");
pub const CHAIN_INFO_RINKEBY_RPC_UNKNOWN_AUTHENTICATION: &str =
    include_str!("chains/rinkeby_rpc_auth_unknown.json");
pub const CHAIN_INFO_RINKEBY_DISABLED_WALLETS: &str =
    include_str!("chains/rinkeby_disabled_wallets.json");
pub const CHAIN_INFO_RINKEBY_ENABLED_FEATURES: &str =
    include_str!("chains/rinkeby_enabled_features.json");

pub const POLYGON_SAFE_APPS: &str = include_str!("safe_apps/polygon_safe_apps.json");
pub const POLYGON_SAFE_APP_URL_QUERY: &str =
    include_str!("safe_apps/polygon_safe_app_url_query.json");
pub const POLYGON_SAFE_APPS_WITH_TAGS: &str =
    include_str!("safe_apps/polygon_safe_apps_with_tags.json");

pub const POLYGON_MASTER_COPIES: &str = include_str!("master_copies/polygon_master_copies.json");

pub const COLLECTIBLES_PAGE: &str = include_str!("collectibles/collectibles_page.json");

pub const EXCHANGE_CURRENCY_RATES: &str = include_str!("exchange/currency_rates.json");

pub const CONTRACT_INFO_BID: &str = include_str!("contracts/contract_info_BID.json");

pub const EMPTY_PAGE: &str = include_str!("commons/empty_page.json");
