---
name: quilt-pattern-mapping
description: Map cellular/GPU patterns from Claw and CudaClaw to Quilt concepts for integration.
category: integration
---
# Quilt Pattern Mapping Skill

This skill outlines the process of analyzing the Claw agent gateway (TypeScript) and CudaClaw (Rust/CUDA) to identify cellular agent patterns, GPU kernels, and communication mechanisms that can be mapped to Quilt's reactive engine and cellular computing model.

## Trigger Conditions
- Need to understand how Claw implements agent routing, session management, and skill execution.
- Need to understand how CudaClaw implements GPU-accelerated kernels for agent computations.
- Goal: derive patterns that can be reused or adapted when integrating Quilt with the SuperInstance ecosystem (e.g., using Quilt as a cell‑level state container for Claw agents).

## Steps

### 1. Examine Claw Source
1.1. Review the overall architecture (`README.md`, `ARCHITECTURE.md` if present).  
1.2. Identify core modules:
    - `src/index.ts` – entry point, CLI, plugin loading.
    - `src/gateway/protocol/` – message routing between channels and LLMs.
    - `src/channels/` – platform adapters (Telegram, LINE, WhatsApp, Discord, Web).
    - `src/memory/` – session and long‑term memory storage.
    - `src/process/` – supervisor, cron, subagent spawning.
    - `src/plugin-sdk/` – how skills are loaded and executed.
1.3. Look for patterns that resemble cellular concepts:
    - **Agents** – each message‑processing cycle as a ternary action (avoid/defer/choose).
    - **Cells** – isolated session contexts that can spawn subagents.
    - **Seeds** – skill system where natural‑language descriptions are optimized.
    - **Equipment** – pluggable skills (memory, reasoning, etc.) that can be equipped/unequipped.
    - **Social** – subagent spawning for parallel work (slaves/co‑workers).
1.4. Extract relevant code snippets or interfaces that could map to Quilt cell definitions.

### 2. Examine CudaClaw Source
2.1. Check `Cargo.toml` for dependencies (`cust` for CUDA, `tokio`, `serde`).
2.2. Look at `src/` directory for:
    - Kernel implementations (`.cu` files or Rust wrappers around `cust`).
    - Host‑side logic that launches kernels and manages GPU memory.
    - Data structures that represent agent states or model weights.
2.3. Identify GPU‑accelerated patterns that could be expressed as Quilt “equipment” (e.g., a REASONING module that runs a CUDA kernel).
2.4. Note any existing interfaces for exporting/importing model parameters (relevant for seed learning).

### 3. Map to Quilt Concepts
3.1. **Cell** – A Quilt cell holds a formula or mutable value. Map a Claw session (or subagent) to a cell whose value is the agent’s internal state (memory + model context).
3.2. **Agent** – The Claw agent loop (router → LLM → response) can be seen as a cell that updates its state on each message. The ternary action corresponds to Quilt’s conditional formulas.
3.3. **Seed** – Claw’s skill system (natural‑language → optimized behavior) maps to Quilt’s seed concept: a describable behavior that can be trained on data.
3.4. **Equipment** – Claw’s pluggable skills (memory, reasoning, etc.) map to Quilt equipment slots. A CUDA‑accelerated reasoning kernel could be an equipment item.
3.5. **Communication** – Claw’s inter‑channel routing and subagent messaging map to Quilt’s cell‑to‑cell signaling via formulas or events.

### 4. Document Findings
4.1. Create a mapping table (Claw concept → Quilt concept) with code references.
4.2. Note any gaps where Quilt would need new equipment or formula types.
4.3. Suggest how to embed a CudaClaw kernel as a Quilt equipment (e.g., via a foreign‑function interface or WebAssembly stub).

### 5. Validate with a Prototype
5.1. If possible, build a minimal Quilt sheet that simulates a Claw agent’s state update using a simple formula.
5.2. Replace the formula with a call to a dummy CUDA kernel (or a placeholder) to test the equipment idea.
5.3. Verify that the rate‑limiting and token‑cap concepts from the Hermes Cloudflare Worker can be applied to limit GPU calls per agent.

## Pitfalls
- Claw is primarily a multi‑channel AI gateway written in TypeScript; its core agent loop is not a native cellular engine but a workflow engine. Do not confuse the gateway with the actual agent model.
- CudaClaw may be a research prototype; its CUDA kernels might be tightly coupled to specific models. Abstract the kernel interface before mapping.
- Quilt’s reactive engine expects pure functions (formulas) with deterministic outputs. Any equipment that introduces side effects (e.g., GPU state) must be encapsulated.

## References
- Claw README: https://github.com/SuperInstance/claw/blob/main/README.md
- CudaClaw Cargo.toml: shows dependencies on `cust` and `tokio`.
- Quilt core docs: see `packages/core/` in the quilt repo for cell and engine definitions.
- Hermes Worker rate‑limiting example: demonstrates per‑IP KV‑based limits that could inspire agent‑level call limits.

## Templates
- `templates/claw-to-quilt-mapping.md` – starter table for mapping concepts.
- `templates/cuda-equipment-proto.ts` – example TypeScript stub for wrapping a CUDA kernel as Quilt equipment.

## Scripts
- `scripts/extract-claw-modules.sh` – helper to list Claw source files by category.
- `scripts/scan-cuda-kernels.sh` – helper to find CUDA‑related code in CudaClaw.
