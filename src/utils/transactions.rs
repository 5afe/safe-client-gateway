use ethabi::{Address, Int};
use ethcontract_common::hash::keccak256;

fn build_cancellation_tx(safe_address: &str, nonce: &str) {
    let safe_address: Address = serde_json::from_str(safe_address).unwrap();
    let nonce: Int = serde_json::from_str(nonce).unwrap();
    let hash = &ethabi::encode(&[
        ethabi::Token::Address(safe_address),
        ethabi::Token::Int(nonce),
    ]);
}
