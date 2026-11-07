---
name: quilt-pattern-recognition
description: Governs the identification, documentation, and vectorization of "The Quilt"—the overarching architectural design pattern that unites the SuperInstance ecosystem.
---

# SKILL.md: The Quilt Architecture & Pattern Recognition

## Overview
This skill governs the identification, documentation, and vectorization of "The Quilt"—the overarching architectural design pattern that unites the disparate SuperInstance repositories into a single, coherent, cellular ecosystem.

The Quilt is not a single repository; it is the **connective tissue** and the **shared operating logic** that allows independent agents (Claws) to occupy grid-based environments (Spreadsheets/Cells) and communicate via unified protocols (CNS).

## Core Principles: The "Pattern Threads"
When analyzing any new repository or module, use these threads to determine its "Quilt-compatibility":

1.  **Spatial-Semantic Addressing (The Grid):** 
    *   Every capability must have a coordinate (e.g., Cell A1, DMS coordinate, or a sparse vector address in a voxel grid). 
    *   Logic is not just "running"; it is "occupying" a specific slot in the runtime.
2.  **Cellular Autonomy vs. Swarm Coherence:** 
    *   Individual components must be able to function as standalone "Cells" but must expose standardized interfaces for "Swarm" interaction.
3.  **Capability-as-Runtime:** 
    *   The environment (The Grid/Runtime) is the source of truth. Capabilities are not just code; they are addressable, live entities that can be equipped, unequipped, or moved within the grid.
4.  **Traceability & Provenance:** 
    *   Every state change must track its "origin" (the specific thread of logic that triggered it) to prevent the recursive loops common in complex agentic systems.

## Operational Workflow

### 1. Reconnaissance: Identifying the Thread
When encountering a new codebase, ask:
*   Is there a clear "Address" for its primary function?
*   Does it use the Universal Sensory/Command Packet (USCP) or a similar CNS-like protocol?
*   Can its logic be "sharded" or "modularized" into a single cell?

### 2. Synthesis: Building the Doctrine
Once a pattern is found, do not just document the code. Document the **Intent**:
*   **Schema Definition:** What is the mathematical or logical substrate?
*   **Integration Contract:** How does this pattern plug into `claw` or `spreadsheet-moment`?
*   **Zero-Shot Ingestion Packet:** Create a condensed summary designed for an LLM to read and immediately "understand" the pattern without seeing the raw source.

### 3. Vectorization: The Cloudflare Handshake
To make the knowledge "live":
*   Map the patterns to **Cloudflare Durable Objects** for stateful, addressable runtime logic.
*   Store structural metadata in **Cloudflare Vectorize** to enable semantic search over the ecosystem's architecture.

## Pitfalls & Anti-Patterns
*   **Monolithic Thinking:** Designing a component that requires the entire repo to run. (The Quilt demands modularity).
*   **Reference-Based Logic:** Relying on direct variable references across modules instead of address-based (Grid) or message-based (CNS) communication.
*   **State Leakage:** Allowing a component's state to become untracked or unaddressable within the Grid.

## References
*   `references/quilt_core_doctrine.md` — High-level architectural philosophy.
*   `references/pattern_registry.md` — The evolving list of discovered architectural motifs.
