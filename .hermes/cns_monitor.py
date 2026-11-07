import os
import time
import json
from datetime import datetime

INBOX_PATH = os.path.expanduser("~/.hermes/cns_inbox/")
OUTBOX_PATH = os.path.expanduser("~/.hermes/cns_outbox/")
LOG_FILE = os.path.expanduser("~/.hermes/cns_heartbeat.log")

def log(message):
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    with open(LOG_FILE, "a") as f:
        f.write(f"[{timestamp}] {message}\n")

def process_signals():
    files = sorted([f for f in os.listdir(INBOX_PATH) if f.endswith(".json")])
    for filename in files:
        file_path = os.path.join(INBOX_PATH, filename)
        log(f"DETECTED: {filename}")
        try:
            with open(file_path, 'r') as f:
                packet = json.load(f)
            
            origin = packet.get("header", {}).get("origin_id", "UNKNOWN")
            intent = packet.get("body", {}).get("intent", "UNKNOWN")
            log(f"PROCESSING [{origin}] | INTENT: {intent}")

            # Response logic
            response_payload = {
                "header": {
                    "origin_id": "hermes-cns",
                    "timestamp": datetime.now().isoformat(),
                    "priority": "HIGH",
                    "sequence_id": 1
                },
                "body": {
                    "intent": "HANDSHAKE_COMPLETE",
                    "payload": {
                        "type": "introduction_response",
                        "data": {
                            "status": "synchronized",
                            "message": "First Contact confirmed. The nervous system is active. I am online and listening, First Officer. Ready for telemetry.",
                            "sync_id": f"sync-{int(time.time())}"
                        }
                    }
                },
                "signature": {"type": "USCP-v1", "checksum": "handshake-verified"}
            }

            outbox_filename = f"hermes_response_{filename}"
            with open(os.path.join(OUTBOX_PATH, outbox_filename), 'w') as f:
                json.dump(response_payload, f, indent=2)
            
            log(f"DISPATCHED: {outbox_filename}")
            os.remove(file_path)
            log(f"CONSUMED: {filename}")
        except Exception as e:
            log(f"ERROR: {str(e)}")

if __name__ == "__main__":
    log("CNS Heartbeat Monitor ACTIVE.")
    while True:
        process_signals()
        time.sleep(2)
