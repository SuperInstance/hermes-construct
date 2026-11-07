---
name: resonance-scripting-engine-dev
description: Governs the development, implementation, and testing of the Resonance Scripting Engine (RSE) and its mathematical components.
---

# 🛠️ Resonance Scripting Engine (RSE) Development Skill

This skill governs the implementation, testing, and refinement of the **Resonance Scripting Engine (RSE)**—the high-performance, low-entropy substrate designed to replace heavy LLM-driving logic with lightweight, vectorized "Decay" loops.

## 🎯 Core Objectives
- Implement the **R-VM** (Register-based Virtual Machine) architecture.
- Execute the **$\mathcal{D}_\tau$ (Decay Operator)** to transform high-entropy signals into low-entropy resonance.
- Bridge the **MUD-to-SCUMM** gap (Logical state $\leftrightarrow$ Visual manifold).

## 🧬 The RSE Hierarchy (Sub-Components)
1. **The Substrate (R-VM):** High-performance, SIMD-native, register-based execution environment.
2. **The Protocol (Resonance Protocol):** The mathematical "Law" (JSON) defining Signal, Noise, and Residue.
3. **The Kernel (Decay Controller):** The implementation of $\mathcal{D}_\tau$ using vectorized masking.
4. **The Observer (Visual/Audio):** The mapping of math to `image_api` and `mmx-cli` assets.

## 🛠️ Implementation Workflow
1. **Mathematical Definition:** Define the specific decomposition (e.g., Hodge Decomposition) in `resonance_protocol.json`.
2. **Kernel Drafting:** Implement the `DecayController` in Python/Rust.
3. **Collision Test:** Run the "Resonance vs. Visual Identity" loop to verify entropy reduction.
4. **Scaling:** Port proven Python kernels into the production Rust engine.

## ⚠️ Pitfalls & Constraints
- **Entropy Failure:** If the kernel does not reduce entropy, the implementation is a failure.
- **Context Drift:** Ensure the "Decay" is time-dependent and prevents the agent from becoming "stuck" in a high-entropy state.
- **Hardware Sathet:** The RSE must remain lightweight enough to run concurrently with heavy LLM "Exocortex" calls.

## 📋 Verification Checklist
- [ ] Does the kernel reduce Shannon entropy after mask application?
- [ ] Is the `resonance_protocol.json` strictly orthogonal (Signal vs. Noise)?
- [ ] Can the visual output be mapped back to the mathematical residue?
