# Consensus Engine Mapping

## Source (TypeScript) – `TripartiteDeliberation.ts`
```ts
export interface TripartiteDeliberation {
  participants: ClawAgent[];
  score: number;
  // ... other fields
}

export function computeScore(...): number {
  // algorithm description
}
```

## Target (Rust) – `tripartite.rs`
- Defined `pub struct TripartiteConsensusEngine` with fields:
  - `participants: Vec<ClawAgent>`
  - `score: f64`
- Implemented `pub fn compute_score(&self) -> f64` mirroring the TS logic.
- Added `ResonanceScore` struct for per‑agent weighting.
- Unit tests (`#[cfg(test)]`) verify equivalence with known TS test vectors.

## Key Translation Decisions
- TS `number` → Rust `f64` for floating‑point precision.
- Optional fields become `Option<T>`.
- Arrays become `Vec<T>`.
- Exported functions become `impl` methods.

## Pitfalls Encountered
- TS used `NaN` checks; Rust requires `is_nan()`.
- Ownership semantics required cloning of agents when stored in the engine.
- Ensure `serde` derives for JSON (de)serialization if needed.

---
*Generated from session 2026‑08‑17*