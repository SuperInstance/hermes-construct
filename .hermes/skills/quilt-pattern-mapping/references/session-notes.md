# Session Notes: Quilt Pattern Mapping (2026-08-21)

## Claw Observations
- Claw is a TypeScript-based AI gateway that normalizes messages across platforms (Telegram, LINE, WhatsApp, Discord, Web) and routes them to LLM providers.
- Core agent loop: ternary action (avoid unknown inputs, defer uncertain ones, choose confident responses) – resembles a cellular agent's decide‑act cycle.
- Session model: each conversation has a session with daily notes (raw logs) and long‑term memory (curated knowledge). Sessions can spawn subagents for parallel work.
- Skill system: natural‑language descriptions (seeds) are turned into optimized behaviors via learning strategies (reinforcement, etc.).
- Equipment system: pluggable modules (MEMORY, REASONING, CONSENSUS, SPREADSHEET, DISTILLATION, COORDINATION) can be equipped/unequipped; when unequipped, "muscle memory" triggers are extracted so the claw knows when to re‑equip.
- Social architecture: master‑slave for parallel work, co‑workers for collaboration, consensus for decisions.

## CudaClaw Observations
- Rust project with CUDA bindings (`cust` crate) and async runtime (`tokio`).
- Appears to focus on GPU‑accelerated kernels for agent computations (likely inference or model operations).
- Dependencies include `serde` for serialization, suggesting structured data exchange between host and device.
- The repository contains build scripts (`compile_cuda.bat`, `build_and_run.bat`) and a `src/` directory with Rust source.

## Mapping Ideas
- A Claw session (with its memory and model context) ↔ a Quilt cell holding a structured value.
- The ternary decision step ↔ a Quilt formula that evaluates conditions and selects an action.
- Skills ↔ Quilt equipment slots; a CUDA‑accelerated reasoning kernel could be a REASONING equipment.
- Subagent spawning ↔ Quilt cell‑to‑cell signaling via formulas that trigger updates in neighboring cells.
- Rate‑limiting concept (from Hermes Worker) could be adapted to limit GPU calls per agent per time window.

## Next Steps
- Extract specific interfaces from Claw's `src/gateway/protocol/` and `src/plugin-sdk/`.
- Look for kernel launch patterns in CudaClaw's `src/` (e.g., functions that call `cust::launch*`).
- Draft a Quilt equipment type that wraps a CUDA kernel via a FFI or WASM stub.
- Consider how to persist agent state across runs using Quilt's reactive engine (maybe using its built‑in persistence or external KV).
