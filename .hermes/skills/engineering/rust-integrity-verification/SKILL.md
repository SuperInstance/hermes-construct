---
name: rust-integrity-verification
description: Governs high-fidelity inspection, compilation auditing, and structural verification of Rust-based systems.
---

# Rust Debugging & Integrity Verification

Governs the high-fidelity inspection, compilation auditing, and structural verification of Rust-based agentic systems.

## 🎯 Purpose
To prevent "Ghost Code" scenarios where testing infrastructure references non-existent modules or crates, and to ensure that modularization efforts do not break the fundamental compilation unit of the engine.

## 🛠 Workflow

### 1. The "Crate Identity" Check
Before attempting any compilation or test run, verify the link between `Cargo.toml` and the `use` statements in the project.
- **Check:** Does `[package] name` in `Cargo.toml` match the crate name used in `tests/` or `examples/`?
- **Pitfall:** Mismatched crate names in `use` statements (e.g., calling it `core_rust` in tests when the package is named `engine-core`) will cause `E0433` errors.

### 2. The "Module Existence" Audit
When a compilation error (`E0422`) indicates a type is missing, do not assume the code is "broken"; assume the file is "missing."
- **Step:** Cross-reference `mod.rs` declarations with the actual filesystem.
- **Verification:** If `pub mod tripartite;` exists in `mod.rs`, the file `tripartite.rs` **must** exist in that same directory.
- **Pitfall:** Relying on outdated `mod.rs` files after a cleanup/modularization move is a primary cause of broken builds.

### 3. The "Integrity-First" Testing Pattern
Never assume a "Passed" status in an orchestrator session without local verification.
- **Pattern:**
    1. `cargo build` (to check structural integrity/linkage).
    2. `cargo test` (to check logic).
- **Rule:** If `cargo build` fails, `cargo test` is an invalid metric.

## 🛑 Pitfalls & Troubleshooting

| Error Code | Cause | Mitigation |
| :--- | :--- | :--- |
| `E0433` | Crate not found in `use` statement. | Match the `use` name to the `name` field in `Cargo.toml`. |
| `E0422` | Unresolved type/module. | Check if the module is `pub` and if the `.rs` file actually exists on disk. |
| `Command Timeout` | Massive dependency graph or slow compilation. | Use `cargo build --no-run` first to check for syntax/linkage errors. |
| `E0432` | Unresolved import (missing re-export). | Ensure `lib.rs` has `pub use` for all public types in submodules. |
| `E0422` (module) | `pub mod rest;` declared but `rest.rs` missing. | Create the missing `.rs` file before compiling. |
| Edition inheritance error | `edition` in `[workspace.lints]` instead of `[workspace.package]`. | Move `edition = "2021"` to `[workspace.package]`. |
| `schemars` feature not found | `uuid`/`chrono` lack `schemars` feature flags. | Use `#[schemars(with = "String")]` on `Uuid`/`DateTime<Utc>` fields. |
| Float test assertion failure | IEEE 754: `0.7999999999999999 != 0.8`. | Use epsilon: `(a - b).abs() < 1e-10` for `f64` equality. |

## Rust Test Construction Patterns (2026-03-19)

### "Know the API Before You Write Tests"
- **Pitfall**: Writing a 200-line integration test file before reading the actual struct definitions produces cascading field-name errors.
- **Pattern**: Read the target structs (`Equipment`, `EquipmentCost`, `EquipmentBenefit`, etc.) with `read_file` or `grep` first. Note every field name, type, and `Default` impl status (`SeedReference` does NOT implement `Default`).
- **Step**: Run `cargo test` immediately after writing the test file. If it fails with `E0560` (unknown field) or `E0277` (Default not satisfied), read the struct definition, fix the test, re-run.

### `try_reequip` Clears Triggers After Firing
- **Behavior**: `Claw::try_reequip()` evaluates all stored `MuscleMemory` patterns and **clears `reequip_triggers`** after evaluation (to prevent re-trigger loops).
- **Test implication**: A test that calls `unequip()` then `try_reequip()` and expects `results.len() == 1` **must construct equipment with non-empty `muscle_memory.triggers`** containing at least one `always` trigger or a matching condition. Empty triggers → empty results → assertion failure.
- **Minimal trigger for test**:
  ```rust
  MuscleMemoryTrigger {
      condition: MuscleMemoryCondition { r#type: "always".into(), ..Default::default() },
      action: "reequip".into(),
      priority: Some(1),
      cooldown_seconds: None,
  }
  ```

## Cargo Workspace Integrity Patterns (2026-08-18)

### Workspace Configuration
- **Edition inheritance**: Place `edition = "2021"` in `[workspace.package]`, NOT in `[workspace.lints]`. The latter causes "error inheriting edition" failures.
- **Clean rebuilds**: After modifying `Cargo.toml` workspace config, run `cargo clean` before `cargo check` to clear stale resolution caches.

### Crate Structure
- **lib.rs re-exports**: Each crate's `lib.rs` must `pub mod` all modules AND `pub use` all public types. Missing re-exports cause `E0432` (unresolved import) in downstream crates.
- **Missing modules**: If `lib.rs` declares `pub mod rest;` and `pub mod ws;`, those files MUST exist. Create them before compilation.
- **Import paths**: Use `claw_core::*` (external crate) not `crate::claw_core::*` (internal module) in dependent crates.

### schemars JsonSchema Derives
- **uuid/chrono**: These crates do NOT have `schemars` features. Use `#[schemars(with = "String")]` on `Uuid` and `DateTime<Utc>` fields.
- **Serde compatibility**: Keep `#[serde(with = "...")]` for serialization, add `#[schemars(with = "String")]` for schema generation.

### Testing
- **Float precision**: Use epsilon comparisons `(a - b).abs() < 1e-10` for `f64` equality in tests. Direct `assert_eq!` on `f64` fails due to IEEE 754 representation.
- **Schema validator tests**: Pass the inner object to validators, not the wrapper (e.g., `validate_claw(&config["claw"])` not `validate_claw(&config)`).

### Windows/MSYS Path Hygiene
- **write_file path resolution**: Relative paths like `/c/Users/...` can resolve to `C:\\\\c\\\\Users\\\\...` (extra `c`). Use absolute Windows paths `C:\\\\Users\\\\...` or verify with `terminal` after writes.
- **Verification pattern**: After any `write_file`, run `terminal cat <path>` to confirm content landed correctly.

### Python Subprocess Windows Path Escaping (2026-08-18)
- **Problem**: Windows paths passed to Python subprocess contain `\U` (e.g., `\Users`) which Python interprets as Unicode escape `\UXXXXXXXX`.
- **Fix**: Use raw strings in Python: `local_dir = r"C:\\\\Users\\\\..."` with double-escaped backslashes from Rust.
- **Pattern**: `let dest_str = dest.to_string_lossy().replace('\\\\', "\\\\\\\\"); format!(r#"local_dir = r\"{}\""#, dest_str)`

### Rust Edition for Async (2026-08-18)
- **Problem**: `async fn` not permitted in Rust 2015 edition.
- **Fix**: Ensure `edition = "2021"` in `[package]` section of `Cargo.toml`. Workspace root must not force edition 2015 via `[workspace.lints]`.
- **Verification**: `cargo check` should not emit `E0670: async fn is not permitted in Rust 2015`.

### qdrant_client Value API (2026-08-18)
- **Problem**: `Value` type has no `as_i64()` or `as_f64()` methods.
- **Fix**: Use `as_str()` and parse: `v.as_str().and_then(|s| s.parse::<u32>().ok())`.
- **Affected**: Payload deserialization in Qdrant search results.

### Model File Existence Check (2026-08-18)
- **Problem**: Model directory exists but is empty (failed download) → tokenizer load fails with "file not found".
- **Fix**: Check for actual model files, not just directory:
  ```rust
  let model_files_exist = model_path.join("config.json").exists() 
      && model_path.join("model.safetensors").exists() 
      && tokenizer_path.exists();
  ```

### Disk Space for Release Builds (2026-08-18)
- **Problem**: `cargo build --release` needs ~5GB free space for incremental compilation artifacts.
- **Fix**: Clean cargo cache and target dirs before build:
  ```bash
  rm -rf ~/.cargo/registry/cache/* ~/.cargo/registry/src/* ~/.cargo/git/*
  rm -rf target/
  ```

### CUDA/cudarc Version Compatibility (2026-08-18)
- **Problem**: CUDA 13.3 not supported by cudarc 0.13.9 (used by candle-core 0.11).
- **Workaround**: CPU fallback via `Device::Cpu` when `fallback_to_cpu=true`. GPU requires newer candle/cudarc or CUDA toolkit downgrade.
- **Config**: `[device] use_cuda = true, fallback_to_cpu = true` in embedder config.

### PyO3/maturin Build Pattern (2026-08-18)
- **Problem**: Building PyO3 extension modules requires specific Cargo.toml configuration.
- **Fix**: Add `[[lib]] crate-type = ["cdylib"]` to Cargo.toml. Use `pyo3` with `features = ["abi3-py38", "extension-module"]`. Build with `maturin build --release --features python` or `cargo build --release --features python` then `maturin develop --release`.
- **Windows**: Output is `.pyd` file (not `.so`). Must be installed via `pip install target/wheels/...` or copied to site-packages.
- **Common failure**: Missing `cdylib` crate-type causes wheel to build without the extension module (only Python stubs).

## 🛑 Pitfalls & Troubleshooting (continued)

| Error Code / Signal | Cause | Mitigation |
| :--- | :--- | :--- |
| `E0119` | Conflicting `impl Display` alongside `#[derive(Error)]` from `thiserror`. | Remove the manual `impl Display` — `thiserror::Error` generates it. |
| `E0277: DateTime<Utc>: JsonSchema` | `chrono` types lack `schemars` derives. | Annotate every `DateTime<Utc>` field with `#[schemars(with = "String")]`. |
| `E0277: Uuid: JsonSchema` | `uuid` crate lacks `schemars` feature. | Annotate every `Uuid` field with `#[schemars(with = "String")]`. |
| `E0277: Vec<(Uuid, String)>: JsonSchema` | Tuples containing `Uuid` fail schema derive. | Annotate the field: `#[schemars(with = "Vec<(String, String)>")]`. |
| `E0670: async fn not permitted` | `edition = "2021"` missing from `[package]` (not `[workspace.lints]`). | Move `edition = "2021"` to `[workspace.package]` or direct `[package]`. |
| `patch` tool mangling | Patch applied to a file modified since last read, or hunk failed to match. | Fall back to `write_file` for a complete clean rewrite. Do NOT retry `patch` on a corrupted file — it will compound the damage. |
| `write_file` lint "Pre-existing lint errors" | The lint diff shows import ordering noise, not necessarily a new error you introduced. | Trust `files_modified` list; re-run `cargo check` to confirm whether the write actually broke compilation. Lint diff output alone is not a failure signal. |
| `common.rs` reconstruction | When piecing a file back together from fragments, `patch` repeatedly fails because it cannot match mangled state. | Use `write_file` with the complete reconstructed content in one shot. Partial patches to macros + nested structs produce undefined state. |
| `delegate_task` timeout in compilation tasks | Compilation subagents hit the timeout and return no output. | Execute `cargo check` / `cargo test` directly in the parent session via `terminal`. Rust compile steps are deterministic and faster inline than via subagent. |
| `ResourceLimits` field mismatch | `common::ResourceLimits` has `max_memory_mb`/`max_cpu_percent`; do NOT use `memory_bytes` or `cpu_above` (those are in `ResourcePressure` inside `UnequipWhen`). Always `grep` the struct definition before accessing fields. |
| `TriggerConfiguration` field mismatch | `common::TriggerConfiguration` has `r#type` (serde-renamed from `type`), `enabled`, `config`; there is NO `periodic` field. Use `trigger_cfg.r#type.as_str()` for type matching. |
| `EquipmentSlot` variant naming | variants are `Memory`, `Reasoning`, `Consensus`, `Spreadsheet`, `Distillation`, `Perception`, `Coordination`, `Communication`, `SelfImprovement`, `Monitoring` — NOT `SpreadsheetIntegration` or `MultiAgentCoordination`. |
| **Dynamic Dispatch Type Inference** | When constructing a `HashMap` or collection of `Box<dyn Trait>`, Rust infers the type from the *first* insertion. If you insert `Box<ConcreteA>` first, the map becomes `HashMap<K, Box<ConcreteA>>`, causing a type mismatch when inserting `Box<ConcreteB>` even though both implement `Trait`. | **Fix:** Explicitly type the variable at declaration: `let mut modules: HashMap<Id, Box<dyn Trait>> = HashMap::new();`. |
| **Crate Visibility & Re-exports** | In a Cargo workspace, files in `examples/` or other crates depend on the *public API* of the crate defined in `src/lib.rs`. A struct marked `pub` in `src/foo.rs` is NOT accessible to examples unless it is explicitly re-exported in `src/lib.rs` via `pub use crate::foo::StructName;`. | **Fix:** Always check `lib.rs` for `pub use` statements when getting `E0432: unresolved import` errors in examples or downstream crates. |

## ⚡ Verification Cadence Rules
- **Compile after every write**, not just at the end. After `write_file` on any `.rs` file, run `cargo check --workspace` immediately.
- **Trust `cargo check` exit code**, not the lint diff output. A `0` exit with lint noise means the compile is clean; do not re-patch based on lint formatting hints alone.
- **One `write_file` per complete conceptual unit** (one file = one call). Partial writes via `patch` on complex Rust files with derive macros and inline comments produce mangled state that requires full file reconstruction.

## 📚 References
- `references/rust-compilation-audit-log.md` — Logs of failed compilation attempts and resolution paths.
- `references/rust-equipment-session-notes.md` — Session-specific notes from the dynamic equipment system implementation (2026-03-19): E0119/E0277 resolution, common.rs reconstruction, patch-tool mangling incident, delegation timeout pattern.
