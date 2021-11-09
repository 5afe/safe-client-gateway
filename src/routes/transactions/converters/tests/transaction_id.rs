use crate::common::models::backend::transactions::{
    CreationTransaction, EthereumTransaction, ModuleTransaction, MultisigTransaction,
};
use crate::common::models::backend::transfers::Transfer;
use crate::utils::hex_hash;

#[test]
fn multisig_transaction_id() {
    let multisig_tx =
        serde_json::from_str::<MultisigTransaction>(crate::tests::json::MULTISIG_TX_WITH_ORIGIN)
            .unwrap();

    let expected = "multisig_0xBc79855178842FDBA0c353494895DEEf509E26bB_0x728e6dec56dc61523b56dc440e34c1c4c39c66895df8e5d3499ed1f7d4fcfe80";
    let actual = multisig_tx.generate_id();

    assert_eq!(expected, actual);
}

#[test]
fn ethereum_transaction_id() {
    let ethereum_tx = serde_json::from_str::<EthereumTransaction>(
        crate::tests::json::ETHEREUM_TX_INCONSISTENT_TOKEN_TYPES,
    )
    .unwrap();

    let transfer_hash = hex_hash(ethereum_tx.transfers.as_ref().unwrap().first().unwrap());

    let expected = format!("ethereum_0xb07de4b2989E180F8907B8C7e617637C26cE2776_0x2f920d8b75eff1857266643aedcfcf677b834ca164b6684a23a2c8e4574432ec_{}", &transfer_hash);
    let actual =
        ethereum_tx.generate_id("0xb07de4b2989E180F8907B8C7e617637C26cE2776", &transfer_hash);

    assert_eq!(expected, actual);
}

#[test]
fn module_transaction_id() {
    let module_tx =
        serde_json::from_str::<ModuleTransaction>(crate::tests::json::MODULE_TX).unwrap();

    let module_tx_hash = hex_hash(&module_tx);

    let expected = format!("module_0x9422ff6AFB126C31F62057e2853d65cBB73f4608_0x705167e310ef0acb80a5f73eb4f8e66cfb32a896ac9380f3eb43e68ef8603a9f_{}", module_tx_hash);
    let actual = module_tx.generate_id();

    assert_eq!(expected, actual);
}

#[test]
fn transfer_transaction_id() {
    let transfer =
        serde_json::from_str::<Transfer>(crate::tests::json::ETHER_TRANSFER_INCOMING).unwrap();

    let transfer_hash = hex_hash(&transfer);
    let eth_tx_hash = "some_valid_tx_hash";

    let expected = format!(
        "ethereum_0x1230B3d59858296A31053C1b8562Ecf89A2f888b_{}_{}",
        &eth_tx_hash, &transfer_hash
    );
    let actual = transfer.generate_id("0x1230B3d59858296A31053C1b8562Ecf89A2f888b", &eth_tx_hash);

    assert_eq!(expected, actual);
}

#[test]
fn creation_transaction_id() {
    let creation_tx =
        serde_json::from_str::<CreationTransaction>(crate::tests::json::CREATION_TX).unwrap();

    let safe_address = "0xd6f5Bef6bb4acD235CF85c0ce196316d10785d67";

    let expected = format!("creation_{}", safe_address);
    let actual = creation_tx.generate_id(safe_address);

    assert_eq!(expected, actual);
}
