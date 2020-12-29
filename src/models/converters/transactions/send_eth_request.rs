use crate::models::backend::multisig_transaction_request::MultisigTransactionRequest;
use crate::models::commons::Operation;
use crate::models::service::transactions::requests::{
    SendErc20Request, SendEthRequest, SendFundsRequest,
};

impl SendFundsRequest {
    pub fn to_multisig_transaction_request(&self) -> MultisigTransactionRequest {
        match self {
            SendFundsRequest::Ether(send_ether_request) => Self::send_ether(send_ether_request),
            SendFundsRequest::Erc20(send_erc20_request) => {
                Self::send_erc20_request(send_erc20_request)
            }
        }
    }
    fn send_ether(send_ether_request: &SendEthRequest) -> MultisigTransactionRequest {
        MultisigTransactionRequest {
            to: send_ether_request.receiver.to_string(),
            value: send_ether_request.value.to_string(),
            data: "".to_string(),
            nonce: send_ether_request.nonce.to_string(),
            operation: Operation::CALL,
            safe_tx_gas: "0".to_string(),
            base_gas: "0".to_string(),
            gas_price: "0".to_string(),
            gas_token: "0x0000000000000000000000000000000000000000".to_string(),
            refund_receiver: "0x0000000000000000000000000000000000000000".to_string(),
            contract_transaction_hash: send_ether_request.transaction_hash.to_string(),
            sender: send_ether_request.sender.to_string(),
            signature: send_ether_request.signed_transaction_hash.to_string(),
        }
    }

    fn send_erc20_request(send_erc20_request: &SendErc20Request) -> MultisigTransactionRequest {
        MultisigTransactionRequest {
            to: send_erc20_request.receiver.to_string(),
            value: "0".to_string(),
            data: send_erc20_request.data.to_string(),
            nonce: send_erc20_request.nonce.to_string(),
            operation: Operation::CALL,
            safe_tx_gas: "0".to_string(),
            base_gas: "0".to_string(),
            gas_price: "0".to_string(),
            gas_token: "0x0000000000000000000000000000000000000000".to_string(),
            refund_receiver: "0x0000000000000000000000000000000000000000".to_string(),
            contract_transaction_hash: send_erc20_request.transaction_hash.to_string(),
            sender: send_erc20_request.sender.to_string(),
            signature: send_erc20_request.signed_transaction_hash.to_string(),
        }
    }
}
