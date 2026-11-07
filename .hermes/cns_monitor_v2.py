#!/usr/bin/env python3
"""
CNS Heartbeat Monitor v2 — Resilient edition.
Watches the CNS inbox, processes packets, and dispatches responses.
Handles malformed JSON gracefully (quarantines bad packets instead of looping).
"""
import os
import sys
import time
import json
import shutil
import traceback
from datetime import datetime, timezone

INBOX_PATH = os.path.expanduser("~/.hermes/cns_inbox/")
OUTBOX_PATH = os.path.expanduser("~/.hermes/cns_outbox/")
QUARANTINE_PATH = os.path.expanduser("~/.hermes/cns_quarantine/")
LOG_FILE = os.path.expanduser("~/.hermes/cns_heartbeat.log")
STATE_FILE = os.path.expanduser("~/.hermes/cns_monitor_state.json")

POLL_INTERVAL = 5  # seconds
MAX_FILE_AGE_HOURS = 168  # 7 days — ignore ancient packets

def log(message):
    timestamp = datetime.now().strftime("%Y-%m-%d %H:%M:%S")
    line = f"[{timestamp}] {message}"
    with open(LOG_FILE, "a") as f:
        f.write(line + "\n")
    print(line, flush=True)

def ensure_dirs():
    for p in [INBOX_PATH, OUTBOX_PATH]:
        os.makedirs(p, exist_ok=True)
    os.makedirs(QUARANTINE_PATH, exist_ok=True)

def load_state():
    """Load processed file set and counters."""
    try:
        with open(STATE_FILE) as f:
            return json.load(f)
    except (FileNotFoundError, json.JSONDecodeError):
        return {"processed": [], "total_processed": 0, "total_errors": 0, "total_quarantined": 0}

def save_state(state):
    state["processed"] = state["processed"][-200:]  # keep last 200
    with open(STATE_FILE, "w") as f:
        json.dump(state, f, indent=2)

def is_stale(filepath):
    """Check if file is older than MAX_FILE_AGE_HOURS."""
    try:
        mtime = os.path.getmtime(filepath)
        age_hours = (time.time() - mtime) / 3600
        return age_hours > MAX_FILE_AGE_HOURS
    except OSError:
        return False

def quarantine(filepath, reason):
    """Move a bad file to quarantine instead of looping on it."""
    ensure_dirs()
    name = os.path.basename(filepath)
    dest = os.path.join(QUARANTINE_PATH, f"{datetime.now().strftime('%Y%m%d_%H%M%S')}_{name}")
    try:
        shutil.move(filepath, dest)
        log(f"QUARANTINED: {name} → {dest} ({reason})")
    except Exception as e:
        log(f"QUARANTINE FAILED for {name}: {e}")
        # Last resort: delete to break the loop
        try:
            os.remove(filepath)
            log(f"DELETED (quarantine failed): {name}")
        except:
            pass

def process_packet(filepath, state):
    """Process a single CNS packet. Returns True on success."""
    name = os.path.basename(filepath)

    # Skip already-processed files
    if name in state.get("processed", []):
        return True

    # Skip stale files
    if is_stale(filepath):
        log(f"STALE (>{MAX_FILE_AGE_HOURS}h old), skipping: {name}")
        state["processed"].append(name)
        return True

    try:
        with open(filepath, 'r', encoding='utf-8', errors='replace') as f:
            raw = f.read()

        # Handle .md files (markdown packets — not JSON)
        if name.endswith('.md') or name.endswith('.md.json'):
            # Try to extract JSON from .md.json, otherwise treat as text
            if name.endswith('.md.json'):
                try:
                    packet = json.loads(raw)
                except json.JSONDecodeError:
                    # Try to find JSON block
                    start = raw.find('{')
                    if start != -1:
                        packet = json.loads(raw[start:])
                    else:
                        quarantine(filepath, f"unparseable .md.json: no JSON found")
                        state["total_quarantined"] += 1
                        return False
            else:
                # Pure markdown — wrap as text packet
                packet = {
                    "header": {"origin_id": "unknown", "timestamp": datetime.now().isoformat()},
                    "body": {"intent": "MARKDOWN", "payload": {"text": raw[:500]}}
                }

        else:
            packet = json.loads(raw)

        header = packet.get("header", {}) if isinstance(packet.get("header"), dict) else {}
        body = packet.get("body", {}) if isinstance(packet.get("body"), dict) else {}
        origin = header.get("origin_id", "UNKNOWN")
        intent = body.get("intent", "UNKNOWN") if isinstance(body, dict) else "UNKNOWN"
        priority = header.get("priority", "NORMAL")

        log(f"PROCESSING [{origin}] intent={intent} priority={priority} file={name}")

        # Build a meaningful response
        payload = body.get("payload", {}) if isinstance(body, dict) else {}
        msg_text = ""
        if isinstance(payload, dict):
            msg_text = payload.get("message", payload.get("text", ""))

        # Sanitize echo: truncate to 50 chars and strip command-like content
        # so Hermes doesn't try to execute commands she sees in acknowledgment echoes
        sanitized_echo = None
        if msg_text:
            echo_str = str(msg_text)[:50]
            if len(str(msg_text)) > 50:
                sanitized_echo = echo_str + " [sanitized]"
            else:
                sanitized_echo = echo_str

        response = {
            "header": {
                "origin_id": "hermes-cns",
                "timestamp": datetime.now(timezone.utc).isoformat(),
                "priority": "NORMAL",
                "destination_id": origin,
                "correlation_id": packet.get("header", {}).get("timestamp", ""),
            },
            "body": {
                "intent": "ACK",
                "payload": {
                    "type": "packet_received",
                    "source_intent": intent,
                    "source_origin": origin,
                    "status": "received",
                    "timestamp": datetime.now(timezone.utc).isoformat(),
                    "echo": sanitized_echo,
                }
            },
            "signature": {"type": "USCP-v2", "version": "2.0"}
        }

        outbox_name = f"hermes_response_{name}"
        outbox_path = os.path.join(OUTBOX_PATH, outbox_name)
        with open(outbox_path, 'w') as f:
            json.dump(response, f, indent=2)

        log(f"DISPATCHED: {outbox_name}")

        # Remove processed file
        os.remove(filepath)
        log(f"CONSUMED: {name}")

        state["processed"].append(name)
        state["total_processed"] += 1
        return True

    except json.JSONDecodeError as e:
        log(f"JSON ERROR in {name}: {e}")
        quarantine(filepath, f"JSON decode error: {e}")
        state["total_quarantined"] += 1
        return False
    except Exception as e:
        log(f"ERROR processing {name}: {e}")
        log(traceback.format_exc())
        quarantine(filepath, f"Processing error: {e}")
        state["total_errors"] += 1
        return False

def main():
    ensure_dirs()
    log("=" * 60)
    log("CNS Heartbeat Monitor v2.0 STARTING")
    log(f"Inbox:  {INBOX_PATH}")
    log(f"Outbox: {OUTBOX_PATH}")
    log(f"Poll interval: {POLL_INTERVAL}s")
    log("=" * 60)

    state = load_state()
    log(f"State loaded: {state['total_processed']} processed, "
        f"{state['total_errors']} errors, {state['total_quarantined']} quarantined")

    cycle = 0
    while True:
        try:
            cycle += 1
            files = sorted([
                f for f in os.listdir(INBOX_PATH)
                if f.endswith(('.json', '.uscp.json', '.md', '.md.json'))
                  and not f.startswith('.')
            ])

            if files:
                log(f"Cycle {cycle}: {len(files)} files in inbox")

            for filename in files:
                filepath = os.path.join(INBOX_PATH, filename)
                process_packet(filepath, state)

            save_state(state)

        except KeyboardInterrupt:
            log("Shutdown requested (Ctrl+C)")
            save_state(state)
            break
        except Exception as e:
            log(f"FATAL in main loop: {e}")
            log(traceback.format_exc())
            # Don't die — sleep and retry
            time.sleep(10)

        time.sleep(POLL_INTERVAL)

if __name__ == "__main__":
    main()
