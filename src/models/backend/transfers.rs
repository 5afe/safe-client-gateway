use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
use ethereum_types::{Address, H256, U256};

#[derive(Deserialize, Debug)]
#[serde(tag = "type")]
pub enum Transfer {
    #[serde(rename(deserialize = "ERC721_TRANSFER"))]
    Erc721(Erc721Transfer),
    #[serde(rename(deserialize = "ERC20_TRANSFER"))]
    Erc20(Erc20Transfer),
    #[serde(rename(deserialize = "ETH_TRANSFER"))]
    Eth(EthTransfer),
    #[serde(other)]
    Unknown,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Erc721Transfer {
    pub execution_date: DateTime<Utc>,
    pub block_number: u64,
    pub to: Address,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Erc20Transfer {
    pub token_address: Address,

}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EthTransfer {
    pub value: U256
}


//
// impl Erc721Transfer {
//     pub fn to_transfer(&self) -> Option<ServiceTransaction> {
//         match self {
//             TransferDto::Erc721 { execution_date, block_number, to } =>
//                 Some(ServiceTransfer {
//                     to,
//                     block_number,
//                     execution_date,
//                 }),
//             _ => None
//         }
//     }
// }

// type": "ERC721_TRANSFER",
//             "executionDate": "2020-06-24T09:34:54Z",
//             "blockNumber": 6722388,
//             "transactionHash": "0x2bd3701332f8d701888bf6bb0a341e7447e2392c232ae4b846854e65efbffbc2",
//             "to": "0x1C8b9B78e3085866521FE206fa4c1a67F49f153A",
//             "value": null,
//             "tokenId": "70",
//             "tokenAddress": "0xD32311b42F0F4B592A2a388F90725f4af686C51b",
//             "tokenInfo": null,