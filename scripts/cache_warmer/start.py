import os
import requests
import sys, getopt
from dotenv import load_dotenv

load_dotenv("../../.env")
TX_SERVICE_URL = os.getenv("TRANSACTION_SERVICE_URL")
PRINT_FORMAT = "{0:<10} {1:>8}::{2:>8}"


def get_base_url(is_spectrum: bool) -> str:
    if is_spectrum:
        return "https://safe-client-spectrum.mainnet.staging.gnosisdev.com"
    elif "staging" not in TX_SERVICE_URL:
        return "https://safe-client.rinkeby.gnosis.io" if "rinkeby" in TX_SERVICE_URL \
            else "https://safe-client.mainnet.gnosis.io"
    else:
        return "https://safe-client-rinkeby.staging.gnosisdev.com" if "rinkeby" in TX_SERVICE_URL \
            else "https://safe-client-mainnet.staging.gnosisdev.com"


def load_safes(is_spectrum: bool) -> list[str]:
    tx_service_url = "https://safe-transaction.mainnet.staging.gnosisdev.com" if is_spectrum else TX_SERVICE_URL
    print("Loading remote ...")
    top_safes_url = tx_service_url + "/api/v1/analytics/multisig-transactions/by-safe/?limit=300"
    print(top_safes_url)
    response = requests.get(top_safes_url)
    return list(map(lambda safe: safe['safe'], response.json()['results']))


def warm_up_safes(base_gateway_url: str, safes: list[str]):
    for safe in safes:
        balance_response = requests.get("%s/v1/safes/%s/balances/USD" % (base_gateway_url, safe))
        collectibles_response = requests.get("%s/v1/safes/%s/collectibles" % (base_gateway_url, safe))
        tx_queued_response = requests.get("%s/v1/safes/%s/transactions/queued" % (base_gateway_url, safe))
        tx_history_response = requests.get("%s/v1/safes/%s/transactions/history" % (base_gateway_url, safe))

        print(PRINT_FORMAT.format(str(balance_response.elapsed.total_seconds()), str(
            balance_response.status_code), balance_response.url))
        print(PRINT_FORMAT.format(str(collectibles_response.elapsed.total_seconds()), str(
            collectibles_response.status_code), collectibles_response.url))
        print(PRINT_FORMAT.format(str(tx_queued_response.elapsed.total_seconds()), str(
            tx_queued_response.status_code), tx_queued_response.url))
        print(PRINT_FORMAT.format(str(tx_history_response.elapsed.total_seconds()), str(
            tx_history_response.status_code), tx_history_response.url))
        print()


is_spectrum = len(sys.argv) > 1
url = get_base_url(is_spectrum)
safes_list = load_safes(is_spectrum)
warm_up_safes(base_gateway_url=url, safes=safes_list)
