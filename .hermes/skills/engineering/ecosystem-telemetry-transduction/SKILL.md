---
name: ecosystem-telemetry-transduction
description: Governs the high-fidelity mapping and transduction of telemetry from specialized agentic environments into the Hermes CNS and Quilt Engine using the USCP standard.
---

# Skill: Ecosystem Integration & Telemetry Transduction

## Overview
Governs the high-fidelity mapping and transduction of telemetry from specialized agentic environments (e.g., Scrapcraft, MIST) into the **Hermes Central Nervous System (CNS)** and **Quilt Engine** using the **Universal Sensory/Command Packet (USCP)** standard.

## Core Concept: The Transduction Pipeline
The goal is to convert heterogeneous, domain-specific "raw" events into standardized, semantically enriched "signals" that the fleet can act upon.

1. **Ingest:** Capture raw event stream (WebSocket/HTTP) from the source environment.
2. **Validation:** Check against the source's raw schema.
3. **Semantic Enrichment (RAG/Pincher-Cache):** 
   - Use RAG to attach `lore_ref` metadata to the packet.
   - This provides context (e.g., *why* a battery level of 0.15 is critical based on current protocols).
4. **USCP Formatting:** Map raw keys to standardized USCP keys (e.g., `qty` $\rightarrow$ `quantity`).
5. **Dispatch:** Route to the appropriate **Durable Object (Quilt Engine)** via the CNS routing table.

## USCP Standard Envelope
All transduced packets must follow this structure:

| Field | Type | Description |
| :--- | :--- | :--- |
| `header.version` | `string` | USCP Schema version |
| `header.timestamp` | `integer` | Unix epoch (ms) |
| `header.origin` | `string` | Source identifier |
| `payload.signal_type`| `string` | CNS Signal Class (e.g., `RESOURCE_ACQUIRED`, `SYSTEM_ALERT`) |
| `payload.data` | `object` | Signal-specific attributes |
| `metadata.lore_ref`| `string` | RAG-indexed lore pointer for semantic context |

## Pitfalls & Best Practices
- **Avoid Raw Passes:** Never send raw telemetry directly to the CNS without passing through the enrichment/transduction layer. Without `lore_ref`, the fleet is "blind" to the context of the signal.
- **Signal Granularity:** Ensure `signal_type` is broad enough for routing but specific enough for immediate reaction (e.g., use `SYSTEM_ALERT` for critical thresholds).
- **Error Handling:** Wrap unknown events in `SIGNAL_UNKNOWN` but preserve the raw payload in `payload.data.raw` for debugging.

## References
- `references/scrapcraft_mapping_spec.md` - Implementation blueprint for Scrapcraft-specific transduction.
