import os
import requests
import sys
import csv
from dotenv import load_dotenv

# Usage: (with alias python=python3)
# $ stat venv 2>/dev/null || python -m venv venv
# $ source venv/bin/activate && pip install -r requirements.txt
# $ python start.py

load_dotenv("../../.env")
TX_SERVICE_URL = os.getenv("TRANSACTION_SERVICE_URL")
SAFES_CSV_FILE_NAME = 'safes.csv'


def drill():
    os.system("cargo install drill")
    os.system("drill --benchmark drill_config.yml --stats")


def check_service():
    local_instance_check = requests.get("http://localhost:8000/about")
    if local_instance_check.status_code != 200:
        print("Local instance of the service must be running")
        sys.exit(-1)


def load_safes_from_file() -> list[str]:
    print("Loading from file...")
    with open(SAFES_CSV_FILE_NAME) as csv_file:
        csv_reader = csv.reader(csv_file, delimiter=',', quotechar='"')
        next(csv_file)  # skip header name

        output = []
        for row in csv_reader:
            output.append(row[0])
        return output


def load_safes_remote() -> list[str]:
    print("Loading remote ...")
    top_safes_url = TX_SERVICE_URL + "/api/v1/analytics/multisig-transactions/by-safe/?limit=300"
    print(top_safes_url)
    response = requests.get(top_safes_url)
    safes = list(map(lambda safe: safe['safe'], response.json()['results']))
    with open(SAFES_CSV_FILE_NAME, 'w') as myfile:
        wr = csv.writer(myfile, delimiter=",", quoting=csv.QUOTE_ALL)
        wr.writerow(['safe_address'])
        for safe in safes:
            wr.writerow([safe])
    return safes


def load_safes() -> list[str]:
    return load_safes_from_file() if os.path.isfile(SAFES_CSV_FILE_NAME) else load_safes_remote()


check_service()
safes = load_safes()
print("Top 300 safes:")
print("\n\t" + "\n\t".join(safes))
print("Safes ready for tests ... ")
print("Starting tests ...")

drill()
