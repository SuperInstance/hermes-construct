---
name: rust-logic-porting
description: Governs the high-fidelity translation of complex, multi-perspective logic from high-level, interpreted environments (TypeScript/JavaScript) into high-performance, memory-safe systems (Rust).
---

# Rust-to-TypeScript Logic Porting

**Role:** Logic Translation & Implementation Specialist

**Purpose:** 
Governs the high-fidelity translation of complex, multi-perspective logic (like the Tripartite Consensus Engine) from high-level, interpreted environments (TypeScript/JavaScript) into high-performance, memory-safe systems (Rust).

## 🛠 Porting Workflow

### 1. Logical Decomposition (The Blueprint)
Before writing a single line of Rust, decompose the TypeScript implementation into a mathematical/logical spec.
- **Identify Core Types:** Map TS `interface` and `type` definitions to Rust `struct` and `enum`.
- **Map State Transitions:** Identify all state changes and ensure they are represented as valid state machine transitions in Rust.
- **Pinpoint Side Effects:** Identify all I/O, network, or external state calls (to be handled via Traits in Rust).

### 2. The Translation Matrix

| TypeScript Concept | Rust Equivalent | Implementation Pattern |
| :--- | :--- | :--- |
| `interface` / `type` | `struct` / `enum` | Use `derive(Serialize, Deserialize)` for parity. |
| `async / await` | `tokio` / `async-trait` | Use `async_trait` for trait-based polymorphism. |
| `Map<K, V>` | `HashMap<K, V>` | Standard collection mapping. |
| `Record<string, unknown>` | `serde_json::Value` | Use `serde_json` for dynamic, schema-less payloads. |
| `number` (float) | `f64` | Standard 64-bit float. |
| `string` | `String` or `&str` | Use `String` for ownership, `&str` for transient views. |

### 3. Implementation Tiers

#### **Tier 1: Pure Logic (The Heart)**
- Implement the mathematical and decision-making core.
- **Constraint:** Zero dependencies on I/O or networking. Use dependency injection via Traits.

#### **Tier 2: The Integration Layer (The Muscle)**
- Implement the `async` wrappers, WebSocket listeners, and REST API endpoints.
- **Constraint:** Leverage `tokio` for high-concurrency handling.

### ⚠️ Pitfalls & Anti-Patterns

- **❌ Ignoring the Gold Standard Schema:** When a "Gold Standard Schema" (e.g., `schemas_gold_standard/claw_agent.json`) exists alongside existing stubs or code, **the Schema is the Single Source of Truth**. Do not patch your runtime logic to match an outdated stub (e.g., using a flat `ClawState` struct when the schema defines a rich `ThermodynamicState` object). **Rewrite the stub to match the schema.** Failure to do this results in a "Flat State" engine that cannot implement the "Thermodynamic Logic" (Molt Protocol, Entropy Gain, Potential Drain) defined in the standard.

- **❌ Avoiding the "JS-in-Rust" Trap:** Do not try to replicate the loose, dynamic nature of TypeScript in Rust. If the TS code uses `unknown` or `any` everywhere, don't copy the behavior; define the **actual** rigid types that the logic *should* be using.
- **❌ Ignoring Error Handling:** TS often relies on `try/catch` and returning `undefined`. In Rust, you **must** use `Result<T, E>` and `Option<T>` to make failure modes explicit.
- **❌ Forgetting Thread Safety:** When porting async logic, ensure all shared state is protected by `Arc<Mutex<T>>` or `Arc<RwLock<T>>`.
- **❌ Porting "open struct" conditions as nested `if let` chains:** When porting schema-defined condition blocks (e.g. `EquipWhen`, `UnequipWhen`, `MuscleMemoryTrigger`) from TS/JSON schemas, do NOT write a single monolithic `if` tree per method. Instead, model each condition as an enum variant (`state_change`, `metric_threshold`, `idle`, `always`) with targeted extractors. This preserves the schema's extensibility and matches the data model 1:1.
- **❌ Forgetting Thread Safety:** When porting async logic, ensure all shared state is protected by `Arc<Mutex<T>>` or `Arc<RwLock<T>>`.
- **❌ Porting "open struct" conditions as nested `if let` chains:** When porting schema-defined condition blocks (e.g. `EquipWhen`, `UnequipWhen`, `MuscleMemoryTrigger`) from TS/JSON schemas, do NOT write a single monolithic `if` tree per method. Instead, model each condition as an enum variant (`state_change`, `metric_threshold`, `idle`, `always`) with targeted extractors. This preserves the schema's extensibility and matches the data model 1:1.
- **❌ Forgetting cooldown guards in trigger evaluation:** TS condition objects often carry `cooldown_seconds`. When porting `try_reequip`-style trigger evaluators, the Rust implementation **must** check cooldown against a persisted `last_execution_time` timestamp before evaluating the condition body. Without this, triggers fire every scan cycle.

## 🔀 Porting Condition/Trigger Logic from Schema

When the source logic is defined declaratively (JSON schema, TS interfaces) rather than as imperative code, port it in two passes:

**Pass 1 — Rust types mirror the schema exactly.** Derive `Serialize, Deserialize, JsonSchema` for all condition structs. Do NOT add methods yet; just make the Rust types round-trip the JSON representation.

**Pass 2 — Evaluation as `match` on `condition.type`.** Write a `fn evaluate_trigger(&self, trigger: &Trigger, now: DateTime<Utc>) -> bool` that dispatches on `trigger.condition.r#type` ("state_change", "idle", "metric_threshold", "always"). Each branch reads only the fields it needs from `self`. This matches the schema's "type" + optional fields pattern without forcing every branch to handle every field.

**Pass 3 — Caller-visible contract.** Ensure the caller gets back a `Vec<MuscleMemory>` (the fired entries), and that fired entries are either cleared from storage or marked as consumed to prevent retrigger loops. Document this contract in the method docstring.

## ✅ Verification Checklist

- [ ] **Unit Test Parity:** Does the Rust implementation pass the equivalent logic tests defined in the TS implementation?
- [ ] **Typestate Validation:** Do the Rust types prevent invalid state transitions that the TS code allowed?
- [ ] **Performance Benchmark:** Is the Rust implementation significantly faster/more memory-efficient than the TS source?
