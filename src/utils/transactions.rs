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

pub const ERC191_BYTE: &'static str = "19";
pub const ERC191_VERSION: &'static str = "01";

pub fn fetch_rejections(
    context: &Context,
    safe_address: &str,
    to: &str,
    nonce: u64,
) -> Option<Vec<String>> {
    let safe_address: Address =
        serde_json::from_value(serde_json::value::Value::String(safe_address.to_string())).unwrap();
    let to: Address =
        serde_json::from_value(serde_json::value::Value::String(to.to_string())).unwrap();
    let hash = hash(&safe_address);
    log::error!("{:#?}", to_hex_string(hash.into()));

    //
    // let tx_encoded = &ethabi::encode(&[
    //     ethabi::Token::Bytes(ERC191_BYTE.into()),
    //     ethabi::Token::Bytes(ERC191_VERSION.into()),
    //     ethabi::Token::Bytes(domain_hash.into()),
    //     ethabi::Token::Bytes(keccak256(domain_separator).into()),
    //     ethabi::Token::Address(safe_address),
    //     ethabi::Token::Address(to),
    //     ethabi::Token::Uint(U256::zero()),               //value
    //     ethabi::Token::Bytes(keccak256(vec![0]).into()), //data, should calculate Keccak
    //     ethabi::Token::Uint(U256::zero()),               //operation
    //     ethabi::Token::Uint(U256::zero()),               //tx_gas
    //     ethabi::Token::Uint(U256::zero()),               //base_gas
    //     ethabi::Token::Address(Address::zero()),         //gas_token
    //     ethabi::Token::Address(Address::zero()),         //refund_receiver
    //     ethabi::Token::Uint(U256::from(nonce)),
    // ]);
    //
    // let hash = H256::from(keccak256(tx_encoded));
    // log::error!("{:#?}", hash.to_string());

    // correct safe_tx_hash 0x931e3e46c1c06ad4449ae193d159dab9e24c50112682ffea083e0052ba53900b
    None
}

fn hash(safe_address: &Address) -> [u8; 32] {
    let erc_191_byte = u8::from_str_radix(ERC191_BYTE, 16).unwrap();
    let erc_191_version = u8::from_str_radix(ERC191_VERSION, 16).unwrap();
    let type_hash: H256 =
        serde_json::from_value(serde_json::Value::String(SAFE_TX_TYPEHASH.into())).unwrap();

    let mut hashable = vec![erc_191_byte, erc_191_version];

    hashable.extend(domain_hash(safe_address).iter());
    hashable.extend(keccak256(type_hash.0).iter());

    return keccak256(hashable);
}

fn domain_hash(safe_address: &Address) -> [u8; 32] {
    let domain_separator: H256 =
        serde_json::from_value(serde_json::Value::String(DOMAIN_SEPARATOR_TYPEHASH.into()))
            .unwrap();

    let safe_address = zero_pad(safe_address.0.into(), 32);
    let input = [domain_separator.0.to_vec(), safe_address].concat();

    keccak256(input)
}

// Android uses 64 but that's because it is hex. In our case with u8 we should use 32
fn zero_pad(input: Vec<u8>, final_length: usize) -> Vec<u8> {
    let padding_length = final_length - input.len();
    if padding_length > 0 {
        [vec![0; padding_length], input].concat()
    } else {
        input
    }
}

// Maybe we could implement https://github.com/ethereum/EIPs/blob/master/EIPS/eip-55.md
fn to_hex_string(input: Vec<u8>) -> String {
    let mut output = String::new();
    for byte in input.iter() {
        output.push_str(&format!("{:02x?}", byte)) // uppercase x is for uppercase hex char.
    }
    output
}

// fetch the cancellation tx:
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
