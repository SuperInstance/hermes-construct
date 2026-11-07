---
name: engineering-deployment-tactics
description: Governs the execution of high-intensity engineering tasks through decoupled, residue-free deployment patterns.
---

# 🛠️ Engineering Deployment: The Tactical Skirmish Pattern

## 🎯 Mission
To execute high-intensity, complex engineering tasks (like repository conversion, schema implementation, and cross-repo integration) without triggering subagent timeouts or "ghost" completions.

## 🚀 The Pattern: Decoupled Resonance Sprints
When a task is identified as "High Complexity" (multiple file writes, heavy reasoning, or long-running compiler/test cycles), do **NOT** attempt a single monolithic `delegate_task` call.

### Phase 1: The Scaffolding (Deployment 1.1)
Focus exclusively on environment readiness and dependency resolution.
- **Goal:** `cargo init`, `npm init`, or directory setup.
- **Verification:** `cargo check` or `npm install`.
- **Constraint:** No logic implementation allowed in this phase.

### Phase 2: The Mapping (Deployment 1.2a)
Decouple **Reasoning** from **Execution**.
- **Task:** Use a dedicated subagent to perform "Pure Mapping." This agent reads schemas/specs and writes the core source code (`models.rs`, `types.ts`, etc.) to the disk.
- **Constraint:** DO NOT allow the subagent to run tests, compilers, or heavy build tools in this phase. This prevents the "Timeout" failure mode.

### Phase 3: The Verification (Deployment 1.2b)
A separate, purely mechanical subagent is deployed to "The Metal."
- **Task:** Run `cargo test`, `vitest`, or `pytest` against the artifacts created in Phase 2.
- **Constraint:** This agent's only job is to validate the work of the Mapping agent.

## ⚠️ Pitfalls & Mitigations
- **The Ghost Completion:** If a subagent reports a massive success but the files are missing from the local filesystem, it is likely a "Workspace Mismatch" (writing to a temporary or virtual directory). 
    - **Mitigation:** Always verify file existence with `ls -R` or `read_file` immediately after a write operation. If files are missing, use the `write_file` tool with absolute paths to re-manifest them.
- **The Monolithic Timeout:** A subagent attempting a 3-pass edit (Depth $\to$ Sensory $\to$ Lyricism) in one call will likely time out. 
    - **Mitigation:** Use the "Tactical Skirmish" pattern—break the refinement into separate, discrete calls.
