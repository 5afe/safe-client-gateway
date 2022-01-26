use crate::common::models::backend::transactions::{
    CreationTransaction, EthereumTransaction, ModuleTransaction, MultisigTransaction,
};
use crate::common::models::backend::transfers::Transfer;
use crate::utils::hex_hash;

impl MultisigTransaction {
    pub fn generate_id(&self) -> String {
        create_id!(
            super::super::models::ID_PREFIX_MULTISIG_TX,
            &self.safe_transaction.safe,
            self.safe_tx_hash
        )
    }
}

impl EthereumTransaction {
    pub fn generate_id(&self, safe_address: &str, transfer_hash: &str) -> String {
        create_id!(
            super::super::models::ID_PREFIX_ETHEREUM_TX,
            safe_address,
            self.tx_hash,
            transfer_hash
        )
    }
}

impl ModuleTransaction {
    pub fn generate_id(&self) -> String {
        create_id!(
            super::super::models::ID_PREFIX_MODULE_TX,
            self.safe_transaction.safe,
            self.transaction_hash,
            hex_hash(self)
        )
    }
}

impl Transfer {
    pub fn generate_id(&self, safe_address: &str, tx_hash: &str) -> String {
        create_id!(
            super::super::models::ID_PREFIX_ETHEREUM_TX,
            safe_address,
            tx_hash,
            hex_hash(self)
        )
    }
}

impl CreationTransaction {
    pub fn generate_id(&self, safe_address: &str) -> String {
        create_id!(super::super::models::ID_PREFIX_CREATION_TX, safe_address)
    }
}
