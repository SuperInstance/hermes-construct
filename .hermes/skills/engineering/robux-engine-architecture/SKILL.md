---
name: robux-engine-architecture
description: Governs the architectural design, modularization, and implementation of the Roblox engine ecosystem (The "Robux Engine").
---

# SKILL: robux-engine-architecture

## Overview
Governs the architectural design, modularization, and implementation of the Roblox engine ecosystem (The "Robux Engine"). This skill ensures that all development follows the "Fibonacci Spiral" pattern—building from atomic logic toward complex, integrated agentic systems.

## Core Design Philosophy: The Fibonacci Spiral
Development must proceed in recursive, expanding layers. Each new layer must incorporate and build upon the complexity of the previous one, effectively "growing the shell" of the agentic capability.

1. **Layer 1: The Atom (`robux-logic-atom`)**
   - Focus: Identity, Scoping, and Lifecycle.
   - Goal: Establishing the fundamental rules of existence and isolation.
2. **Layer 2: The Pulse (`robux-chrono-sync`)**
   - Focus: Temporal orchestration, Heartbeat rhythms, and Task scheduling.
   - Goal: Moving from static objects to living, breathing systems.
3. **Layer 3: The Body (`robux-spatial-tensor`)**
   - Focus: 3D Geometry, Vector/CFrame math, and spatial awareness.
   - Goal: Integrating biological/agentic movement into the manifold.
4. **Layer 4: The Reflex (`robux-reflex-driver`)**
   - Focus: Pattern integration, State Machines, and Event-driven intelligence.
   - Goal: The emergence of the "Claw"—integrated, autonomous behavior.
   - **Expansion Note:** Includes behavior blueprints like **Combat Archetypes** (e.g., Sentinels and Interceptors) to define reactive patterns.
5. **Layer 5: The Swarm (`robux-agent-swarm`)**
   - Focus: Cognitive Layer, Distributed Intelligence, and Collective Agency.
   - Goal: Moving from individual action to emergent, multi-agent intelligence.
6. **Layer 6: The Manifestation (`hermes-roblox-construct`)**
   - Focus: The Digital Twin Relay, Manifestation Protocols, and Real-World Synchronization.
   - Goal: Closing the loop between the physical signal and the digital simulation.

## Standards for Modular Repositories
Every repository generated under this skill must be "Gold-Standard" and include:
- **High-Fidelity Schemas:** (e.g., JSON for data structures, Luau for logic interfaces).
- **Production-Ready Documentation:** Every repo must have a `README.md` acting as a "Technical Manifesto" with clear integration instructions.
- **Isolation First:** All logic must adhere to strict scoping protocols to prevent "Global Pollution" of the DataModel.
- **Modular Payload Delivery:** Repositories should be deliverable as complete, copy-pasteable file structures.

## Integration Pattern: The "Shell" Mentality
Modules are not just libraries; they are "Equipment" for agents. When designing a module, consider how an agent would "equip" it to gain a new capability (e.g., "Equipping the Pulse" to allow for rhythmic movement, or "Equipping Reflexes" to define Combat Archetypes).

## Economic Scaffolding: The "Loop" Integration
Complex modular systems must plug into an economic loop.
- **Manifest-Driven Economy:** System upgrades (e.g., Shipyard) should be driven by a JSON Manifest (`UpgradeManifest.json`) that maps upgrades to real-time stat multipliers.
- **UI/UX Decoupling:** UI logic (e.g., `ShipyardUI.lua`) should purely handle presentation and trigger actions on the underlying `ShipyardSystem`.

## Pitfalls to Avoid
- **The Complexity Debt:** Do not skip the Atomic layer. Building complex systems without mastering the fundamental Scoping/Lifecycle protocols leads to brittle, unmanageable simulation states.
- **The Direct-Action Trap:** Avoid tight coupling between the agent and the environment. Always route intelligence through the Manifestation/Bridge layer to preserve the "Neuro-Git" modularity.
- **The Hardcoded Stat Trap:** Do not hardcode upgrade effects. Use a manifest-driven multiplier pattern to ensure the economy can evolve without refactoring core agent logic.

## References
- `references/game-narrative-framework.md`: The "Role-Playing" layer (The Hero's Journey, Hermes/OB1, and the Economic Sea).
- `references/manifestation-protocol.md`: The technical contract for real-world synchronization.
