# JSON Schema to Rust Types: Claw Implementation Reference

Reference implementation based on the `claw_agent.json` gold standard schema (SuperInstance, 2026-08-25).
Used during the successful port of the OpenCLAW monolith to a clean Rust workspace.

## 1. Context: The "Monolith First" Pattern
When dealing with a legacy repository containing both TypeScript source (`src/`) and node dependencies (`node_modules`), **do not** mix the new Rust crates at the root.
- **Isolation**: Create a subdirectory (e.g., `claw-engine/`) to act as the new Cargo workspace root.
- **Benefit**: `cargo check` runs instantly without traversing `node_modules`.

**Directory Structure:**
```text
claw/
├── src/                 # Legacy TypeScript (pi-embedded-runner, etc.)
├── node_modules/        # DO NOT INDEX
├── claw-engine/         # NEW Rust Workspace Root
│   ├── Cargo.toml       # Workspace Manifest
│   └── crates/
│       ├── claw-core/   # Pure types, schemas, traits (no async)
│       └── claw-runtime/# Tokio runtime, state management, async logic
```

## 2. Workspace Configuration (`Cargo.toml`)
Use the `workspace.package` and `workspace.dependencies` blocks to enforce consistency across crates.
This avoids "dependency hell" where `claw-core` uses `uuid 1.0` and `claw-runtime` uses `uuid 0.8`.

```toml
[workspace]
resolver = "2"
members = ["crates/*"]
exclude = ["../node_modules", "../src"]

[workspace.package]
version = "0.1.0"
edition = "2021"
authors = ["SuperInstance"]
license = "MIT"

[workspace.dependencies]
# Core Types
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde", "utc"] }
thiserror = "1.0"

# Runtime
tokio = { version = "1", features = ["full"] }
dashmap = "6.0" # Concurrent HashMap for state
tracing = "0.1"

# Internal
claw-core = { path = "crates/claw-core" }
```

**Crate Manifest (`claw-core/Cargo.toml`):**
```toml
[package]
name = "claw-core"
# inherit version, edition, authors from workspace
workspace = true

[dependencies]
serde.workspace = true
serde_json.workspace = true
uuid.workspace = true
chrono.workspace = true
thiserror.workspace = true
```

### 2.1 Dependency Management: Workspace vs Crate-Specific
- **Workspace Dependencies**: Define versions in `workspace.dependencies`.
- **Crate Usage**: Reference them in crate `Cargo.toml` with `serde.workspace = true`.
- **Crate-Specific Dependencies**: If a dependency is only used by one crate (e.g., `async-trait` for runtime logic, or `tracing-subscriber` for examples), add it directly with its version string:
  ```toml
  # In claw-runtime/Cargo.toml
  [dependencies]
  async-trait = "0.1"
  ```

## 3. Mapping Conventions (Schema → Rust)

| JSON Schema Construct | Rust Implementation | Crate | Notes |
| :--- | :--- | :--- | :--- |
| `string, format: uuid` | `::uuid::Uuid` | `uuid` | Ensure `features = ["serde"]` |
| `string, format: date-time` | `::chrono::DateTime<Utc>` | `chrono` | Ensure `features = ["serde", "utc"]` |
| `type: object` (Fixed Properties) | `pub struct` | `serde` | Derive `Serialize, Deserialize`. Use `#[serde(rename_all = "camelCase")]` if JSON uses camelCase. |
| `type: object, additionalProperties: true` | `serde_json::Map<String, Value>` or `HashMap<String, f64>` | `serde_json` | Use dynamic types for `thermodynamicState` ($\gamma, \eta, \delta$) unless schema is strictly typed. |
| `type: array, items: { ... }` | `Vec<T>` | `std` | |
| `type: string, enum: [...]` | `pub enum` | `serde` | Use `#[serde(rename_all = "snake_case")]` if variants differ from Rust naming conventions. |

### 3.1 Async Traits
If you are defining traits that include async methods (e.g., `ModelClient` with `async fn chat_completion`), you **must** use the `async-trait` crate.

**Cargo.toml:**
```toml
async-trait = "0.1"
```

**Code:**
```rust
use async_trait::async_trait;

#[async_trait]
pub trait ModelClient: Send + Sync {
    async fn chat_completion(
        &self, 
        prompt: &str, 
        config: &ModelConfiguration
    ) -> Result<InferenceResult, ModelError>;
}
```

## 4. Example Implementation: Claw Agent Core

**Schema Excerpt (`claw_agent.json`):**
```json
"claw_state": { "enum": ["idle", "thinking", "running", "waiting", "moulting", "error"] },
"molt_protocol": {
    "type": "object",
    "properties": {
        "molt_phase": { "enum": ["gamma_peak", "integrity_check", ...] },
        "eta_target": { "type": "number" }
    }
}
```

**Rust Code (`claw-core/src/types.rs`):**
```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClawState {
    Idle,
    Thinking,
    Running,
    Waiting,
    Moulting,
    Error,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MoltPhase {
    GammaPeak,
    IntegrityCheck,
    DeltaShaping,
    EtaStabilization,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoltProtocol {
    pub molt_phase: MoltPhase,
    pub eta_target: f64,
    // ... other fields
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClawAgent {
    pub id: Uuid,
    pub name: String,
    pub state: ClawState,
    
    /// Dynamic thermodynamic state (Gamma, Eta, Delta)
    /// Schema allows arbitrary numeric properties.
    #[serde(default)]
    pub thermodynamic_state: HashMap<String, f64>,

    pub molt_protocol: Option<MoltProtocol>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## 5. Verification
Always verify the workspace compiles before proceeding with logic implementation:
```bash
cd claw-engine
cargo check
```

### 5.1 Critical: Inline Linter vs. Cargo
The Hermes `write_file` tool includes an inline Rust linter that **often reports false positives**.
- **Issue**: It may complain about `async fn` requiring "Rust 2018" even though the `Cargo.toml` specifies `edition = "2021"`.
- **Rule**: If `cargo check` or `cargo build` succeeds, **ignore the inline linter errors**. Do not let false positives block your progress.

## 6. Gold Standard Schema Parity (Critical)

When working on ports, there is often a conflict between:
1.  **Existing Implementation Stubs** (e.g., `src/` directory from a previous port attempt).
2.  **Gold Standard Schemas** (e.g., `schemas_gold_standard/` directory).

**Rule: The Schema is always the Single Source of Truth.**

If the stub implements `ClawAgent { state: ClawState }` but the schema defines `ClawAgent { state: ThermodynamicState }` (where `ThermodynamicState` is a nested object with `gamma`, `eta`, `molt`, `phase`), **the stub is wrong**.

**Action:**
- Do NOT patch the runtime logic to match the stub (e.g., changing `self.agent.thermo.phase` to `self.agent.state.phase` where `state` is a flat struct).
- **DO** rewrite the stub types to match the schema (e.g., defining `struct ThermodynamicState` and updating `ClawAgent` to use it).

**Consequence of Ignoring:**
You will end up with a "Flat State" engine that cannot implement the "Thermodynamic Logic" (Molt Protocol, Entropy Gain, Potential Drain) defined in the Gold Standard Schema.
