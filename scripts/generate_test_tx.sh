#!/bin/bash

curl --location --request POST 'https://safe-transaction.rinkeby.gnosis.io/api/v1/safes/0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f/multisig-transactions/' \
--header 'Content-Type: application/json' \
--data-raw '{
    "to": "0x05c85Ab5B09Eb8A55020d72daf6091E04e264af9",
    "value": "100000",
    "operation": "0",
    "safeTxGas": "0",
    "baseGas": "0",
    "gasPrice": "0",
    "nonce": "3",
    "contractTransactionHash": "0x8ec2103ec6d9d298f38e1d5b6e0e3f475a875a103219742b8376ca0fda5b5eb9",
    "sender": "0x05c85Ab5B09Eb8A55020d72daf6091E04e264af9"
}'

curl --location --request POST 'https://safe-transaction.rinkeby.gnosis.io/api/v1/safes/0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f/multisig-transactions/' \
--header 'Content-Type: application/json' \
--data-raw '{
    "to": "0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f",
    "value": "0",
    "operation": "0",
    "safeTxGas": "0",
    "baseGas": "0",
    "gasPrice": "0",
    "nonce": "3",
    "contractTransactionHash": "0xdfa23b5831bc5530bc7646d372dddd9afd1ef1883d799752a515f27cd4058bf5",
    "sender": "0x05c85Ab5B09Eb8A55020d72daf6091E04e264af9"
}'

curl --location --request POST 'https://safe-transaction.rinkeby.gnosis.io/api/v1/safes/0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f/multisig-transactions/' \
--header 'Content-Type: application/json' \
--data-raw '{
    "to": "0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f",
    "value": "12378797979797979792616486748431541",
    "operation": "0",
    "safeTxGas": "0",
    "baseGas": "0",
    "gasPrice": "0",
    "nonce": "3",
    "contractTransactionHash": "0x1addcdee38ce15c04ccfead718c23aadcc17b62774b4e6875d9d02231129e67f",
    "sender": "0x05c85Ab5B09Eb8A55020d72daf6091E04e264af9"
}'

curl --location --request POST 'https://safe-transaction.rinkeby.gnosis.io/api/v1/safes/0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f/multisig-transactions/' \
--header 'Content-Type: application/json' \
--data-raw '{
    "to": "0x05c85Ab5B09Eb8A55020d72daf6091E04e264af9",
    "value": "12378797979797979792616486748431541",
    "operation": "0",
    "safeTxGas": "0",
    "baseGas": "0",
    "gasPrice": "0",
    "nonce":
    "contractTransactionHash": "0xd720f976fe7dd6692217dedbeec1ca2936d3e677ddf77e3da57b8b8b914dc09d",
    "sender": "0x05c85Ab5B09Eb8A55020d72daf6091E04e264af9"
}'

curl --location --request POST 'https://safe-transaction.rinkeby.gnosis.io/api/v1/safes/0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f/multisig-transactions/' \
--header 'Content-Type: application/json' \
--data-raw '{
    "to": "0x05c85Ab5B09Eb8A55020d72daf6091E04e264af9",
    "value": "0",
    "operation": "0",
    "safeTxGas": "0",
    "baseGas": "0",
    "gasPrice": "0",
    "nonce": "5",
    "contractTransactionHash": "0xfd9a8a8b365004ace135fdd75f5870e7c752921d3881235ddaab9a62a4a6f2bb",
    "sender": "0x05c85Ab5B09Eb8A55020d72daf6091E04e264af9"
}'

curl --location --request POST 'https://safe-transaction.rinkeby.gnosis.io/api/v1/safes/0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f/multisig-transactions/' \
--header 'Content-Type: application/json' \
--data-raw '{
    "to": "0x126ab4d9e87b5cba98Ddeb75Df703E83500b6B7f",
    "value": "0",
    "data": "0xfd9a8a8b365004ace135fdd75f5870e7c752921d3881235ddaab9a62a4a6f2bb",
    "operation": "0",
    "safeTxGas": "0",
    "baseGas": "0",
    "gasPrice": "0",
    "nonce": "5",
    "contractTransactionHash": "0x4398414227cebb41d76d8aa471f6cf77f939c7d3dde789b40c8db669447f1457",
    "sender": "0x05c85Ab5B09Eb8A55020d72daf6091E04e264af9"
}'