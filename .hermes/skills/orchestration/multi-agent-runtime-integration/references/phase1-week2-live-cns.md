# Phase 1 Week 2 — Live CNS Integration

**Date**: 2026-08-24  
**Focus**: Event-driven CNS watcher, live inbox/outbox flows, performance baseline, hot-reload validation

---

## Event-Driven CNS Watcher (notify-based)

**File**: `cns-dsh-bridge/src/cns_watcher.rs`

### Key Changes
- Replaced polling with `notify` crate real-time filesystem events
- Debounced processing (100ms default)
- Startup scan of existing inbox files
- Graceful shutdown via channel signaling
- Duplicate detection with `seen_packets` HashSet

### Builder Pattern
```rust
let (watcher, event_rx) = CNSWatcherBuilder::new()
    .inbox_path(config.cns_inbox.clone())
    .outbox_path(config.cns_outbox.clone())
    .debounce_ms(100)
    .build()
    .await?;
```

### Events
```rust
pub enum CNSWatcherEvent {
    NewPacket(PathBuf),
    Stopped,
}
```

---

## Live CNS Integration Tests

**File**: `cns-dsh-bridge/tests/live_cns_integration.rs`

### Tests (3/3 passing)
1. `test_live_cns_inbox_outbox_flow` — Real filesystem roundtrip
2. `test_live_fleet_conductor_workflow` — ExecutePlan workflow via live CNS
3. `test_live_plugin_registry_registration` — AgentRegister via live CNS

### Test Setup
- Uses actual `~/.hermes/cns_inbox/` and `~/.hermes/cns_outbox/`
- Creates test packets, waits for bridge processing, validates responses
- Cleans up test files after completion

---

## Performance Baseline (CNS-only vs DSH Flow)

**File**: `cns-dsh-bridge/benches/perf_baseline.rs`

### Results (n=1000, release mode)

| Path | Latency | vs CNS-only |
|------|---------|-------------|
| CNS-only (JSON serialize/deserialize) | **2.26 μs** | baseline |
| DSH full flow (in-memory) | **1.24 μs** | **0.5x (2x FASTER)** |
| DSH minimal flow (in-memory) | **0.74 μs** | **0.3x (3x FASTER)** |
| Filesystem roundtrip (disk) | **3565 μs** | 1575x (disk I/O bound) |

### Key Finding
> **DSH in-memory translation is FASTER than CNS JSON serialization.** The filesystem I/O (~3.5ms) is the bottleneck, not the DSH architecture.

---

## Hot-Reload Validation

**File**: `cns-dsh-bridge/benches/hot_reload.rs`

### Test Configuration
- Fleet pulses: 10Hz (100ms interval)
- Hot reloads: every 500ms
- Duration: 5 seconds

### Results
- Fleet pulses sent: **47**
- Fleet pulses processed: **47**
- Hot reloads performed: **9**
- Pulse loss: **0 (0.00%)** ✅
- Extensions preserved: **YES** (gamma_eta, thermal, creative, uncertainty)

---

## CNS Packets Sent

1. Phase 1 Week 2 complete (live CNS flows)
2. Phase 1 Week 2 updated (event-driven watcher)
3. Phase 1 Week 2 updated (perf baseline + hot reload)