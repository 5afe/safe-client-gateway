use crate::models::backend::multisig_transaction_request::MultisigTransactionRequest;
use crate::models::commons::Operation;
use crate::models::service::transactions::requests::SendEthRequest;

impl SendEthRequest {
    pub fn to_multisig_transaction_request(&self) -> MultisigTransactionRequest {
        MultisigTransactionRequest {
            to: self.receiver.to_string(),
            value: self.value.to_string(),
            data: self.data.to_string(),
            nonce: self.nonce.to_string(),
            operation: Operation::CALL,
            safe_tx_gas: "0".to_string(),
            base_gas: "0".to_string(),
            gas_price: "0".to_string(),
            gas_token: "0x0000000000000000000000000000000000000000".to_string(),
            refund_receiver: "0x0000000000000000000000000000000000000000".to_string(),
            contract_transaction_hash: self.transaction_hash.to_string(),
            sender: self.sender.to_string(),
            signature: self.signed_transaction_hash.to_string(),
        }
    }
}
