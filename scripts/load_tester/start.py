import os
import requests
import sys
from dotenv import load_dotenv

load_dotenv("../../.env")
TX_SERVICE_URL = os.getenv("TRANSACTION_SERVICE_URL")
PRINT_FORMAT = "{0:<10} {1:>8}::{2:>8}"


def check_service():
    local_instance_check = requests.get("http://localhost:8000/about")
    if local_instance_check.status_code != 200:
        print("Local instance of the service must be running")
        sys.exit(-1)


def load_safes() -> list[str]:
    print("Loading remote ...")
    top_safes_url = TX_SERVICE_URL + "/api/v1/analytics/multisig-transactions/by-safe/?limit=300"
    print(top_safes_url)
    response = requests.get(top_safes_url)
    return list(map(lambda safe: safe['safe'], response.json()['results']))


check_service()
safes = load_safes()
print("Top 300 safes:")
print("\n\t" + "\n\t".join(safes))
print("Safes ready ... ")
print("Populating cache ...")

for safe in safes:
    balance_response = requests.get("http://localhost:8000/v1/safes/" + safe + "/balances/USD")
    collectibles_response = requests.get("http://localhost:8000/v1/safes/" + safe + "/collectibles")
    tx_queued_response = requests.get("http://localhost:8000/v1/safes/" + safe + "/transactions/queued")
    tx_history_response = requests.get("http://localhost:8000/v1/safes/" + safe + "/transactions/history")

    print(PRINT_FORMAT.format(str(balance_response.elapsed.total_seconds()), str(
        balance_response.status_code), balance_response.url))
    print(PRINT_FORMAT.format(str(collectibles_response.elapsed.total_seconds()), str(
        collectibles_response.status_code), collectibles_response.url))
    print(PRINT_FORMAT.format(str(tx_queued_response.elapsed.total_seconds()), str(
        tx_queued_response.status_code), tx_queued_response.url))
    print(PRINT_FORMAT.format(str(tx_history_response.elapsed.total_seconds()), str(
        tx_history_response.status_code), tx_history_response.url))
    print()
