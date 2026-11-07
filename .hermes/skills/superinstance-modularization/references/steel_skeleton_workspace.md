# Steel Skeleton Workspace — Cargo Architecture Reference

## Overview
Three-crate workspace created at `/c/Users/casey/polln/claw/` implementing the foundational "Steel Skeleton" for the Claw cellular agent engine.

## Crate Structure

### claw-core (9 tests passing)
**Purpose**: Domain models with JsonSchema derives for validation layer
**Key modules**:
- `common.rs` — ModelProvider, ClawState, EquipmentSlot, Capability, ModelConfiguration, MemoryConfiguration, SeedReference
- `claw.rs` — ClawAgent, RuntimeConfiguration, AgentMetrics, AgentError
- `equipment.rs` — EquipmentModule, EquipmentCost, MuscleMemory, TriggerThresholds, EquipWhen/UnequipWhen
- `seed.rs` — ClawSeed, TriggerCondition, LearningStrategy, TrainingConfiguration, StabilizationMetrics, MuscleMemory
- `consensus.rs` — ResonanceScore, TripartiteConsensusEngine, ConsensusDecision, ConsensusConfig

**Design notes**:
- All Uuid/DateTime<Utc> fields use `#[schemars(with = "String")]` since uuid/chrono lack schemars features
- Floating-point test assertions use epsilon comparison (`abs() < 1e-10`) for ResonanceScore averages
- TripartiteConsensusEngine integration test validates 3-agent consensus with resonance scoring

### claw-api (0 tests, compiles clean)
**Purpose**: Async traits + transport types (REST + WebSocket)
**Traits**:
- `Agent` — start/stop/heartbeat/status
- `ClawAgent` — execute/equip/unequip/spawn_slave/request_consensus
- `BotAgent` — execute/set_interval
- `EquipmentManager` — list/equip/unequip/get_muscle_memory
- `SeedManager` — train/get_stabilization
- `ConsensusEngine` — deliberate

**Transport types**:
- `rest.rs` — CreateClawRequest, ExecuteTaskRequest, EquipRequest, ListClawsResponse, etc.
- `ws.rs` — ClientMessage, ServerMessage, Heartbeat, TaskRequest/Response

### claw-schema (4 tests passing)
**Purpose**: JSON Schema validation (Draft 7) via `jsonschema` crate
- SchemaValidator with lazy-compiled schemas via `once_cell`
- validate_claw/bot/seed/config() methods
- Embedded schemas expanded from original claw-schema.json

## Workspace Configuration
```toml
# Root Cargo.toml
resolver = "2"
edition = "2021"
workspace.members = ["claw-core", "claw-api", "claw-schema"]
workspace.dependencies = { serde, uuid, chrono, tokio, jsonschema, schemars, async-trait, tracing }
```

## Key Decisions
- Edition 2021 required for async fn in traits (not 2015)
- Workspace deps prevent version drift across crates
- schemars JsonSchema derives enable automatic schema generation from Rust types
- TripartiteConsensusEngine uses ResonanceScore (coherence, alignment, trust) with weighted average

## Verification
```bash
cd /c/Users/casey/polln/claw && cargo test --workspace
# 13 tests passing
```