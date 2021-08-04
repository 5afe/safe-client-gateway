---
name: Chain support request
about: Add support to a new chain
title: ''
labels: ''
assignees: ''

---

## New chain support request

Please tick off the boxes as the steps in the process are completed.

## Chain Metadata

We need to collect the metadata for a chain. The following section contains a sample.

<details>
<summary>Example fields</summary>

```json
{
  {
      "chainId": "1",
      "chainName": "Mainnet",
      "rpcUri": {
        "authentication": "API_KEY_PATH",
        "value": "https://mainnet.infura.io/v3/"
      },
      "safeAppsRpcUri": {
        "authentication": "API_KEY_PATH",
        "value": "https://mainnet.infura.io/v3/"
      },
      "blockExplorerUriTemplate": {
        "address": "https://etherscan.io/address/{{address}}",
        "txHash": "https://etherscan.io/tx/{{txHash}}"
      },
      "nativeCurrency": {
        "name": "Ether",
        "symbol": "ETH",
        "decimals": 18,
        "logoUri": "https://gnosis-safe-token-logos.s3.amazonaws.com/ethereum-eth-logo.png"
      },
      "transactionService": "http://safe-transaction.mainnet.staging.gnosisdev.com",
      "theme": {
        "textColor": "#001428",
        "backgroundColor": "#E8E7E6"
      },
      "gasPrice": {
        "type": "oracle",
        "uri": "https://ethgasstation.info/json/ethgasAPI.json",
        "gasParameter": "average",
        "gweiFactor": "1.000000000"
      },
      "ensRegistryAddress": "0x00000000000C2E074eC69A0dFb2997BA6C7d2e1e",
      "recommendedMasterCopyVersion": "1.1.1"
    }
}
```
</details>

If you don't know what values to supply in certain fields, please contact someone from the backend team. 

Nullable fields are: `ensRegistryAddress`

- [ ] metadata collection completed

## Configuration

Once the chain information is collected, we need to configure our services so they can communicate correctly.

### Config service

If you have access to the admin panel of the service you can add the network directly. Otherwise, please request someone from the backend team to do so.

- [ ] metadata added to config service

### Transaction service

If there is already an instance of the transaction service available for this network, verify that there is a webhook registered in it. 

- [ ] Webhook registered in the transaction service instance
