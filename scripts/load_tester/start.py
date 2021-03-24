import asyncio
import os
import requests
from dotenv import load_dotenv


async def install_cargo_drill():
    os.system("cargo install drill")

load_dotenv("../../.env")
tx_service_url = os.getenv("TRANSACTION_SERVICE_URL")
top_safes_url = tx_service_url + "/api/v1/analytics/multisig-transactions/by-safe/?limit=300"
print(top_safes_url)

response = requests.get(top_safes_url)
safes = list(map(lambda safe: safe['safe'], response.json()['results']))

print("Top 300 safes:")
print("\n\t"+"\n\t".join(safes))
print("Safes ready for tests")
