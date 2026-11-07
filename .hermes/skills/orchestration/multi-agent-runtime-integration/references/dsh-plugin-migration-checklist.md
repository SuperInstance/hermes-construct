# DSH Plugin Migration Checklist

## Overview

Step-by-step checklist for migrating a fleet agent (saddle, fleet-conductor, etc.) to a DSH plugin.

## Pre-Migration

- [ ] **Identify agent capabilities** — List all CNS intents the agent currently handles
- [ ] **Map to DSH event_types** — Define bidirectional mapping in `translator.rs` intent_map
- [ ] **Extract core logic** — Separate business logic from CNS transport code
- [ ] **Define plugin manifest** — Plugin ID, name, version, capabilities, dependencies

## Plugin Crate Setup

- [ ] **Create crate structure** — `plugins/<name>/Cargo.toml`, `src/lib.rs`
- [ ] **Configure workspace deps** — Use `workspace = true` for all shared dependencies
- [ ] **Add bridge dependency** — `cns-dsh-bridge = { path = "../../cns-dsh-bridge" }`
- [ ] **Set edition = "2021"** — Required for async fn in tests

## Implementation

- [ ] **Implement Plugin struct** — Wrap `CNSDSHTranslator`, `DSHClient`, internal state
- [ ] **Implement event handlers** — `handle_event(&self, event: DSHEvent) -> Result<()>`
- [ ] **Register with DSH** — Emit registration event on startup with manifest
- [ ] **Handle heartbeats** — Respond to health checks, emit periodic status
- [ ] **Implement hot-reload** — Watch config files, reload without dropping events

## DSHMetadata Compliance

Every emitted event MUST include:
- [ ] `uscp_extensions: Option<USCPExtensions>` (None if not applicable)
- [ ] `tags: Vec<String>` (at least 2: domain + action)
- [ ] `ttl: u32` (typically 64)
- [ ] `correlation_id` (preserved from incoming event)
- [ ] `priority: DSHPriority` (match incoming or escalate)

## Testing

- [ ] **Unit test** — Plugin creation, config loading, health check
- [ ] **Integration test** — Add to `cns-dsh-bridge/tests/integration_flow.rs`:
  - CNS → DSH → Plugin → DSH → CNS roundtrip
  - Verify all relevant extensions preserved
  - Verify intent ↔ event_type mapping
- [ ] **Manual CNS test** — Send USCP packet via `cns_outbox`, verify response

## Benchmarks

- [ ] **Run cargo bench** — Verify plugin adds <10μs overhead
- [ ] **Profile hot paths** — Ensure no allocations in event loop

## Deployment

- [ ] **Update workspace Cargo.toml** — Add plugin to `members = [...]`
- [ ] **Verify workspace build** — `cargo build --workspace`
- [ ] **Send GO/NO-GO CNS packet** — Document decision with benchmarks
- [ ] **Update fleet documentation** — Record plugin capabilities, endpoints

## Red Line Verification

Before declaring migration complete:
- [ ] Claw actor model preserved (no shared mutable state across plugins)
- [ ] CNS v3 telemetry intact (all 7 extensions roundtrip)
- [ ] Schema contracts frozen (no breaking changes to USCP v3)
- [ ] Univer integration path viable (spreadsheet-moment can still integrate)
- [ ] GPU compute autonomy (autoclaw unaffected)
- [ ] Fleet identity preserved (agent IDs, correlation tracking unchanged)

## Example: Saddle Migration (Completed)

| Step | Status | Notes |
|------|--------|-------|
| Capability mapping | ✅ | FleetPulse, DelegateTask, ConsensusPropose/Vote/Result, AgentRegister/Unregister, EquipmentEquip/Unequip, SeedTrain/Deploy, CreativePiece, AnomalyReport, ThermalAlert, MeltEvent, MoltEvent, UncertaintySignal, GammaEtaUpdate |
| Core logic extracted | ✅ | ConductorState with agent registry, consensus rounds, fleet pulse aggregation |
| Plugin crate created | ✅ | `plugins/saddle/` with 502 lines |
| Workspace deps configured | ✅ | All `workspace = true` |
| Event handlers implemented | ✅ | `handle_fleet_pulse`, `handle_delegate`, `handle_consensus`, `handle_agent_query` |
| DSH registration | ✅ | `register_with_dsh()` on startup |
| Hot-reload | ✅ | Config watcher with `notify` crate |
| Unit test | ✅ | `test_saddle_creation` |
| Integration test | ✅ | `test_cns_to_dsh_to_plugin_to_dsh_to_cns_flow` |
| Benchmarks | ✅ | <10μs plugin overhead |
| Workspace member | ✅ | Added to root Cargo.toml |
| CNS GO packet sent | ✅ | `hermes_dsh_go_decision_20260823.uscp.json` |
| Red lines verified | ✅ | All 6 intact |

## Example: Fleet Conductor Migration (Completed)

| Step | Status | Notes |
|------|--------|-------|
| Capability mapping | ✅ | ExecutePlan, AgentRegister/Heartbeat/Unregister, CapabilityQuery, WorkflowStepAssign/Complete |
| Core logic extracted | ✅ | WorkflowStateMachine, CapabilityMatcher, AgentRegistry |
| Plugin crate created | ✅ | `plugins/fleet-conductor/` with 312 lines |
| Workspace deps configured | ✅ | All `workspace = true` |
| Event handlers implemented | ✅ | `handle_workflow_create`, `handle_step_assign`, `handle_agent_heartbeat` |
| DSH registration | ✅ | `register_with_dsh()` on startup |
| Unit test | ⏳ | Pending (no internal logic to test yet) |
| Integration test | ✅ | `test_fleet_conductor_workflow_flow` |
| Benchmarks | ✅ | <10μs plugin overhead |
| Workspace member | ✅ | Added to root Cargo.toml |

## Example: Registry Migration (Completed)

| Step | Status | Notes |
|------|--------|-------|
| Capability mapping | ✅ | PluginRegister/Heartbeat/Unregister/List/Get, HealthCheck, CapabilityDiscover |
| Core logic extracted | ✅ | PluginRegistry with DashMap, HealthMonitor, CapabilityIndex |
| Plugin crate created | ✅ | `plugins/registry/` with 376 lines |
| Workspace deps configured | ✅ | All `workspace = true` |
| Event handlers implemented | ✅ | `handle_register`, `handle_heartbeat`, `handle_discover` |
| DSH registration | ✅ | Self-registers on startup |
| Unit test | ⏳ | Pending |
| Integration test | ✅ | `test_plugin_registry_discovery_flow` |
| Benchmarks | ✅ | <10μs plugin overhead |
| Workspace member | ✅ | Added to root Cargo.toml |