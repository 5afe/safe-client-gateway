use crate::cache::cache_operations::RequestCached;
use crate::config::{base_transaction_service_url, chain_id, transaction_request_timeout};
use crate::models::backend::transactions::MultisigTransaction;
use crate::providers::info::SAFE_V_1_3_0;
use crate::providers::info::{DefaultInfoProvider, InfoProvider};
use crate::utils::context::Context;
use ethabi::Uint;
use ethcontract_common::hash::keccak256;
use ethereum_types::{Address, H256};
use semver::Version;

pub const DOMAIN_SEPARATOR_TYPEHASH_LEGACY: &'static str =
    "0x035aff83d86937d35b32e04f0ddc6ff469290eef2f1b692d8a815c89404d4749";
pub const DOMAIN_SEPARATOR_TYPEHASH: &'static str =
    "0x47e79534a245952e8b16893a336b85a3d9ea9fa8c573f3d803afb92a79469218";
pub const SAFE_TX_TYPEHASH: &'static str =
    "0xbb8310d486368db6bd6f849402fdd73ad53d316b5a4b2644ad6efe0f941286d8";

pub const ERC191_BYTE: &'static str = "19";
pub const ERC191_VERSION: &'static str = "01";

pub async fn fetch_rejections(
    context: &Context<'_>,
    safe_address: &str,
    nonce: u64,
) -> Option<Vec<String>> {
    let info_provider = DefaultInfoProvider::new(&context);
    let version = info_provider.safe_info(safe_address).await.ok()
        .as_ref()
        .and_then(|safe_info| safe_info.version.as_ref().map(|it| Version::parse(it).ok()))
        .flatten();
    let is_legacy = use_legacy_domain_separator(version);
    let safe_address: Address =
        serde_json::from_value(serde_json::value::Value::String(safe_address.to_string())).unwrap();
    let domain_hash = if is_legacy {
        domain_hash_v100(&safe_address)
    } else {
        domain_hash_v130(&safe_address, chain_id())
    };

    let safe_tx_hash = to_hex_string!(hash(safe_address, nonce, domain_hash).to_vec());

    let multisig_tx = fetch_cancellation_tx(context, safe_tx_hash).await;
    multisig_tx
        .as_ref()
        .map(|cancel_tx| {
            cancel_tx.confirmations.as_ref().map(|confirmations| {
                confirmations
                    .iter()
                    .map(|confirmation| confirmation.owner.to_string())
                    .collect()
            })
        })
        .flatten()
}

pub(super) fn hash(safe_address: Address, nonce: u64, domain_hash: [u8; 32]) -> [u8; 32] {
    let erc_191_byte = u8::from_str_radix(ERC191_BYTE, 16).unwrap();
    let erc_191_version = u8::from_str_radix(ERC191_VERSION, 16).unwrap();

    let mut encoded = ethabi::encode(&[
        ethabi::Token::Uint(Uint::from(domain_hash)),
        ethabi::Token::Uint(Uint::from(cancellation_parts_hash(&safe_address, nonce))),
    ]);

    encoded.insert(0, erc_191_version);
    encoded.insert(0, erc_191_byte);
    keccak256(encoded)
}

pub(super) fn domain_hash_v130(safe_address: &Address, chain_id: u64) -> [u8; 32] {
    let domain_separator: H256 =
        serde_json::from_value(serde_json::Value::String(DOMAIN_SEPARATOR_TYPEHASH.into()))
            .unwrap();

    let encoded = ethabi::encode(&[
        ethabi::Token::Uint(Uint::from(domain_separator.0)),
        ethabi::Token::Uint(Uint::from(chain_id)),
        ethabi::Token::Address(Address::from(safe_address.0)),
    ]);
    keccak256(encoded)
}

pub(super) fn domain_hash_v100(safe_address: &Address) -> [u8; 32] {
    let domain_separator: H256 = serde_json::from_value(serde_json::Value::String(
        DOMAIN_SEPARATOR_TYPEHASH_LEGACY.into(),
    ))
        .unwrap();

    let encoded = ethabi::encode(&[
        ethabi::Token::Uint(Uint::from(domain_separator.0)),
        ethabi::Token::Address(Address::from(safe_address.0)),
    ]);
    keccak256(encoded)
}

pub(super) fn cancellation_parts_hash(safe_address: &Address, nonce: u64) -> [u8; 32] {
    let safe_type_hash: H256 =
        serde_json::from_value(serde_json::Value::String(SAFE_TX_TYPEHASH.into())).unwrap();

    let encoded_parts = &ethabi::encode(&[
        ethabi::Token::Uint(Uint::from(safe_type_hash.0)),
        ethabi::Token::Address(Address::from(safe_address.0)), //to
        ethabi::Token::Uint(Uint::zero()),                     //value
        ethabi::Token::Uint(Uint::from(keccak256(vec![]))),    //data
        ethabi::Token::Uint(Uint::zero()),                     //operation
        ethabi::Token::Uint(Uint::zero()),                     //safe_tx_gas
        ethabi::Token::Uint(Uint::zero()),                     //base_gas
        ethabi::Token::Uint(Uint::zero()),                     //gas_price
        ethabi::Token::Address(Address::zero()),               //gas_token
        ethabi::Token::Address(Address::zero()),               //refund_receiver
        ethabi::Token::Uint(Uint::from(nonce)),                //base_gas
    ]);

    keccak256(encoded_parts)
}

pub(super) fn use_legacy_domain_separator(version: Option<Version>) -> bool {
    if let Some(version) = version.as_ref() {
        version < &SAFE_V_1_3_0
    } else {
        true
    }
}

// We silently fail if the cancellation transaction is not found
async fn fetch_cancellation_tx(
    context: &Context<'_>,
    safe_tx_hash: String,
) -> Option<MultisigTransaction> {
    let url = format!(
        "{}/v1/multisig-transactions/{}/",
        base_transaction_service_url(),
        safe_tx_hash
    );
    let body = RequestCached::new(url)
        .request_timeout(transaction_request_timeout())
        .execute(context.client(), context.cache())
        .await
        .ok();
    body.as_ref()
        .map(|body| serde_json::from_str::<MultisigTransaction>(body).ok())
        .flatten()
}
