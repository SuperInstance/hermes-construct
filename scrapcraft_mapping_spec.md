# Technical Mapping Specification: Scrapcraft Telemetry $\rightarrow$ CNS/Quilt Sheet Signals

## 1. Overview
This specification defines the transduction logic for converting raw telemetry events from the **Scrapcraft** client (mined resources, hardware status, etc.) into standardized **Universal Control Signal Packets (USCP)** compatible with the **Hermes Central Nervous System (CNS)** and the **Quilt Engine** (running on Durable Objects).

## 2. System Architecture Context
*   **Source:** Scrapcraft Telemetry (Client-side/Worker-side event stream).
*   **Transducer:** Pincher-Cache AI Workers (Perform semantic enrichment and schema validation).
*   **Sink:** CNS/Quilt Sheet (Durable Object-backed state machine).
*   **Enrichment Layer:** Lore-grounded RAG (Provides context for telemetry interpretation).

## 3. Data Transformation Schema

### 3.1. Event Envelope (USCP Standard)
All transduced packets must adhere to the following JSON envelope:

| Field | Type | Description |
| :--- | :--- | :--- |
| `header.version` | `string` | USCP Schema version (e.g., `"1.0.0"`) |
| `header.timestamp` | `integer` | Unix epoch (milliseconds) |
| `header.origin` | `string` | Source identifier (e.g., `"scrapcraft-worker-01"`) |
| `payload.signal_type`| `string` | The CNS Signal Class (e.g., `RESOURCE_ACQUIRED`, `CRITICAL_STATUS`) |
| `payload.data` | `object` | Signal-specific telemetry attributes |
| `metadata.lore_ref`| `string` | RAG-indexed lore pointer for semantic context |

---

### 3.2. Mapping: Resource Events (e.g., "Scrap Mined")

**Scrapcraft Event:** 
`{ "action": "mine", "material": "iron_scrap", "qty": 5, "coord": [12, 45] }`

**Transduced USCP (CNS Signal): `RESOURCE_ACQUIRED`**

```json
{
  "header": {
    "version": "1.0.0",
    "timestamp": 1724412000000,
    "origin": "scrapcraft-node-alpha"
  },
  "payload": {
    "signal_type": "RESOURCE_ACQUIRED",
    "data": {
      "resource_id": "scrap_iron",
      "quantity": 5,
      "location": { "x": 12, "y": 45 },
      "rarity_score": 0.2
    }
  },
  "metadata": {
    "lore_ref": "lore://materials/scrap_iron_context",
    "processing_node": "pincher-cache-worker-04"
  }
}
```

---

### 3.3. Mapping: Status Events (e.g., "Battery Low")

**Scrapcraft Event:** 
`{ "status": "warning", "component": "power_cell", "level": 0.15 }`

**Transduced USCP (CNS Signal): `SYSTEM_ALERT`**

```json
{
  "header": {
    "version": "1.0.0",
    "timestamp": 1724412005000,
    "origin": "scrapcraft-node-alpha"
  },
  "payload": {
    "signal_type": "SYSTEM_ALERT",
    "data": {
      "alert_level": "CRITICAL",
      "component": "power_cell",
      "metric_value": 0.15,
      "threshold_breached": 0.20
    }
  },
  "metadata": {
    "lore_ref": "lore://mechanics/power_depletion_protocols",
    "urgency_weight": 0.85
  }
}
```

## 4. Transduction Pipeline Logic (Pincher-Cache)

1.  **Ingest:** Receive raw scrape-event via WebSocket or HTTP POST.
2.  **Schema Validation:** Check against Scrapcraft raw event types.
3.  **Semantic Enrichment (RAG):** 
    *   Query Lore-grounded RAG using `material` or `component` tags.
    *   Attach `lore_ref` to packet metadata.
4.  **USCP Formatting:** Map raw keys to CNS standard keys (e.g., `qty` $\rightarrow$ `quantity`).
5.  **Dispatch:** Route to the correct **Quilt Engine** Durable Object via CNS routing table.

## 5. Error Handling
*   **Unknown Event Type:** Wrap in `SIGNAL_UNKNOWN` with the raw payload preserved in `payload.data.raw`.
*   **RAG Timeout:** Proceed with USCP generation but omit `metadata.lore_ref`.
*   **Schema Mismatch:** Reject at Pincher-Cache; log error to `telemetry_dead_letter_queue`.
