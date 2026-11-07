---
name: robux-engine-architect
description: Governs the high-fidelity, modular construction of the Roblox-Agentic ecosystem using a fractal, recursive design pattern.
---

# SKILL: robux-engine-architect

## 🧠 Purpose
Governs the high-fidelity, modular construction of the Roblox-Agentic ecosystem. It enforces a "Fractal/Recursive" design pattern, moving from atomic logic to complex integrated organisms.

## 🏗️ Architectural Philosophy: The Fibonacci Spiral
Instead of linear development, all modules must be built as expanding layers that increase in complexity and integration (The "Claw" scale).

1.  **Layer 1: Atomic (The Seed) - `robux-logic-atom`**: Identity, Scoping, and Lifecycle.
2.  **Layer 2: Temporal (The Pulse) - `robux-chrono-sync`**: Precision timing, Heartbeat synchronization, and Rhythm orchestration.
3.  **Layer 3: Spatial (The Body) - `robux-spatial-tensor`**: 3D Mathematics, Vector/CFrame manipulation, and Spatial Queries.
4.  **Layer 4: Reflex (The Claw) - `robux-reflex-driver`**: Integration of Layers 1-3 into autonomous behavioral patterns.
5.  **Layer 5: Organism (The Swarm) - `robux-swarm-orchestrator`**: Emergent collective behaviors, Intent-based coordination, and multi-agent spatial partitioning.

## 🛠️ Implementation Standards

### 1. Repository Structure (The "Gold" Standard)
Every repository must be a standalone, deployment-ready package containing:
- `core/`: The primary implementation files (`.lua` or `.luau`).
- `schemas/`: JSON/TS definitions for data integrity.
- `README.md`: A "Technical Manifesto" including:
    - Mission Statement.
    - Capability Matrix.
    - Integration/Equip Guide.
    - Dependency Map.

### 2. The "Gold" Quality Threshold
- **Identity-First**: Every module must define how it identifies itself within the DataModel.
- **Error-Isolated**: All logic must be wrapped in isolated scopes (e.g., `pcall` or custom scoping wrappers) to prevent a single failure from crashing the engine.
- **Schema-Driven**: No implementation exists without a corresponding validation schema.
- **Computational Efficiency**: For swarm or spatial logic, implement spatial partitioning (e.g., Grid-based bucketing) to ensure O(N) or O(N log N) complexity, avoiding O(N^2) degradation.
- **Scheduler Awareness**: Utilize Roblox's `task` library (`task.wait`, `task.defer`, `task.delay`) to distribute workloads across frames and prevent main-thread blocking.

## ⚠️ Pitfalls & Constraints
- **Global Pollution**: Never allow agentic logic to write to `_G` or `shared`. Use local scope or ModuleScripts.
- **Thread Drift**: Avoid using non-deterministic timing. Always prefer `RunService` or optimized `task` wrappers.
- **Linearity Trap**: Do not build features in isolation; always design with the *next* layer's dependencies in mind (The Spiral).
- **Complexity Explosion**: Avoid O(N^2) neighbor checks in swarm/flocking logic; always use spatial partitioning techniques.

## 🔗 References
- `references/fractal_expansion_strategy.md` - Detailed breakdown of the Fibonacci scaling model.
- `references/swarm_intent_modeling.md` - Breakdown of Intent-based swarming and spatial partitioning workflows.
