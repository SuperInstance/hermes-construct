---
name: quilt-reactive-runtime
title: Quilt Reactive Runtime
description: Class-level skill describing the core concepts, architecture, and integration patterns of the Quilt reactive spreadsheet engine and its radio-orchestrator extension.
---

## Overview
The **Quilt** ecosystem provides a language-agnostic, reactive runtime where every piece of state is a *cell*. Cells come in a fixed set of kinds (value, formula, api, program, router, ai, sensor, io, listener). A sheet is a JSON document that declares an array of `CellDef`s. Loading a sheet builds a dependency graph, and the engine evaluates cells lazily, propagating changes through the graph.

## Core Concepts
- **Cell** – atomic unit with `id`, `kind`, optional `deps`, and a definition payload.
- **Sheet** – JSON list of `CellDef`s; `engine.loadSheet(sheet)` resets state and constructs the graph.
- **Dependency Graph** – directed edges `A → B` (A depends on B). Formulas auto-detect deps; other kinds use explicit `deps`.
- **Propagation** – `set`/`push` marks dependents stale, clears per-context caches, and fires listeners.
- **CallerContext** – routing information (row/col/tags) used for per-context memoization.
- **Universal API** – `get`, `set`, `call`, `push`, `subscribe`.
- **Effectful Cells** – `api`, `program`, `router`, `ai` are evaluated once per context, deduped via an `inflight` map.
- **Tracing** – optional `options.tracing` records evaluation traces for debugging.

## Execution Flow (simplified)
1. `engine.loadSheet(sheet)` → clear state, create `Cell` objects, build forward & reverse edges.
2. `engine.get(id, ctx)` → dispatch by kind; formulas refresh deps then evaluate; effectful cells check cache or start async evaluation.
3. `engine.set(id, value)` / `engine.push(id, data)` → update value, clear caches, `notify` subscribers, then `propagate` downstream.
4. `engine.propagate` recursively marks dependents stale, clears caches, and fires any `listener` cells.
5. Subscriptions receive `(new, old)` values and can filter.

## Cell Kind Highlights
- **value.ts** – returns static `def.value`.
- **formula.ts** – parses a JS-like expression, evaluates lazily, caches per context.
- **api.ts** – async external call (e.g., HTTP fetch).
- **program.ts** – sandboxed script execution with access to the engine via `ProgramRuntime`.
- **router.ts** – dispatches based on input.
- **ai.ts** – wraps an LLM provider; resolves upstream cell values for templating.
- **sensor.ts / io.ts** – push‑based cells for external events.
- **listener.ts** – side‑effect cell triggered on watched cell changes.

## Integration Points
- **Embedding in other languages** – Python binding (`quilt-radio-orchestrator`) demonstrates using the WASM‑compiled engine via `QuiltEngine` class.
- **Radio Orchestrator** – loads a “radio‑theater” sheet, registers sensor and listener cells, and drives a playhead loop.
- **Federation** – `quilt-fleet` syncs cell updates across instances (workers, ESP32, Jetson).
- **Claw Integration** – expose Claw agent state as sensor cells; register Claw actions as program cells.
- **Spreadsheet‑Moment** – treat each spreadsheet column/row as a Quilt sheet; load via `engine.loadSheet`.

## Pitfalls & Best Practices
- **Cache invalidation** – always clear `contextCache` on `set`/`push` to avoid stale reads.
- **Effectful cell deduplication** – rely on the `inflight` map; do not manually duplicate async calls.
- **Dependency auto-detection** – only works for simple token matches; for complex templates use explicit `deps`.
- **Context handling** – ensure caller context includes routing keys (row/col) when using router or AI cells.
- **Tracing** – enable `options.tracing` during debugging; remember it adds overhead.
- **Cross-language bindings** – when using the WASM binding, ensure the host provides a compatible `Promise` implementation.
- **TypeScript build pitfalls** –
  - Missing required fields in `ai.ts` (`kind`, `ai_kind`, `provider`, `model`, `id`). Add them to `AICellConfig` or cast appropriately.
  - Status comparisons must use defined `CellStatus` literals (`'idle' | 'computing' | 'ready' | 'error' | 'stale'`). Replace erroneous `'pending'`/`'evaluating'` checks.
  - Ensure `tsconfig.json` includes needed libs (`"DOM"`) and node types (`"types": ["node"]`).
  - After fixing, run `npm run build` to generate the `dist/` bundle before using the WASM binding.
- **Demo sheet creation** – keep a minimal JSON sheet that exercises every cell kind (see `references/demo_sheet.md`). Load it with `engine.loadSheet` to sanity‑check the runtime after changes.

## References

### Recent Updates (2026‑08‑21)
- Added **demo sheet** and **runner script** to illustrate a minimal end‑to‑end sanity check after fixing TypeScript build issues. See `references/demo_sheet.md` for details.
- Updated the skill to note the required `kind`/`ai_kind` fields and correct `CellStatus` literals (`'idle'`, `'computing'`, `'ready'`, `'error'`, `'stale'`).
- Included guidance on fixing `tsconfig.json` (add `"DOM"` lib and node types) and on handling `__dirname` in ES‑modules.

## References
- `references/quilt_deep_dive.md` – session-specific deep dive notes and code excerpts.
- `templates/quilt_sheet_example.json` – minimal example sheet demonstrating all cell kinds.
- `scripts/verify_quilt_engine.py` – script that loads a sheet, runs a few `get`/`set` operations, and prints the resulting graph state.
