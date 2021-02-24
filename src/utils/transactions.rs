use crate::utils::context::Context;
use ethcontract_common::hash::keccak256;
use ethereum_types::{Address, U256};

pub fn fetch_rejections(context: &Context, safe_address: &str, nonce: &u64) -> Option<Vec<String>> {
    log::debug!("{:#?}", &safe_address);

    // let safe_address: Address =
    //     serde_json::from_value(serde_json::value::Value::String(safe_address.to_string())).unwrap();
    // let nonce: Int = serde_json::from_str(nonce).unwrap();

    // let hash = &ethabi::encode(&[
    //     ethabi::Token::Address(safe_address),
    //     ethabi::Token::Bytes(vec![0]),
    //     ethabi::Token::Uint(U256::from(0)),
    //     ethabi::Token::Uint(U256::from(0)),
    // ]);
    // log::debug!("{:#?}", hash);

    // val to = when (val txInfo = transaction.txInfo) {
    //     is TransactionInfo.Transfer -> {
    //         when (val transferInfo = txInfo.transferInfo) {
    //             is TransferInfo.Erc20Transfer -> {
    //                 transferInfo.tokenAddress
    //             }
    //             is TransferInfo.Erc721Transfer -> {
    //                 transferInfo.tokenAddress
    //             }
    //             is TransferInfo.EtherTransfer -> {
    //                 txInfo.recipient
    //             }
    //         }
    //     }
    //     is TransactionInfo.Custom -> {
    //         txInfo.to
    //     }
    //     is TransactionInfo.SettingsChange -> {
    //         safeAddress
    //     }
    //     else -> {
    //     throw UnsupportedTransactionType(transaction::javaClass.name)
    //     }
    // }.value.paddedHexString()
    //
    // val value = transaction.txData?.value.paddedHexString()
    // val data = Sha3Utils.keccak(transaction.txData?.hexData?.hexToByteArray() ?: ByteArray(0)).toHex().padStart(64, '0')
    // val operationString = (transaction.txData?.operation?.id?.toBigInteger() ?: BigInteger.ZERO).paddedHexString()
    // val gasPriceString = executionInfo.gasPrice.paddedHexString()
    // val txGasString = executionInfo.safeTxGas.paddedHexString()
    // val dataGasString = executionInfo.baseGas.paddedHexString()
    // val gasTokenString = executionInfo.gasToken.value.paddedHexString()
    // val refundReceiverString = BigInteger.ZERO.paddedHexString()
    // val nonce = executionInfo.nonce.paddedHexString()

    log::error!("{:#?}", &ethabi::encode(&[ethabi::Token::Uint(0.into())]));
    None
}

// Android uses 64 but that's because it is hex. In our case with u8 we should use 32
fn zero_pad(input: Vec<u8>, final_length: usize) -> Vec<u8> {
    let padding_length = final_length - input.len();
    if padding_length > 0 {
        [input, vec![0; padding_length]].concat()
    } else {
        input
    }
}
