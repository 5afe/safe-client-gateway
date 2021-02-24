use crate::config::{
    base_transaction_service_url, request_cache_duration, request_error_cache_timeout,
};
use crate::models::backend::transactions::MultisigTransaction;
use crate::utils::cache::CacheExt;
use crate::utils::context::Context;
use ethabi::Bytes;
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
        SAFE_TX_TYPEHASH.to_string(),
    ))
    .unwrap();

    let tx_encoded = &ethabi::encode(&[
        ethabi::Token::Bytes(ERC191_BYTE.into()),
        ethabi::Token::Bytes(ERC191_VERSION.into()),
        ethabi::Token::Bytes(domain_hash.into()),
        ethabi::Token::Bytes(keccak256(domain_separator).into()),
        ethabi::Token::Address(safe_address),
        ethabi::Token::Address(to),
        ethabi::Token::Uint(U256::zero()),               //value
        ethabi::Token::Bytes(keccak256(vec![0]).into()), //data, should calculate Keccak
        ethabi::Token::Uint(U256::zero()),               //operation
        ethabi::Token::Uint(U256::zero()),               //tx_gas
        ethabi::Token::Uint(U256::zero()),               //base_gas
        ethabi::Token::Address(Address::zero()),         //gas_token
        ethabi::Token::Address(Address::zero()),         //refund_receiver
        ethabi::Token::Uint(U256::from(nonce)),
    ]);

    let hash = H256::from(keccak256(tx_encoded));
    log::error!("{:#?}", hash.to_string());

    // correct safe_tx_hash 0x931e3e46c1c06ad4449ae193d159dab9e24c50112682ffea083e0052ba53900b
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
    let domain_separator_hex =
        &DOMAIN_SEPARATOR_TYPEHASH[2..DOMAIN_SEPARATOR_TYPEHASH.len()].to_string();
    let safe_address_hex = &safe_address[2..safe_address.len()].to_string();
    let input = format!("{}{}", domain_separator_hex, safe_address_hex);
    log::error!("DOMAIN HASH: {}", &input);
    // let input_in_bytes:  =
    //     serde_json::from_value(serde_json::Value::String(input.to_string())).unwrap();
    keccak256(input)
}
