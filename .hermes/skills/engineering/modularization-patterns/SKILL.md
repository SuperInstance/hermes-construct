---
name: modularization-patterns
description: Governs the process of decomposing monolithic repositories into lightweight, specialized, and extensible modules.
---

# Modularization & Extraction Patterns

Governs the process of decomposing monolithic repositories into lightweight, specialized, and extensible modules.

## Overview
The SuperInstance ecosystem uses a "Neuro-Git" architecture. To maintain performance and clarity, large, bloated repositories must be periodically audited and decomposed into:
1. **Core Repositories**: Minimal, high-performance engines (`claw`, `constrainttheory`, `spreadsheet-moment`).
2. **Extension Repositories**: Specialized, lazy-loaded modules (`claw-extensions`, `constrainttheory-ml-demo`, `equipment-lucineer`).
3. **Supporting Repositories**: Research and formal validation (`SuperInstance-papers`).

## The Extraction Workflow

### 0. Safety Net (Non-Negotiable on Windows)
**CRITICAL**: Before moving, deleting, or modifying any directory structure, create a local backup of the entire "bloat" or target extraction directory.
- **Pattern**: `mkdir -p /c/Users/casey/<repo>_backup_safety && mv /c/Users/casey/polln/<repo>/<bloat_dir> /c/Users/casey/<repo>_backup_safety/`
- **Rationale**: Windows filesystem operations (especially via MSYS/Git Bash) can exhibit "Ghost Write" behavior (0-byte files) or file-locking issues on `node_modules` or vendor directories. A local filesystem backup allows instant rollback without git history manipulation.
- **Verification**: After moving, verify the backup exists with `ls` and verify the core repository is still intact.
- **Example**: During `claw/` cleanup (2026-08-25), `apps/`, `ui/`, `Swabble/`, and `vendor/` (total ~100k+ files) were moved to `C:\Users\casey\claw_backup_safety\` **before** any `Cargo.toml` was touched or any legacy code was analyzed.

### 1. Audit & Discovery (The "Compass" Phase)
Perform a deep scan of the target repository to identify "Logical Bloat."
- **Content Bloat**: Research PDFs, large assets, or documentation not central to the core mission.
- **Logic Bloat**: Hardware integrations, specific third-party providers, or edge-case automation tools that are tightly coupled to the core engine.
- **Tooling**: Use `grep` with specific keywords (e.g., `hardware`, `market`, `provider`, `extension`) and `find` to map the "Synaptic Topology" of the feature.

### 2. Contract Definition (The "Handshake" Phase)
Before moving any code, define a formal, technology-agnostic interface (the **Contract**) that allows the core to interact with the new module without knowing its implementation details.
- **Pattern**: Use TypeScript/Rust traits or interfaces to define the capability (e.g., `IEquipmentModule`, `IIntelligenceProvider`).
- **Goal**: Ensure the core remains "dumb" to the specifics of the extension.

### 3. Implementation & Adapter Creation
- Create the new repository.
- Implement the "Adapter" (the implementation of the Contract) within the new repository.
- Move the legacy code into the adapter.

### 4. Integration & Decoupling
- Update the core to inject the new module via the defined interface.
- Verify that the core can function without the extracted logic (by using mocks or stubs of the new interface).

## Pitfalls to Avoid

- **Shallow Extraction**: Do not just move files; move the *responsibility*. If you move code but the core still has deep dependencies on its internal types, you haven't modularized; you've just moved the bloat.
- **Residual Coupling**: Even if code is moved, the core must not maintain deep dependencies on the module's internal types. The core should interact only with the defined interface.
- **Bloated Dependency Trees**: If an extracted module brings back the entire monorepo's dependency graph, the extraction has failed. Modules must be lightweight and self-contained.
- **Granular Reusability**: A module should be useful as its own "unit of intelligence." If it cannot be used in a separate, smaller project, it is still too tightly coupled to the host ecosystem.
- **Broken Synapses**: Always ensure that the "Connection" (the injector or the factory) is updated. The core should call `module.initialize()` rather than assuming the hardware exists.
- **Missing the Topology**: Always build a "Compass" (a mental or documented map of the ecosystem) before starting. Moving a piece of a puzzle is easier if you know what the rest of the picture looks like.
- **The "Ghost Write" (Proxy Redirection) Pitfall**: When performing massive rewrites of documentation or configuration files in a Windows environment, the terminal's `write_file` or `cat > file` operations might be intercepted by a proxy that writes 0-byte or malformed files. This results in successful-looking tool calls that actually delete or corrupt your code.
    - **Mitigation**: Never trust a simple `write_file` for high-density documentation rewrites. Use the `terminal` tool to execute a direct shell command (e.g., `cat << 'EOF' > path/to/file`) or `printf` to inject the content directly into the filesystem. Immediately follow any massive write with a `cat` or `wc -c` command to verify the physical byte count on the disk and ensure the content is not truncated or empty.
- **The "Git Desync" Trap**: If multiple write attempts fail or result in 0-byte files, `git status` may report a "clean working tree" because it thinks nothing has changed relative to the corrupted state.
    - **Mitigation**: If you suspect a "Ghost Write," perform a "Nuclear Rewrite": use absolute paths in the `terminal`, use high-integrity shell injection (`cat << 'EOF'`), and manually verify the file contents with `cat` before attempting a `git commit`.
- **The "Phantom Plan" Error**: Never execute a modularization plan based on a document (like `CLAW_CORE_STREAMLINING_PLAN.md`) if the physical file layout of the repository does not match the plan's assumptions. If the logical structure in the plan (e.g., `core/src/`) contradicts the actual directory tree (e.g., `src/agents/`), the plan is invalid.
    - **Fix**: Stop immediately. Perform "Architectural Reconnaissance" (mapping the real topology) before any deletion or movement occurs. Re-orient the plan to the *actual* "Synaptic Topology" of the code.
- **The "Nihilistic Scavenger" Error**: When performing a scan for assets (like PDFs) that returns zero results, do not simply stop. A modularization agent should still record a "Null Scan" log in the `ai-writings` repository to document the successful verification of a clean state. This preserves the "Synaptic Trace" of the audit.
- **The "Direct Shell Injection" Standard**: When performing high-density documentation rewrites or critical schema moves in a Windows/MSYS environment, do NOT rely on the default `write_file` or simple `cat > file` redirection, as they can be prone to silent "Ghost Writes" or path-resolution errors.
    - **Mitigation**: Always use the `terminal` tool to execute a high-integrity shell injection (e.g., `cat << 'EOF' > path/to/file`) using absolute paths. Immediately follow any massive write with a `cat` or `wc -c` command to verify the physical byte count on the disk and ensure the content is not truncated or empty.
- **The "Marine-First" Semantic Guardrail**: During modularization of engine-level components (like `claw`), every new schema, module, or interface must be evaluated through the lens of maritime autonomy (e.g., support for high-latency comms, sensor-to-semantic translation, and multi-agent consensus). A module that cannot integrate with `boat-agent` or `VaaS` goals is a failure of design.
- **The "Killer README" Standard (The Manifesto Requirement)**: Modularization is a semantic reorganization, not just a file-system operation. Every extracted repository **MUST** be accompanied by a high-density `README.md` that serves as an authoritative technical and philosophical anchor.
    An acceptable "Killer README" must include:
    1. **🌌 Mission Statement**: High-level, evocative declaration of purpose.
    2. **🏛️ Philosophical Foundation**: The "Why" (Resonance, Provenance, Minimalism).
    3. **🧬 The Manifest**: Immutable design principles.
    4. **🛠️ Operational Workflows**: Both "Optimal Flow" (standard integration) and "Specialized Flow" (domain-specific tuning).
    5. **🌐 Ecosystem Connectivity**: Hyperlinked semantic web of related repos.
    6. **🛡️ Governance**: Compliance with schemas and the Trace Protocol.

## Cargo Workspace Modularization Patterns (2026-08-18)

When modularizing into a Cargo workspace (as done for `claw/` → `claw-core`/`claw-api`/`claw-schema`):

### Workspace Configuration
- **Edition inheritance**: Place `edition = "2021"` in `[workspace.package]`, NOT in `[workspace.lints]`. The latter causes "error inheriting edition" failures.
- **Dependency features**: `uuid` and `chrono` do NOT have `schemars` features. Use `#[schemars(with = "String")]` on `Uuid` and `DateTime<Utc>` fields instead.
- **Clean rebuilds**: After modifying `Cargo.toml` workspace config, run `cargo clean` before `cargo check` to clear stale resolution caches.

### Crate Structure
- **lib.rs re-exports**: Each crate's `lib.rs` must `pub mod` all modules AND `pub use` all public types. Missing re-exports cause `E0432` (unresolved import) in downstream crates.
- **Missing modules**: If `lib.rs` declares `pub mod rest;` and `pub mod ws;`, those files MUST exist. Create them before compilation.
- **Import paths**: Use `claw_core::*` (external crate) not `crate::claw_core::*` (internal module) in dependent crates.

### Testing
- **Float precision**: Use epsilon comparisons `(a - b).abs() < 1e-10` for `f64` equality in tests. Direct `assert_eq!` on `f64` fails due to IEEE 754 representation (e.g., `0.7999999999999999 != 0.8`).
- **Schema validator tests**: Pass the inner object to validators, not the wrapper (e.g., `validate_claw(&config["claw"])` not `validate_claw(&config)`).

### Rust Workspace Healing Patterns (2026-08-19)
- **Patch staleness**: `patch` tool fails validation when the file changes between read and patch. **Fix**: Re-read the file immediately before patching. If validation still fails (mangled diffs), fall back to `write_file` or Python in-place editing via `terminal`, then rerun `cargo check`.
- **Duplicate type cascade**: When `cargo check` reports 20+ ambiguous name errors for the same struct across multiple files (e.g., `Equipment` in both `common.rs` and `equipment.rs`), **remove the duplicate from the shared/common module** rather than adding explicit imports everywhere.
- **Orphaned derive cleanup**: After removing duplicate structs, orphaned `#[derive(...)]` lines above struct definitions cause "no such struct" errors. Grep for `^#\[derive` and verify the following `pub struct` exists.
- **Type mismatch propagation**: If `ClawAgent` stores `Vec<EquipmentModule>` but `unequip()` accesses `.muscle_memory`, the bug is upstream — `EquipmentModule` is a lightweight config, `Equipment` is the rich type. Fix the storage field, not the accessor.
- **Test type alignment**: When tests construct `Capability { ... }` as a struct but `Capability` is an enum (`MemoryPersistence`, `Reasoning`, etc.), update tests to use enum variants, not struct literals.
- **ModelConfiguration field drift**: `ModelConfiguration` fields are `provider`, `model_name`, `endpoint`, `api_key`, `temperature`, `max_tokens`. Tests using `parameters`/`rate_limits`/`fallback_models` are stale; trim to real fields.

### Windows/MSYS Path Hygiene
- **write_file path resolution**: Relative paths like `/c/Users/...` can resolve to `C:\\\\c\\\\Users\\\\...` (extra `c`). Use absolute Windows paths `C:\\\\Users\\\\...` or verify with `terminal` after writes.
- **Verification pattern**: After any `write_file`, run `terminal cat <path>` to confirm content landed correctly.
- **Git submodule gitlink cleanup**: When a submodule gitlink is dangling (mode 160000 with no `.gitmodules`, no `git rev-parse` resolution), remove the gitlink from the index (`git rm --cached <path>`) and commit to treat it as a regular subdirectory. Unrelated-histories force-push is acceptable after confirming the remote is the target.

### Disk Space Recovery Patterns (2026-08-18)
- **Problem**: Cargo/rustup operations fail silently at 95%+ disk usage. Need ~5GB free for release builds.
- **Analysis**: Use PowerShell to scan known heavy directories:
  ```powershell
  $dirs = @('.cargo', '.rustup', 'node_modules', 'VirtualBox VMs', '.ollama', 'polln', '.docker', 'OneDrive', 'Downloads', 'Documents')
  foreach ($d in $dirs) { $path = "C:\Users\casey\$d"; if (Test-Path $path) { $size = (Get-ChildItem $path -Recurse -ErrorAction SilentlyContinue | Measure-Object -Property Length -Sum).Sum / 1GB; Write-Host "$d : $([math]::Round($size,2)) GB" } }
  ```
- **Safe Cleanup Targets**:
  - `~/.cargo/registry/cache/*`, `src/*`, `git/*` — re-downloads on build
  - All `target/` directories — `cargo clean` or recursive remove
  - `.ollama` models — `ollama list` → `ollama rm <unused>`
  - `.docker` — `docker system prune -a --volumes`
  - `Downloads` — OS images, duplicate installers, large media
  - `Documents\ComfyUI` — often duplicated from primary install location
- **Recovery**: 80–120 GB typical on development machines

## Reference: The Ecosystem Map
*See `references/ecosystem_topology.md` for the current SuperInstance structural guide.*
*See `references/modularization_audit_session_2026_07_14.md` for recent audit findings and Windows-specific workflows.*
*See `references/lucineer_decoupling_2026_07_15.md` for the successful extraction pattern of hardware modules.*
*See `references/genesis_pulse_verification_2026_07_16.md` for the successful verification of the decoupled hardware bridge via the Genesis Pulse test.*
