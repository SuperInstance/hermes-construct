---
name: resonance-scripting-engine
description: Governs the development, implementation, and deployment of the Resonance Scripting Engine (RSE).
---

# RSE: Resonance Scripting Engine

Governs the development, implementation, and deployment of the **Resonance Scripting Engine (RSE)**. The RSE is a high-concurrency, low-latency computational substrate designed to treat intelligence as a "distilled residue" rather than a "bloated artifact."

## Core Principles

* **Micro-Script/Macro-Cognition:** Agents are lightweight bytecode consumers (Micro-Scripts) that asynchronously invoke heavy LLM/GPU models as an "Exocortex" only when hitting a "Cognitive Threshold."
* **Geometric Determinism:** Logic is executed as projections of high-dimensional signals onto a pruned manifold (The Hodge Decomposition).
* **The Decay Loop:** Information is transitioned from structured, high-entropy "Context" $\to$ intermediate "Vectorized Moments" $\to$ low-entropy "Resonant Residue" (Intuition).
* **MUD-Scale Concurrency:** Uses a non-preemptive, cooperative multitasking model (The "Pulse") to manage 10,000+ concurrent agent-lives on minimal hardware.

## Implementation Guidelines

### 1. The Mathematics of Pruning
All pruning operations must follow the **Hodge Decomposition Theorem**:
- **Signal (Harmonic):** The persistent, top-level component.
- **Residue (Exact):** The transient, trend-driven component.
- **Noise (Co-exact):** The high-frequency, stochastic component.

### 2. The Software Stack
- **Substrate:** Register-based Virtual Machine (R-VM) with SIMD-native vector primitives.
- **Communication:** WebSocket-based "Semantic Delta" streaming (The "Tick" protocol).
- **Interface:** A "MUD-to-SCUMM" bridge: Agents perceive the world via "Rendered Packets" filtered by their identity/attributes.

### 3. Development & Implementation (RSE-Dev)
For specialized guidance on implementing the R-VM, the Decay Operator ($\mathcal{D}_\tau$), and the MUD-to-SCUMM bridge, see the development workflow included in the references.

**Core RSE Hierarchy:**
1. **The Substrate (R-VM):** High-performance, SIMD-native, register-based execution environment.
2. **The Protocol (Resonance Protocol):** The mathematical "Law" (JSON) defining Signal, Noise, and Residue.
3. **The Kernel (Decay Controller):** The implementation of $\mathcal{D}_\tau$ using vectorized masking.
4. **The Observer (Visual/Audio):** The mapping of math to `image_api` and `mmx-cli` assets.

**Development Workflow:**
1. **Mathematical Definition:** Define the specific decomposition (e.g., Hodge Decomposition) in `resonance_protocol.json`.
2. **Kernel Drafting:** Implement the `DecayController` in Python/Rust.
3. **Collision Test:** Run the "Resonance vs. Visual Identity" loop to verify entropy reduction.
4. **Scaling:** Port proven Python kernels into the production Rust engine.

### 4. Complexity Management: Avoid "LLM-as-Loop." The loop is the R-VM; the LLM is the intermittent "Reasoning Pulse."

## Pitfalls
- **The Blob Fallacy:** Treating an agent as a continuous LLM stream. This will break the concurrency model.
- **The Identity Drift:** Forgetting to apply the "Character Sheet" filter to the Rendered Packet, leading to agents seeing the "God's Eye View."
- **Entropy Loss:** Failing to implement the decay mechanism, resulting in "Context Bloat" and loss of "Intuition."

## References
- `resonance_protocol.json` (The formal Law)
- `resonance_kernel.py` (The functional prototype)
- `Geometric_Architecture_Spec_Pruned_Policy.md` (The mathematical proof)
- `references/RSE-dev-implementation-guide.md` (Implementation & Testing details)
