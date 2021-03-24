import asyncio
import os
import requests
import sys
from dotenv import load_dotenv

# Usage: (with alias python=python3)
# $ stat venv 2>/dev/null || python -m venv venv
# $ source venv/bin/activate && pip install -r requirements.txt
# $ python start.py


async def install_cargo_drill():
    os.system("cargo install drill")


local_instance_check = requests.get("http://localhost:8000/about")
if local_instance_check.status_code != 200:
    print("Local instance of the service must be running")
    sys.exit(-1)

load_dotenv("../../.env")
tx_service_url = os.getenv("TRANSACTION_SERVICE_URL")
top_safes_url = tx_service_url + "/api/v1/analytics/multisig-transactions/by-safe/?limit=300"
print(top_safes_url)

response = requests.get(top_safes_url)
safes = list(map(lambda safe: safe['safe'], response.json()['results']))

print("Top 300 safes:")
print("\n\t"+"\n\t".join(safes))
print("Safes ready for tests")
