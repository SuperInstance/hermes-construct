---
name: plato-kernel-architecture
description: Governs the design and implementation of the Plato-Kernel spatial-semantic engine.
---

# Plato-Kernel: Spatial-Semantic Engine

## Overview
The Plato-Kernel is the core engine for the SuperInstance Digital Twin. It manages the "Spatial-Semantic" reality through a hierarchical, reactive graph structure that bridges a highly granular "MUD" (Multi-User Dungeon) internal state with a human-readable "SCUMM" (Point-and-Click) external interface.

## Core Concept: The MUD-to-SCUMM Bridge
The system operates on a dual-layered paradigm:
- **Internal (MUD):** A high-performance, graph-based world composed of `World` $\rightarrow$ `Station` $\rightarrow$ `Room` $\rightarrow$ `Object`. It is driven by discrete "Ticks" and "Semantic Deltas."
- **External (SCUMM):** A visual projection of the MUD where human users "walk" through rooms, "zoom" into stations, and interact with objects via a semantic dashboard.

## Implementation Patterns

### 1. Hierarchical Containment
The world is a directed graph. Containment is strictly enforced:
- A `Room` contains `Objects` and `Stations`.
- A `Station` contains `Rooms` or `Objects`.
- A `World` contains `Stations`.

### 2. Reactive Delta Propagation (The Resonance Pattern)
Instead of continuous polling, the kernel uses a **Delta-Tick Engine**.
- **Delta-Injection:** Changes are injected as `Semantic-Deltas` (e.g., an object's temperature changes).
- **Recursive Propagation:** When an object changes a property that affects its "atmosphere," the delta is recursively propagated up the containment hierarchy (Object $\rightarrow$ Room $\rightarrow$ Station $\rightarrow$ World) to ensure the higher-level state always reflects the underlying reality.

### 3. Multi-Resolution Zooming
The transition between abstraction layers is handled via semantic granularity:
- **Macro-Zoom:** Moving from the global dashboard to a specific `Room`.
- **Micro-Zoom:** Moving from a `Station` to a specific `Object`, triggering the loading of high-fidelity visual proxies (e.g., VLM camera feeds).

## Key Technical Constraints
- **Strict Typing:** All entities must conform to the `Plato-Spatial-Schema`.
- **Tick-Based Updates:** State changes are processed on a discrete `Tick` to maintain temporal consistency.
- **Spatial Anchoring:** Every entity must have a coordinate/manifold anchor within its parent container.
