use crate::config::{
    base_transaction_service_url, request_cache_duration, request_error_cache_timeout,
};
use crate::models::backend::transactions::MultisigTransaction;
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use ethcontract_common::hash::keccak256;
use ethereum_types::{Address, H256, U256};
use std::str::FromStr;

pub const DOMAIN_SEPARATOR_TYPEHASH: &'static str =
    "0x035aff83d86937d35b32e04f0ddc6ff469290eef2f1b692d8a815c89404d4749";
pub const SAFE_TX_TYPEHASH: &'static str =
    "0xbb8310d486368db6bd6f849402fdd73ad53d316b5a4b2644ad6efe0f941286d8";

pub const ERC191_BYTE: &'static str = "0x19";
pub const ERC191_VERSION: &'static str = "0x01";

pub fn fetch_rejections(
    context: &Context,
    safe_address: &str,
    to: &str,
    nonce: u64,
) -> Option<Vec<String>> {
    let domain_hash = domain_hash(&safe_address);
    let safe_address: Address =
        serde_json::from_value(serde_json::value::Value::String(safe_address.to_string())).unwrap();
    let to: Address =
        serde_json::from_value(serde_json::value::Value::String(to.to_string())).unwrap();

    let domain_separator: H256 = serde_json::from_value(serde_json::value::Value::String(
        DOMAIN_SEPARATOR_TYPEHASH.to_string(),
    ))
    .unwrap();

    let tx_encoded = &ethabi::encode(&[
        ethabi::Token::Bytes(ERC191_BYTE.into()),
        ethabi::Token::Bytes(ERC191_VERSION.into()),
        ethabi::Token::Bytes(domain_hash.into()),
        ethabi::Token::Address(safe_address),
        ethabi::Token::Address(to),
        ethabi::Token::Uint(U256::zero()),       //value
        ethabi::Token::Bytes(vec![0]),           //data, should calculate Keccak
        ethabi::Token::Uint(U256::zero()),       //operation
        ethabi::Token::Uint(U256::zero()),       //tx_gas
        ethabi::Token::Uint(U256::zero()),       //base_gas
        ethabi::Token::Address(Address::zero()), //gas_token
        ethabi::Token::Address(Address::zero()), //refund_receiver
        ethabi::Token::Uint(U256::from(nonce)),
    ]);

    let hash = H256::from(keccak256(tx_encoded));
    log::error!("{:#?}", hash.to_string());

    None
    // let url = format!(
    //     "{}/v1/multisig-transactions/{}/",
    //     base_transaction_service_url(),
    //     safe_tx_hash
    // );
    // let body = context
    //     .cache()
    //     .request_cached(
    //         &context.client(),
    //         &url,
    //         request_cache_duration(),
    //         request_error_cache_timeout(),
    //     )
    //     .ok();
    // let multisig_tx: Option<MultisigTransaction> = body
    //     .as_ref()
    //     .map(|body| serde_json::from_str::<MultisigTransaction>(body).ok())
    //     .flatten();
    //
    // multisig_tx
    //     .as_ref()
    //     .map(|cancel_tx| {
    //         cancel_tx.confirmations.as_ref().map(|confirmations| {
    //             confirmations
    //                 .iter()
    //                 .map(|confirmation| confirmation.owner)
    //                 .collect()
    //         })
    //     })
    //     .flatten()
}

fn domain_hash(safe_address: &str) -> [u8; 32] {
    let input = format!("{}{}", DOMAIN_SEPARATOR_TYPEHASH, safe_address);
    let input_in_bytes: H256 = serde_json::from_value(serde_json::Value::String(input)).unwrap();
    keccak256(input_in_bytes)
}
