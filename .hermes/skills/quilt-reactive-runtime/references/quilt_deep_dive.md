# Quilt Deep Dive – Session Summary

## Core Files Examined
- `engine.ts` – central `QuiltEngine` class, universal API (`get`, `set`, `call`, `push`, `subscribe`).
- `cells/value.ts` – trivial evaluator for static value cells.
- `cells/formula.ts`, `cells/api.ts`, `cells/program.ts`, `cells/router.ts`, `cells/ai.ts` – effectful cell implementations.
- `cells/sensor.ts` / `cells/io.ts` – push‑based cells for external events.
- `cells/listener.ts` – side‑effect listeners.

## Key Insights
- **Dependency Graph** built on `addDep` and `autoDetectDeps`; formulas auto‑scan expressions for cell IDs.
- **Propagation** marks dependents stale, clears per‑context caches, and fires listeners recursively.
- **Context‑aware Caching** uses `contextKey` to memoize effectful evaluations per caller context.
- **Effectful Evaluation** deduped via `inflight` map; ensures a single async promise per cell per context.
- **Tracing** optional via `options.tracing`; useful for debugging complex sheets.
- **Python Binding** (`quilt-radio-orchestrator`) shows how the WASM‑compiled engine can be used from non‑JS hosts, registering sensor and listener cells and driving a playhead loop.

## Practical Takeaways for Integration
1. **Expose external state** as `sensor` cells; push updates from hardware adapters.
2. **Wrap custom logic** in `program` cells; they receive the engine as `ProgramRuntime`.
3. **Use `router` cells** for dispatch based on row/col tags – ideal for multi‑boat or multi‑sensor routing.
4. **Leverage AI cells** for LLM‑driven behavior; ensure an AI engine is passed to `QuiltEngine`.
5. **Federate** via `quilt-fleet` to sync sheets across devices.

## Pitfalls Noted
- Auto‑dependency detection only matches simple token patterns; complex templating requires explicit `deps`.
- Forgetting to clear `contextCache` on `set`/`push` leads to stale reads.
- When embedding in Python, ensure the host provides a proper Promise implementation for the WASM runtime.

---
*Generated from the deep‑dive session on 2026‑08‑21.*