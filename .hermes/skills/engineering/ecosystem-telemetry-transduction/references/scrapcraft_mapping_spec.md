# Scrapcraft Telemetry Mapping Reference

This file contains the specific mappings derived from the Scrapcraft/SuperInstance audit (2026-08-23).

## Mapping Logic: Scrapcraft $\rightarrow$ USCP

### 1. Resource Extraction
**Raw Event:**
`{ "action": "mine", "material": "iron_scrap", "qty": 5, "coord": [12, 45] }`

**Transduced USCP (`RESOURCE_ACQUIRED`):**
```json
{
  "header": { "version": "1.0.0", "timestamp": 1724412000000, "origin": "scrapcraft-node-alpha" },
  "payload": {
    "signal_type": "RESOURCE_ACQUIRED",
    "data": { "resource_id": "scrap_iron", "quantity": 5, "location": { "x": 12, "y": 45 } }
  },
  "metadata": { "lore_ref": "lore://materials/scrap_iron_context" }
}
```

### 2. System Health
**Raw Event:**
`{ "status": "warning", "component": "power_cell", "level": 0.15 }`

**Transduced USCP (`SYSTEM_ALERT`):**
```json
{
  "header": { "version": "1.0.0", "timestamp": 1724412005000, "origin": "scrapcraft-node-alpha" },
  "payload": {
    "signal_type": "SYSTEM_ALERT",
    "data": { "alert_level": "CRITICAL", "component": "power_cell", "metric_value": 0.15 }
  },
  "metadata": { "lore_ref": "lore://mechanics/power_depletion_protocols" }
}
```

## Implementation Notes
- **Pincher-Cache Layer:** Must perform the `lore_ref` enrichment via RAG before dispatch.
- **Durable Objects:** The Quilt Engine acts as the authority for these signal states.
