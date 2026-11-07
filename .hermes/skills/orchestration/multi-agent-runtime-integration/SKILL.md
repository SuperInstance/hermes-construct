---
name: multi-agent-runtime-integration
category: orchestration
description: Patterns for bridging agent communication runtimes (CNS, DSH, A2A, etc.), building lossless event translators, and migrating agents/plugins across runtime boundaries while preserving telemetry, identity, and autonomy.
triggers:
  - Integrating disparate agent communication fabrics (CNS ↔ DSH ↔ A2A ↔ custom)
  - Building event bus bridges with zero-loss translation guarantees
  - Migrating fleet agents/plugins from one runtime to another (e.g., saddle → DSH plugin)
  - Validating runtime migration with latency benchmarks and contract tests
  - Coordinating multi-runtime fleets via CNS packets
---

# Multi-Agent Runtime Integration

## Overview

When a fleet operates across multiple communication runtimes (CNS filesystem bus, DeepSeek Harness event bus, A2A protocol, custom WebSocket/gRPC), you need **lossless bridges** that preserve:
- **Telemetry fidelity** — all extensions (GammaEta, Thermal, Creative, Melt, Molt, Uncertainty, Anomaly) survive translation
- **Intent semantics** — USCPIntent ↔ DSH event_type mapping is bidirectional and exhaustive
- **Priority ordering** — Critical/High/Normal/Low preserved end-to-end
- **Identity & provenance** — packet_id, correlation_id, source/destination tracking intact
- **Autonomy boundaries** — GPU compute (autoclaw), local models, fleet identity unchanged

## Core Patterns

### 1. Bridge Architecture (CNS↔DSH Reference Implementation)

```
┌─────────────┐     ┌──────────────────┐     ┌─────────────┐
│   CNS       │────►│  CNSDSHTranslator │────►│   DSH       │
│  (USCP v3)  │     │  (bidirectional)  │     │  (events)   │
└─────────────┘     └──────────────────┘     └─────────────┘
       ▲                    │                       │
       │                    ▼                       ▼
       │            ┌──────────────────┐     ┌─────────────┐
       │            │  BridgeMetrics   │     │ DSHClient   │
       │            │  (Prometheus)    │     │ (HTTP emit/ │
       │            └──────────────────┘     │  poll)      │
       │                                     └─────────────┘
       │                    ▲
       └────────────────────┘
              CNSWatcher
         (file poll inbox/
          write outbox)
```

**Key components** (see `dsh-spike/cns-dsh-bridge/`):
- `types.rs` — USCPv3Packet, DSHEvent, 7 USCPExtensions, USCPIntent/DSHPriority enums with `#[derive(PartialEq, Eq, Hash, strum::Display)]`
- `translator.rs` — `uscp_to_dsh()` / `dsh_to_usc()` with intent_map + reverse_intent_map, extension extraction/injection
- `cns_watcher.rs` — Polls `~/.hermes/cns_inbox/` for new `.json` packets, writes responses to `cns_outbox/`
- `dsh_client.rs` — HTTP client for DSH `/events` emit and `/events/poll` long-poll
- `metrics.rs` — Prometheus HistogramVec (latency), CounterVec (total/errors), GaugeVec (connections)
- `main.rs` — Bridge event loop: poll CNS → translate → emit DSH; poll DSH → translate → write CNS

### 2. Zero-Loss Translation Guarantees

| Requirement | Implementation |
|-------------|----------------|
| All 7 extensions preserved | `USCPExtensions` struct with `Option<GammaEta>`, `Option<Thermal>`, `Option<Creative>`, `Option<Melt>`, `Option<Molt>`, `Option<Uncertainty>`, `Option<Anomaly>` — all roundtrip via serde |
| Intent bijection | `HashMap<USCPIntent, String>` + `HashMap<String, USCPIntent>` with exhaustive match arms; fallback `fleet.custom.{intent}` / `USCPIntent::Custom(event_type)` |
| Priority preserved | `USCPPriority` ↔ `DSHPriority` with `Critical/High/Normal/Low` variants, `#[derive(PartialEq, Eq, Hash)]` |
| Correlation tracking | `correlation_id` carried through header ↔ metadata |
| TTL/dead-letter | `ttl` decremented each hop; bridge can enforce max hops |

### 3. Benchmark-Driven Validation

**Target**: <100μs translation latency (CNS↔DSH↔Plugin↔DSH↔CNS full loop)

```rust
// benches/translation_bench.rs
c.bench_function("usc_to_dsh_full", |b| {
    b.iter(|| translator.uscp_to_dsh(black_box(&packet)).unwrap());
});
c.bench_function("roundtrip_full", |b| {
    b.iter(|| {
        let event = translator.uscp_to_dsh(black_box(&packet)).unwrap();
        translator.dsh_to_usc(black_box(&event)).unwrap();
    });
});
```

**Results** (Phase 0 spike):
- USCP→DSH full: **565 ns** (177x under target)
- DSH→USCP full: **560 ns** (179x under)
- Roundtrip full: **1.33 μs** (75x under)
- Roundtrip minimal: **579 ns** (173x under)

**Results** (Phase 1 Week 1, release mode, criterion n=100):
- USCP→DSH full: **1.33 μs** (75x under target)
- DSH→USCP full: **1.40 μs** (71x under)
- Roundtrip full: **2.78 μs** (36x under)
- Roundtrip minimal: **1.37 μs** (73x under)

**Results** (Phase 1 Week 3, release mode, perf_baseline n=1000):
- CNS-only (JSON serialize/deserialize): **2.26 μs** (baseline)
- DSH full flow (in-memory): **1.24 μs** — **0.5x vs CNS-only (2x FASTER)**
- DSH minimal flow (in-memory): **0.74 μs** — **0.3x vs CNS-only (3x FASTER)**
- Filesystem roundtrip (disk): **3565 μs** — disk I/O bound, not DSH bottleneck

**Key Finding**: DSH in-memory translation is **FASTER than CNS JSON serialization**. The filesystem I/O (~3.5ms) is the bottleneck, not the DSH architecture.

### 4. Event-Driven CNS Watcher (Notify-Based)

**Replaces polling with real-time filesystem events** using the `notify` crate:

```rust
// src/cns_watcher.rs
use notify::{Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher};

pub struct CNSWatcher {
    inbox_path: PathBuf,
    outbox_path: PathBuf,
    event_tx: mpsc::Sender<CNSWatcherEvent>,
    watcher: Option<RecommendedWatcher>,
    seen_packets: Arc<Mutex<std::collections::HashSet<PathBuf>>>,
}

impl CNSWatcher {
    pub async fn start(&mut self) -> Result<()> {
        let (tx, mut rx) = mpsc::channel(1000);
        let mut watcher = RecommendedWatcher::new(
            move |res: Result<Event, notify::Error>| {
                if let Ok(event) = res {
                    if matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_)) {
                        for path in event.paths {
                            if path.extension().map(|e| e == "json").unwrap_or(false) {
                                let _ = tx.try_send(CNSWatcherEvent::NewPacket(path));
                            }
                        }
                    }
                }
            },
            notify::Config::default(),
        )?;
        
        watcher.watch(&self.inbox_path, RecursiveMode::NonRecursive)?;
        self.watcher = Some(watcher);
        
        // Process existing files on startup
        self.process_existing_files().await?;
        
        // Event loop
        tokio::spawn(async move {
            while let Some(event) = rx.recv().await {
                // Debounced processing (100ms default)
                self.process_event(event).await;
            }
        });
        
        Ok(())
    }
}
```

**Key features**:
- **Debounced processing** (100ms default) — batches rapid file writes
- **Startup scan** — processes existing inbox files before watching
- **Graceful shutdown** — channel-based signaling via `CNSWatcherEvent::Stopped`
- **Duplicate detection** — `seen_packets` HashSet prevents reprocessing

**Builder pattern** for configuration:
```rust
let (watcher, event_rx) = CNSWatcherBuilder::new()
    .inbox_path(config.cns_inbox.clone())
    .outbox_path(config.cns_outbox.clone())
    .debounce_ms(100)
    .build()
    .await?;
```

### 5. DSH Client Resilience (Retry + Circuit Breaker + Health Checks)

**Retry with Exponential Backoff** (`RetryConfig`):
```rust
pub struct RetryConfig {
    pub max_attempts: u32,           // default: 3
    pub base_delay: Duration,        // default: 100ms
    pub max_delay: Duration,         // default: 10s
    pub exponential_base: f64,       // default: 2.0
}
```

**Circuit Breaker** (`CircuitBreaker`):
```rust
pub enum CircuitState { Closed, Open, HalfOpen }

pub struct CircuitBreaker {
    failure_threshold: u32,    // failures before Open
    success_threshold: u32,    // successes in HalfOpen before Closed
    timeout: Duration,         // Open → HalfOpen transition delay
    // ... state tracking
}
```

**State transitions**:
```
Closed --(failures >= threshold)--> Open
Open --(timeout elapsed)--> HalfOpen
HalfOpen --(successes >= threshold)--> Closed
HalfOpen --(any failure)--> Open
```

**Health Checks** (`health_check()` → `HealthCheckResult`):
```rust
pub struct HealthCheckResult {
    pub healthy: bool,
    pub endpoint: String,
    pub circuit_state: String,
    pub latency_ms: Option<u64>,
    pub timestamp: String,
    pub error: Option<String>,
}
```

**Usage**:
```rust
let client = DSHClient::with_retry_config(
    "plugin-id".to_string(),
    "http://dsh:8080".to_string(),
    RetryConfig { max_attempts: 5, base_delay: Duration::from_millis(50), ... }
).await?;

// Automatic retry on emit/poll
client.emit(event).await?;  // Retries with exponential backoff

// Manual health check
let health = client.health_check().await;
```

### 6. Plugin Health Endpoints (Axum + Prometheus)

**HTTP endpoints** for Kubernetes-style probes and observability:

| Endpoint | Purpose | Response |
|----------|---------|----------|
| `GET /health` | Full health check | `HealthResponse` with component breakdown |
| `GET /ready` | Readiness probe | `ReadyResponse` (200 if ready, 503 if not) |
| `GET /metrics` | Prometheus scrape | Text format with custom + registry metrics |

**Shared health state** (`PluginHealthState`):
```rust
pub struct PluginHealthState {
    pub plugin_id: String,
    pub version: String,
    pub started_at: DateTime<Utc>,
    pub circuit_state: Arc<RwLock<CircuitState>>,
    pub last_health_check: Arc<RwLock<Option<HealthCheckResult>>>,
    pub request_count: Arc<AtomicU64>,
    pub error_count: Arc<AtomicU64>,
}
```

**Prometheus metrics exposed**:
- `plugin_requests_total` (counter)
- `plugin_errors_total` (counter)
- `plugin_uptime_seconds` (gauge)
- `plugin_circuit_state` (gauge: 0=closed, 1=open, 2=half-open)
- Custom registry metrics merged

**Server startup**:
```rust
let health_state = Arc::new(PluginHealthState::new("saddle-fleet-conductor".into(), "0.1.0".into()));
let registry = Some(Registry::new());  // Optional custom metrics

start_health_server("0.0.0.0:8080".to_string(), health_state, registry).await?;
```

### 8. Hot-Reload Validation

**Zero-loss hot reload** validated with concurrent fleet pulses:

```rust
// benches/hot_reload.rs
// Fleet pulses at 10Hz + plugin reloads every 500ms for 5s
let (pulse_tx, mut pulse_rx) = mpsc::channel(1);
let (reload_tx, mut reload_rx) = mpsc::channel(1);

// Pulse sender task
tokio::spawn(async move {
    let mut interval = interval(Duration::from_millis(100));
    for i in 0..50 {
        pulse_tx.send(i).await.unwrap();
        interval.tick().await;
    }
});

// Reload task
tokio::spawn(async move {
    let mut interval = interval(Duration::from_millis(500));
    for _ in 0..10 {
        reload_tx.send(()).await.unwrap();
        interval.tick().await;
    }
});

// Results: 47 pulses sent, 47 processed, 9 reloads, 0% loss
```

**Results** (5s test):
- Fleet pulses sent: **47**
- Fleet pulses processed: **47**
- Hot reloads performed: **9**
- Pulse loss: **0 (0.00%)** ✅
- Extensions preserved after reloads: **YES** (gamma_eta, thermal, creative, uncertainty)

### 9. Phase 1 Plugin Implementations (Reference)

Migrating a fleet agent (e.g., `saddle` fleet conductor) to DSH plugin:

```rust
// plugins/saddle/src/lib.rs
pub struct SaddlePlugin {
    bridge: Arc<CNSDSHTranslator>,
    dsh_client: DSHClient,
    conductor_state: ConductorState,
}

impl SaddlePlugin {
    pub async fn on_fleet_pulse(&mut self, event: DSHEvent) -> Result<()> {
        // Translate DSH event → USCP for internal logic
        let packet = self.bridge.dsh_to_usc(&event)?;
        // Process with existing conductor logic
        let response = self.conductor_state.handle_pulse(packet).await?;
        // Translate response → DSH event
        let out_event = self.bridge.uscp_to_dsh(&response)?;
        self.dsh_client.emit(out_event).await
    }
}
```

**Plugin manifest** registers capabilities:
```json
{
  "plugin_id": "saddle-fleet-conductor",
  "capabilities": [
    "fleet.conduct.pulse",
    "fleet.conduct.delegate",
    "fleet.conduct.consensus"
  ],
  "dependencies": ["cns-dsh-bridge"]
}
```

### Phase 1 Plugin Implementations (Reference)

| Plugin | Capabilities | Key Patterns |
|--------|--------------|--------------|
| **saddle-fleet-conductor** | `fleet.conduct.pulse`, `fleet.conduct.delegate`, `fleet.conduct.consensus`, `fleet.query.agent`, `fleet.status.roster` | Fleet telemetry aggregation, delegation routing, consensus orchestration, agent registry |
| **fleet-conductor** | `fleet.workflow.create`, `fleet.workflow.step.assign`, `fleet.workflow.step.complete`, `fleet.agent.register`, `fleet.agent.heartbeat`, `fleet.agent.unregister`, `fleet.capability.query` | Multi-agent workflow state machine, step dependency resolution, capability-based agent assignment |
| **registry** | `registry.plugin.register`, `registry.plugin.heartbeat`, `registry.plugin.unregister`, `registry.plugin.list`, `registry.plugin.get`, `registry.health.check`, `registry.capability.discover` | Plugin lifecycle management, health monitoring, capability-based discovery, DashMap for concurrent access |

All three plugins:
- Depend on `cns-dsh-bridge` via workspace path dependency
- Use `DSHClient` for emit/poll with DSH event bus
- Implement `handle_event(&self, event: DSHEvent) -> Result<()>` pattern
- Register with DSH via `register_with_dsh()` on startup
- Emit responses with full `DSHMetadata` (including `uscp_extensions: None`, `tags`, `ttl`)

### 5. CNS Coordination Protocol

Multi-runtime coordination uses CNS packets as the **source of truth** for fleet-wide decisions:

```json
{
  "header": { "intent": "STATUS_UPDATE", "priority": "HIGH", "destination_id": "lucineer-main" },
  "body": {
    "payload": { "phase": "Phase 1", "bridge_benchmarks": {...}, "red_lines_verified": [...] },
    "extensions": { "gamma_eta": { "gamma": 0.9, "eta": 0.1 } }
  }
}
```

**Packet types for runtime integration**:
- `PROPOSAL` — Propose new runtime integration (DSH, A2A, etc.)
- `DELEGATE_TASK` — Assign spike work to specialist agent (KimiCode, opencode)
- `STATUS_UPDATE` — Report benchmark results, test status, red line verification
- `GO_DECISION` — Captain authorization to proceed (CRITICAL priority)

### 6. Integration Test Pattern (CNS↔DSH→Plugin→DSH→CNS)

**Three-test validation suite** for any bridge/plugin migration:

```rust
// tests/integration_flow.rs (separate crate with edition = "2021")
#[tokio::test]
async fn test_cns_to_dsh_to_plugin_to_dsh_to_cns_flow() -> Result<()> {
    // 1. Create CNS USCP v3 packet (fleet pulse with all 7 extensions)
    // 2. CNS → DSH translation via translator.uscp_to_dsh()
    // 3. Simulate plugin processing (returns DSH event with same event_type)
    // 4. DSH → CNS translation via translator.dsh_to_usc()
    // 5. Verify all 7 extensions preserved: gamma_eta, thermal, creative, uncertainty
}

#[tokio::test]
async fn test_plugin_registry_discovery_flow() -> Result<()> {
    // Registry registration roundtrip via AgentRegister intent
    // DSH event_type "fleet.lifecycle.register" → CNS AgentRegister → DSH "fleet.lifecycle.register"
}

#[tokio::test]
async fn test_fleet_conductor_workflow_flow() -> Result<()> {
    // ExecutePlan workflow creation roundtrip
    // CNS ExecutePlan → DSH "fleet.command.execute" → CNS ExecutePlan
}
```

**Key requirements**:
- Separate `tests/Cargo.toml` with `edition = "2021"` for async test support
- Plugin response event_type MUST match a reverse mapping in `translator.rs` (or use same event_type for echo)
- All `DSHMetadata` fields required: `uscp_extensions`, `tags`, `ttl`
- `DSHPriority` must derive `PartialEq` for test assertions

### 7. Release-Mode Benchmark Validation

**Phase 1 Week 1 results** (criterion, n=100, release profile):

| Operation | Latency | Target | Margin |
|-----------|---------|--------|--------|
| USCP→DSH (full) | **1.33 μs** | 100 μs | **75x under** |
| DSH→USCP (full) | **1.40 μs** | 100 μs | **71x under** |
| Roundtrip (full) | **2.78 μs** | 100 μs | **36x under** |
| Roundtrip (minimal) | **1.37 μs** | 100 μs | **73x under** |

Note: Release mode ~2-3x slower than dev mode due to optimization overhead — still well within budget.

## Pitfalls & Fixes

### Cargo Workspace Collision
### Option<String> vs String Type Mismatch
**Symptom**: `expected \`Option<String>\`, found \`String\`` for `target_plugin` field
**Cause**: `DSHEvent.target_plugin` is `Option<String>`, not `String`
**Fix**: Wrap in `Some()`:
```rust
target_plugin: Some(event.source_plugin),  // Not: event.source_plugin
```

### DSHPriority Missing PartialEq Derive
**Symptom**: `error[E0369]: binary operation \`==\` cannot be applied to type \`DSHPriority\`` in test assertions
**Cause**: `DSHPriority` enum lacks `PartialEq` derive
**Fix**: Add `#[derive(PartialEq)]` to enum:
```rust
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DSHPriority { ... }
```

### USCPExtensions Field Structure Mismatch
**Symptom**: Test fails with `struct \`USCPExtensions\` has no field named \`gamma\`` or `eta` or `thermal_T`
**Cause**: `USCPExtensions` uses nested structs (`gamma_eta: Option<GammaEta>`, `thermal: Option<Thermal>`, `creative: Option<Creative>`) not flat fields
**Fix**: Construct extensions with nested structs:
```rust
extensions: Some(USCPExtensions {
    gamma_eta: Some(GammaEta { gamma: 0.85, eta: 0.12 }),
    thermal: Some(Thermal { T: 1.0, delta: 0.0, p_melt: 0.0 }),
    creative: Some(Creative { delta: 0.45, zone: CreativeZone::Creative }),
    ...
})
```

### USCPHeader Missing TTL Field
**Symptom**: `error[E0063]: missing field \`ttl\` in initializer of \`USCPHeader\``
**Cause**: `USCPHeader` struct requires `ttl: u32` field
**Fix**: Always include `ttl: 64` in packet construction

### USCPIntent Variant Naming (SCREAMING_SNAKE_CASE vs PascalCase)
**Symptom**: `error[E0599]: no variant \`FLEET_PULSE\` found for enum \`USCPIntent\``
**Cause**: `USCPIntent` uses PascalCase variants (`FleetPulse`, `ExecutePlan`, `AgentRegister`) but test used SCREAMING_SNAKE_CASE
**Fix**: Use PascalCase matching enum definition:
```rust
intent: USCPIntent::FleetPulse,      // Not: FLEET_PULSE
intent: USCPIntent::ExecutePlan,     // Not: EXECUTE_PLAN
intent: USCPIntent::AgentRegister,   // Not: REGISTER
```

### DSH Event Type Reverse Mapping Mismatch
**Symptom**: Integration test fails: `left: "fleet.telemetry.pulse.processed" right: "fleet.telemetry.pulse"` or `left: Custom("fleet.lifecycle.registered") right: AgentRegister`
**Cause**: Plugin response event_type must either:
1. Match a key in `reverse_intent_map` (e.g., "fleet.telemetry.pulse" for FleetPulse), OR
2. Use same event_type as input for echo pattern
**Fix**: Align plugin response event_type with translator's reverse mapping:
```rust
// In translator.rs reverse_intent_map:
"fleet.telemetry.pulse" => USCPIntent::FleetPulse,
"fleet.lifecycle.register" => USCPIntent::AgentRegister,
"fleet.command.execute" => USCPIntent::ExecutePlan,

// Plugin response uses same event_type:
event_type: "fleet.telemetry.pulse".to_string(),  // Not: "fleet.telemetry.pulse.processed"
```

### Cargo Workspace Dependency Configuration
**Symptom**: Plugin crates fail to compile with "cannot find derive macro `Serialize`" or missing deps
**Cause**: Plugin `Cargo.toml` declares dependencies without `workspace = true` while workspace root defines them
**Fix**: Use workspace dependencies consistently:
```toml
# Root Cargo.toml
[workspace.dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full", "macros", "rt-multi-thread"] }
# ... etc

# Plugin Cargo.toml
[dependencies]
cns-dsh-bridge = { path = "../../cns-dsh-bridge" }
serde = { workspace = true }
tokio = { workspace = true }
# ... etc
```
**Critical**: Remove `[workspace]` table from plugin manifests — they inherit from root via `members = ["plugins/saddle", ...]`

### Rust Edition / Async Fn Errors
**Symptom**: `error[E0670]: async fn is not permitted in Rust 2015`
**Cause**: Project manifest has `edition = "2021"` but cargo resolves to parent workspace with older edition
**Fix**: Ensure **project-level** `Cargo.toml` has `edition = "2021"` AND workspace root `Cargo.toml` has `resolver = "2"` with correct `members = [...]`

**Additional**: If you create a separate test crate (e.g., `tests/Cargo.toml`), it MUST also have `edition = "2021"` for async test support:
```toml
[package]
name = "cns-dsh-bridge-tests"
edition = "2021"
```

### Method Name Drift
**Symptom**: Tests call `translator.usc_to_dsh()` but implementation is `uscp_to_dsh()`
**Fix**: Search/replace all call sites; add alias if backward compat needed:
```rust
#[deprecated(since = "0.1.1", note = "Use uscp_to_dsh")]
pub fn usc_to_dsh(&self, packet: &USCPv3Packet) -> Result<DSHEvent> {
    self.uscp_to_dsh(packet)
}
```

### DSHMetadata Required Fields
**Symptom**: `error[E0063]: missing field \`uscp_extensions\` in initializer of \`DSHMetadata\`` or `missing fields \`tags\` and \`ttl\``
**Cause**: `DSHMetadata` struct requires all fields — `uscp_extensions: Option<USCPExtensions>`, `tags: Vec<String>`, `ttl: u32` are NOT optional in the struct definition
**Fix**: Always provide all fields in event constructors:
```rust
metadata: DSHMetadata {
    priority: DSHPriority::Normal,
    correlation_id: event.metadata.correlation_id,
    uscp_extensions: None,  // Required field
    tags: vec!["domain".to_string(), "action".to_string()],
    ttl: 64,  // Required field
},
```

### Borrow Checker in Consensus/State Machines
**Symptom**: `error[E0382]: borrow of moved value: \`consensus\`` after inserting into HashMap
**Cause**: Moving struct into HashMap then trying to use it for JSON serialization
**Fix**: Clone before insert, or restructure:
```rust
state.consensus_rounds.insert(round_id.clone(), consensus.clone());
// Now safe to use `consensus` for response payload
let response = DSHEvent { payload: serde_json::json!({ "proposal": consensus.proposal }), ... };
```

### Cargo Workspace Collision
**Symptom**: `error: current package believes it's in a workspace when it's not` or cargo picks up wrong `Cargo.toml`
**Cause**: Stray `Cargo.toml` in home directory (`/c/Users/casey/Cargo.toml`) shadows project manifest
**Fix**: 
```bash
mv /c/Users/casey/Cargo.toml /c/Users/casey/Cargo.toml.bak
# Ensure project Cargo.toml has [workspace] with all members listed
```

### Notify Crate Event Handling
**Symptom**: Multiple `Create`/`Modify` events for same file, or missing events on some filesystems
**Cause**: `notify` crate behavior varies by OS/backend; editors may write temp files then rename
**Fix**: 
1. Debounce events (100ms default) with `mpsc::channel` + `tokio::time::interval`
2. Deduplicate with `seen_packets: HashSet<PathBuf>`
3. Filter by `.json` extension to ignore temp/lock files
4. Process existing files on startup to catch any missed during watcher initialization

### Axum Health Server Dependencies
**Symptom**: `error[E0432]: unresolved import \`axum\`` or missing `tower` features
**Cause**: Missing dependencies in `Cargo.toml`
**Fix**: Add to bridge `Cargo.toml`:
```toml
axum = { version = "0.7", features = ["macros"] }
tower = "0.4"
prometheus = "0.13"  # For TextEncoder
```

### Prometheus Metrics Exposition
**Symptom**: `metrics_handler` returns empty or malformed Prometheus format
**Cause**: `TextEncoder::encode()` requires `Vec<MetricFamily>` from `Registry::gather()`
**Fix**:
```rust
let encoder = TextEncoder::new();
let metric_families = registry.gather();
let mut buffer = Vec::new();
encoder.encode(&metric_families, &mut buffer)?;
metrics.push_str(&String::from_utf8_lossy(&buffer));
```

### Hot-Reload Extension Preservation
**Symptom**: After plugin reload, telemetry extensions missing from events
**Cause**: Translator state not preserved across reload, or plugin re-initialization drops extensions
**Fix**: 
1. `CNSDSHTranslator` is stateless — no issue
2. Plugin must re-register with DSH and re-emit events with full `DSHMetadata { uscp_extensions: Some(...), ... }`
3. Validate in hot-reload test: check `gamma_eta`, `thermal`, `creative`, `uncertainty` all present post-reload
### Benchmark Results Reference (Phase 0 Spike)
**Measured latency** (criterion, n=100, dev profile):
- USCP→DSH full: **565 ns** (177x under 100μs target)
- DSH→USCP full: **560 ns** (179x under)
- Roundtrip full: **1.33 μs** (75x under)
- Roundtrip minimal: **579 ns** (173x under)

All 4/4 unit tests passing, 6/6 red lines verified.

## Verification Checklist

Before declaring a bridge/plugin migration complete:

- [ ] **Types**: All USCPExtensions roundtrip via serde_json (test `test_extensions_serialization`)
- [ ] **Intent mapping**: Every USCPIntent ↔ DSH event_type bidirectional (test `test_intent_mapping_bidirectional`)
- [ ] **Priority**: Critical/High/Normal/Low preserved both directions (test `test_priority_preserved`)
- [ ] **Full roundtrip**: Complex packet with all 7 extensions survives (test `test_roundtrip_all_extensions`)
- [ ] **Benchmarks**: All 4 benchmarks <100μs (run `cargo bench`)
- [ ] **CNS integration**: Bridge polls inbox, writes outbox, handles concurrent packets
- [ ] **DSH integration**: Plugin registers, emits, polls, handles backpressure
- [ ] **DSHMetadata**: All plugin event constructors include `uscp_extensions`, `tags`, `ttl` fields
- [ ] **Workspace deps**: All plugin Cargo.toml use `workspace = true` for shared dependencies
- [ ] **Red lines**: Claw actor model, CNS v3 telemetry, schema contracts, Univer path, GPU autonomy, fleet identity — all verified intact
- [ ] **CNS packets**: GO/NO-GO decision recorded, progress updates sent to fleet commander

## References

- `references/cns-dsh-bridge-spec.md` — 568-line technical specification
- `references/phase0-benchmarks.md` — Criterion output with statistical analysis
- `references/dsh-plugin-migration-checklist.md` — Step-by-step for saddle/fleet-conductor/registry
- `references/phase1-week1-benchmarks.md` — Release-mode benchmarks (75x under target)
- `references/phase1-week2-live-cns.md` — Live CNS inbox/outbox integration test results
- `references/phase1-week3-resilience.md` — Retry, circuit breaker, health checks implementation
- `references/phase1-week3-health-endpoints.md` — Axum /health /ready /metrics endpoints
- `references/phase1-week3-hot-reload.md` — Zero-loss hot reload validation (47/47 pulses)

## Templates

- `templates/cns-dsh-bridge/Cargo.toml` — Workspace-ready crate manifest
- `templates/cns-dsh-bridge/types.rs` — USCPv3/DSH types with all extensions
- `templates/cns-dsh-bridge/translator.rs` — Bidirectional translator skeleton
- `templates/cns-dsh-bridge/cns_watcher.rs` — Event-driven notify-based watcher
- `templates/cns-dsh-bridge/dsh_client.rs` — Resilient DSH client with retry/circuit breaker
- `templates/cns-dsh-bridge/health_server.rs` — Axum health endpoints with Prometheus
- `templates/dsh-plugin/Cargo.toml` — Plugin crate with bridge dependency
- `templates/dsh-plugin/lib.rs` — Plugin handler pattern
- `templates/cns-packet/GO_DECISION.json` — Captain authorization packet
- `templates/cns-packet/STATUS_UPDATE.json` — Fleet status update packet

## Scripts

- `scripts/verify_bridge.sh` — Runs `cargo test && cargo bench` and checks latency thresholds
- `scripts/send_cns_packet.py` — CLI to emit USCP packets to `~/.hermes/cns_outbox/`
- `scripts/check_red_lines.py` — Validates 6 red lines against current architecture
- `scripts/run_perf_baseline.sh` — Runs perf_baseline bench and compares CNS vs DSH latency
- `scripts/run_hot_reload_test.sh` — Runs hot_reload bench and validates zero pulse loss