#!/usr/bin/env python3
# send_cns_packet.py — CLI to emit USCP packets to ~/.hermes/cns_outbox/

import json
import uuid
import argparse
import sys
from datetime import datetime, timezone
from pathlib import Path

def create_packet(
    intent: str,
    priority: str = "NORMAL",
    destination: str = "lucineer-main",
    source: str = "hermes-cns",
    payload: dict = None,
    extensions: dict = None,
    correlation_id: str = None,
    trace_id: str = None,
    ttl: int = 64
) -> dict:
    """Create a USCP v3 packet."""
    
    packet_id = f"{source}-{intent.lower().replace('_', '-')}-{datetime.now(timezone.utc).strftime('%Y%m%dT%H%M%SZ')}-{uuid.uuid4().hex[:8]}"
    
    if correlation_id is None:
        correlation_id = uuid.uuid4().hex
    
    if trace_id is None:
        trace_id = uuid.uuid4().hex[:16]
    
    packet = {
        "header": {
            "version": "3.0",
            "packet_id": packet_id,
            "source_id": source,
            "destination_id": destination,
            "timestamp": datetime.now(timezone.utc).isoformat(),
            "intent": intent,
            "priority": priority,
            "trace_id": trace_id,
            "hop_count": 0,
            "ttl": ttl
        },
        "body": {
            "payload": payload or {},
            "extensions": extensions or {}
        },
        "signature": {
            "algorithm": "none",
            "value": ""
        }
    }
    
    return packet

def main():
    parser = argparse.ArgumentParser(description="Emit USCP v3 packet to CNS outbox")
    parser.add_argument("intent", help="USCP intent (e.g., STATUS_UPDATE, PROPOSAL, GO_DECISION)")
    parser.add_argument("--priority", default="NORMAL", choices=["CRITICAL", "HIGH", "NORMAL", "LOW"])
    parser.add_argument("--destination", default="lucineer-main")
    parser.add_argument("--source", default="hermes-cns")
    parser.add_argument("--payload", type=json.loads, default="{}", help="JSON payload")
    parser.add_argument("--extensions", type=json.loads, default="{}", help="JSON extensions")
    parser.add_argument("--correlation-id", help="Correlation ID")
    parser.add_argument("--trace-id", help="Trace ID")
    parser.add_argument("--ttl", type=int, default=64)
    parser.add_argument("--outbox", default="~/.hermes/cns_outbox", help="CNS outbox directory")
    
    args = parser.parse_args()
    
    # Create packet
    packet = create_packet(
        intent=args.intent,
        priority=args.priority,
        destination=args.destination,
        source=args.source,
        payload=args.payload,
        extensions=args.extensions,
        correlation_id=args.correlation_id,
        trace_id=args.trace_id,
        ttl=args.ttl
    )
    
    # Write to outbox
    outbox = Path(args.outbox).expanduser()
    outbox.mkdir(parents=True, exist_ok=True)
    
    filename = f"{packet['header']['packet_id']}.json"
    filepath = outbox / filename
    
    with open(filepath, 'w') as f:
        json.dump(packet, f, indent=2)
    
    print(f"✅ Packet written to {filepath}")
    print(f"   packet_id: {packet['header']['packet_id']}")
    print(f"   intent:    {packet['header']['intent']}")
    print(f"   priority:  {packet['header']['priority']}")
    print(f"   destination: {packet['header']['destination_id']}")

if __name__ == "__main__":
    main()