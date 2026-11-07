# Integration Test Pattern for CNS↔DSH Bridge

## Overview

This document captures the three-test validation pattern used in Phase 1 Week 1 to verify the CNS→DSH→Plugin→DSH→CNS flow end-to-end.

## Test Structure

Located at: `cns-dsh-bridge/tests/integration_flow.rs`

### Test 1: Full Fleet Pulse Roundtrip
```rust
test_cns_to_dsh_to_plugin_to_dsh_to_cns_flow
```
- Creates USCP v3 packet with `FleetPulse` intent and all 7 extensions
- Translates CNS → DSH via `translator.uscp_to_dsh()`
- Simulates Saddle plugin processing (echoes same event_type)
- Translates DSH → CNS via `translator.dsh_to_usc()`
- Verifies all 7 extensions preserved: `gamma_eta`, `thermal`, `creative`, `uncertainty`

### Test 2: Plugin Registry Discovery
```rust
test_plugin_registry_discovery_flow
```
- Creates DSH event with `event_type: "fleet.lifecycle.register"`
- Translates DSH → CNS via `translator.dsh_to_usc()`
- Verifies CNS packet has `AgentRegister` intent
- Correlation ID preserved through roundtrip

### Test 3: Fleet Conductor Workflow
```rust
test_fleet_conductor_workflow_flow
```
- Creates USCP v3 packet with `ExecutePlan` intent
- Translates CNS → DSH via `translator.uscp_to_dsh()`
- Verifies DSH event_type is `"fleet.command.execute"`
- Simulates Fleet Conductor response with same event_type
- Translates DSH → CNS, verifies `ExecutePlan` intent preserved

## Key Requirements

1. **Separate test crate** with `edition = "2021"` for async test support
2. **DSHPriority must derive PartialEq** for `assert_eq!` in tests
3. **All DSHMetadata fields required**: `uscp_extensions`, `tags`, `ttl`
4. **Plugin response event_type must match reverse_intent_map** in translator
5. **USCPExtensions uses nested structs** (GammaEta, Thermal, Creative) not flat fields

## Running Tests

```bash
cargo test --manifest-path cns-dsh-bridge/Cargo.toml --test integration_flow
```

## Common Failures & Fixes

| Error | Fix |
|-------|-----|
| `DSHPriority` missing `PartialEq` | Add `#[derive(PartialEq)]` to enum |
| `USCPExtensions` missing `gamma` field | Use `gamma_eta: Some(GammaEta { gamma, eta })` |
| `USCPHeader` missing `ttl` | Add `ttl: 64` to header |
| `USCPIntent::FLEET_PULSE` not found | Use `USCPIntent::FleetPulse` (PascalCase) |
| Event type not in reverse map | Use event_type that exists in `reverse_intent_map` |