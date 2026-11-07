# Equipment System Implementation Session Notes (2026-03-19)
Subsystem: claw-core dynamic equip/unequip.

## Error Chain & Fixes

| Error | Location | Fix |
|-------|----------|-----|
| `patch` hunk-not-found mangled common.rs | common.rs | Used write_file for full rewrite; patch tool unsafe on partially-corrupted Rust files |
| `E0119: conflicting Display impl` | common.rs:261 | thiserror.Error generates Display automatically; remove any manual `impl Display for XxxError {}` |
| `E0277: DateTime<Utc>: JsonSchema not satisfied` (×5) | common.rs | Annotate each DateTime field with `#[schemars(with = "String")]` |
| `E0277: Uuid: JsonSchema not satisfied` | common.rs | Annotate each Uuid field with `#[schemars(with = "String")]` |
| `E0277: Vec<(Uuid,String)>: JsonSchema` | common.rs:458 votes | `#[schemars(with = "Vec<(String,String)>")]` on the field |
| `E0277: ModelConfiguration: Clone not satisfied` | common.rs:280 | Derive typo: `Claude` → `Clone` |
| `E0432: unresolved import` (×4) | claw-api/src/lib.rs | Removed 4 phantom re-exports: CapabilityMetrics, FallbackModel, ModelParameters, RateLimits |
| `E0433: ambiguous name Equipment` (×26) | claw-api/ | Duplicate `Equipment` struct defined in both `common.rs` and `equipment.rs`; removed duplicates from `common.rs` |
| `E0277: MemoryConfiguration: JsonSchema` | common.rs | Orphaned derive on removed struct; cleaned up orphaned derives after deduplication |
| Type mismatch: `ClawAgent` stores `Vec<EquipmentModule>` but `unequip()` accesses `.muscle_memory` | claw.rs | `EquipmentModule` is lightweight config, `Equipment` is the rich type; fixed storage field type |
| Test construction `Capability { ... }` as struct | lib.rs | `Capability` is an enum (`MemoryPersistence`, `Reasoning`, etc.); update tests to use enum variants |
| Test instantiation with invalid `ModelConfiguration` fields | lib.rs | Real fields: `provider`, `model_name`, `endpoint`, `api_key`, `temperature`, `max_tokens`; remove stale `parameters`/`rate_limits`/`fallback_models` |
| Dangling git submodule gitlink | repo index | Mode 160000 gitlink with no `.gitmodules`; removed with `git rm --cached <path>` to treat as regular subdirectory |
| Unrelated histories merge conflict | git push | Force push to `origin/clean-publish` after confirming remote is target |
| `patch` tool staleness validation failure | claw.rs | Re-read file immediately before patching; if still failing, use write_file |

## Key Lessons
- Patch retry on a corrupted file compounds damage; use write_file for full rewrite instead.
- write_file lint output ("Pre-existing lint errors") is cosmetic noise; trust cargo check exit code.
- delegate_task times out on compilation work; run cargo check inline via terminal.
- schemars annotations on Uuid/DateTime/tuples must be exhaustive and checked via grep before compile.
- Equipment API: equip(slot, module) -> Result with equipment Error; unequip returns both module and MuscleMemory tuple.
- **Duplicate type deduplication**: When 20+ ambiguous errors reference the same struct name across files, remove the duplicate from the common/shared module rather than adding N explicit imports.
- **Type storage alignment**: If `unequip()` accesses `.muscle_memory`, the storage MUST be `Vec<Equipment>` (rich type), not `Vec<EquipmentModule>` (lightweight config).
- **Test enum vs struct**: If `cargo test` fails with "no such field" on an enum-like name, check whether the type is an enum, not a struct.
- **Git submodule cleanup**: Dangling mode-160000 gitlinks with no `.gitmodules` should be removed from index (`git rm --cached`) to unblock git operations.
- **Force push protocol**: After unrelated-histories conflict on a feature branch, force push is acceptable after confirming the remote is the canonical target.

## Session 2 Additions (2026-03-19 post-compaction)

### Field Mismatch Traps
- `ResourceLimits` has `max_memory_mb`/`max_cpu_percent`/`max_concurrent_operations` — NOT `memory_bytes` or `cpu_above`. The latter exist only inside `ResourcePressure` within `UnequipWhen`.
- `TriggerConfiguration` has `r#type` (mapped from `type` via serde), `enabled`, `config` — NOT `periodic`. Use `trigger_cfg.r#type.as_str()` to match trigger types.
- `EquipmentSlot` variants: `Memory`, `Reasoning`, `Consensus`, `Spreadsheet`, `Distillation`, `Perception`, `Coordination`, `Communication`, `SelfImprovement`, `Monitoring`. Tests used `SpreadsheetIntegration` and `MultiAgentCoordination` — both invalid.

### Integration Test Patterns
- Write the **complete** test file in one `write_file` call once struct APIs are understood. Incremental `patch` on integration tests produces import-ordering lint noise and wastes turns.
- `try_reequip()` implementation **clears `reequip_triggers`** after evaluation. Tests expecting non-empty results must construct equipment with non-empty triggers (e.g. `"always"` trigger).
- `make_seed_ref()` helper should be used in tests; `SeedReference` does NOT implement `Default`, so `Default::default()` for that field in `ClawAgent::new()` is invalid. Provide an explicit `SeedReference { seed_id, parameters, version }`.
- Latency test `test_equip_unequip_latency_under_1ms` passes in debug builds; the equip/unequip path is a simple Vec remove/push — no hidden allocations.