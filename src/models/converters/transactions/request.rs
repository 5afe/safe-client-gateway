use crate::models::backend::requests::ConfirmationRequest as ConfirmationRequestDto;
use crate::models::service::transactions::requests::ConfirmationRequest;
use crate::models::service::transactions::details::{TransactionDetails, DetailedExecutionInfo};
use anyhow::Result;
use crate::models::commons::Operation;

impl ConfirmationRequest {
    pub(crate) fn build_confirmation_request(&self, safe_address: &str, safe_tx_hash: &str, transaction_details: TransactionDetails) -> Result<ConfirmationRequestDto> {
        match transaction_details.detailed_execution_info {
            Some(DetailedExecutionInfo::Multisig(it)) => {
                let tx_data = transaction_details.tx_data.expect("No tx_data in transaction details");
                Ok(ConfirmationRequestDto {
                    safe: safe_address.to_string(),
                    contract_transaction_hash: safe_tx_hash.to_string(),
                    sender: self.signer.to_string(),
                    signature: self.signature.to_string(),
                    to: tx_data.to,
                    value: tx_data.value.unwrap_or(String::from("0")),
                    data: tx_data.hex_data.unwrap_or(String::from("0x")),
                    operation: serde_json::to_string::<Operation>(&tx_data.operation)?,
                    gas_token: it.gas_token.to_string(),
                    safe_tx_gas: it.safe_tx_gas.to_string(),
                    base_gas: it.base_gas.to_string(),
                    gas_price: it.gas_price.to_string(),
                    refund_receiver: it.refund_receiver.to_string(),
                    nonce: it.nonce.to_string(),
                    origin: "".to_string(),
                })
            }
            _ => anyhow::bail!("Bad transaction detail type")
        }
    }
}