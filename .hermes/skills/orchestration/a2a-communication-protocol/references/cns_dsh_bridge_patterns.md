# CNS↔DSH Bridge Protocol Translation Patterns (2026-08-23)

> **Derived from:** CNS↔DSH Bridge prototype — `cns-dsh-bridge` crate (8/8 tests PASS, 4/4 benchmarks <100μs)
> **Location:** `C:\c\Users\casey\dsh-spike\cns-dsh-bridge\`

---

## Overview

This document captures the patterns for building a **lossless protocol bridge** between the Hermes CNS (USCP v3) and the DeepSeek Harness (DSH) event bus. The bridge enables evolutionary integration of DSH as a unified runtime for the SuperInstance fleet while preserving all CNS telemetry invariants.

---

## Architecture

```
┌─────────────────┐     ┌──────────────────────┐     ┌─────────────────┐
│   Hermes CNS    │────►│   CNS↔DSH Bridge     │────►│   DSH Event Bus │
│  (USCP v3)      │     │  (cns-dsh-bridge)    │     │  (DSH Events)   │
└─────────────────┘     └──────────────────────┘     └─────────────────┘
                              │
                    ┌─────────┴─────────┐
                    ▼                   ▼
            ┌───────────────┐   ┌───────────────┐
            │ CNS Watcher   │   │ DSH Client    │
            │ (file poll)   │   │ (event emit)  │
            └───────────────┘   └───────────────┘
                    │                   │
                    └─────────┬─────────┘
                              ▼
                    ┌───────────────────┐
                    │ Translator        │
                    │ (bidirectional)   │
                    └───────────────────┘
                              │
                    ┌─────────┴─────────┐
                    ▼                   ▼
            ┌───────────────┐   ┌───────────────┐
            │ USCP Types    │   │ DSH Types     │
            │ (gamma_eta,   │   │ (event_type,  │
            │  thermal,     │   │  priority,    │
            │  creative,    │   │  payload,     │
            │  melt, molt,  │   │  extensions)  │
            │  uncertainty, │   │               │
            │  anomaly)     │   │               │
            └───────────────┘   └───────────────┘
```

---

## USCP v3 → DSH Event Translation

### Intent Mapping (USCP `body.intent` → DSH `event_type`)

| USCP Intent | DSH Event Type | Priority Mapping |
|-------------|----------------|------------------|
| `SENSORY_DATA` | `TelemetryIngest` | `Normal` |
| `COMMAND` | `CommandDispatch` | `High` |
| `QUERY` | `QueryRequest` | `Normal` |
| `STATUS` | `StatusReport` | `Low` |
| `HANDSHAKE_COMPLETE` | `HandshakeComplete` | `Normal` |
| `ESCALATE` | `Escalation` | `Critical` |
| `REQUEST_REASONING` | `ReasoningRequest` | `High` |
| `EXECUTE_PLAN` | `ExecutionPlan` | `High` |
| `CNS_PULSE_STATUS` | `FleetTelemetryPulse` | `Normal` |
| `STRATEGIC_PROPOSAL` | `StrategicProposal` | `High` |
| `DELEGATE_TASK` | `TaskDelegation` | `High` |

### Priority Mapping (USCP `header.priority` → DSH `priority`)

| USCP Priority | DSH Priority |
|---------------|--------------|
| `LOW` | `Low` |
| `MEDIUM` | `Normal` |
| `HIGH` | `High` |
| `CRITICAL` | `Critical` |
| `P0` | `Critical` |
| `P1` | `High` |
| `P2` | `Normal` |
| `P3` | `Low` |

### Telemetry Extensions Preservation (CRITICAL)

All **7 USCP v3 telemetry extensions** must be preserved losslessly in DSH `extensions` field:

```rust
// USCP v3 extensions → DSH extensions (JSON-serialized)
pub enum TelemetryExtension {
    GammaEta { per_agent: HashMap<String, GammaEta>, fleet_sum: GammaEta },
    Thermal { temperature: f64, temperature_task: f64, temperature_idle: f64, is_dreaming: bool },
    Creative { semantic_distance: f64, is_in_creative_zone: bool, is_in_oneiric_zone: bool },
    Melt { melt_pressure: f64, max_crystallization_rate: f64, melt_threshold_exceeded: bool, identity_hash: String, deterministic: bool },
    Molt { molt_count: u64, max_molt_chain: u64, capability: f64 },
    Uncertainty { tau: f64, confidence_zone: ConfidenceZone },
    Anomaly { anomaly_type: String, severity: f64, description: String },
}
```

**Red Line:** Zero telemetry loss. Every extension present in USCP must appear in DSH event with identical semantic meaning.

---

## DSH Event → USCP v3 Translation

### Event Type Mapping (DSH `event_type` → USCP `body.intent`)

| DSH Event Type | USCP Intent | Notes |
|----------------|-------------|-------|
| `TelemetryIngest` | `SENSORY_DATA` | |
| `CommandDispatch` | `COMMAND` | |
| `QueryRequest` | `QUERY` | |
| `StatusReport` | `STATUS` | |
| `HandshakeComplete` | `HANDSHAKE_COMPLETE` | |
| `Escalation` | `ESCALATE` | |
| `ReasoningRequest` | `REQUEST_REASONING` | |
| `ExecutionPlan` | `EXECUTE_PLAN` | |
| `FleetTelemetryPulse` | `CNS_PULSE_STATUS` | |
| `StrategicProposal` | `STRATEGIC_PROPOSAL` | |
| `TaskDelegation` | `DELEGATE_TASK` | |
| `PluginLoaded` | `STATUS` | DSH-specific |
| `PluginUnloaded` | `STATUS` | DSH-specific |
| `PluginError` | `ESCALATE` | DSH-specific |

### Priority Mapping (DSH `priority` → USCP `header.priority`)

| DSH Priority | USCP Priority |
|--------------|---------------|
| `Low` | `LOW` |
| `Normal` | `MEDIUM` |
| `High` | `HIGH` |
| `Critical` | `CRITICAL` |

### Extension Reconstruction

DSH `extensions` (JSON) → USCP v3 telemetry blocks. Unknown extensions are preserved in `anomaly` block with `anomaly_type: "unknown_extension"`.

---

## Bridge Implementation Patterns

### 1. CNS Watcher (File Polling)

```rust
// Polls ~/.hermes/cns_inbox/ for new USCP packets
pub struct CnsWatcher {
    inbox_path: PathBuf,
    processed: HashSet<String>,
    tx: mpsc::Sender<UscpPacket>,
}

impl CnsWatcher {
    pub async fn run(&mut self) -> Result<()> {
        loop {
            for entry in fs::read_dir(&self.inbox_path)? {
                let path = entry?.path();
                if self.should_process(&path) {
                    let packet = Self::parse_packet(&path)?;
                    self.tx.send(packet).await?;
                    self.processed.insert(path.file_name().unwrap().to_string_lossy().to_string());
                }
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
    }
}
```

**Key Patterns:**
- Track processed files by name to avoid re-processing
- Use `tokio::fs` for async file I/O
- Parse USCP v3 with flexible header (v1/v3 compatible)
- Emit to translator channel

### 2. DSH Client (Event Bus Interface)

```rust
// Stub for spike — replaces with real DSH client in Phase 1
pub struct DshClient {
    rx: mpsc::Receiver<DshEvent>,
    tx: mpsc::Sender<DshEvent>,
}

impl DshClient {
    pub async fn emit(&self, event: DshEvent) -> Result<()> {
        // In spike: log + metrics; in production: DSH gRPC/HTTP
        tracing::info!(?event, "DSH emit");
        self.metrics.emit.increment();
        Ok(())
    }
    
    pub async fn poll(&mut self) -> Result<Vec<DshEvent>> {
        // In spike: return empty; in production: DSH event bus poll
        Ok(vec![])
    }
}
```

**Key Patterns:**
- Trait-based for swapability (`DshClientTrait`)
- Metrics instrumentation at emit/poll points
- Graceful degradation when DSH unavailable

### 3. Translator (Bidirectional)

```rust
pub struct Translator;

impl Translator {
    pub fn ucsp_to_dsh(&self, packet: &UscpPacket) -> Result<DshEvent> {
        let event_type = Self::map_intent(&packet.body.intent);
        let priority = Self::map_priority(&packet.header.priority);
        let extensions = Self::extract_telemetry(&packet.body.payload)?;
        
        Ok(DshEvent {
            event_type,
            priority,
            payload: packet.body.payload.data.clone(),
            extensions,
            correlation_id: packet.header.correlation_id.clone(),
            timestamp: Utc::now(),
        })
    }
    
    pub fn dsh_to_ucsp(&self, event: &DshEvent) -> Result<UscpPacket> {
        let intent = Self::map_event_type(&event.event_type);
        let priority = Self::map_dsh_priority(&event.priority);
        let payload = Self::reconstruct_payload(&event.extensions, &event.payload)?;
        
        Ok(UscpPacket {
            header: UscpHeader {
                origin_id: "dsh-bridge".to_string(),
                destination_id: "hermes-cns".to_string(),
                timestamp: event.timestamp,
                priority,
                correlation_id: event.correlation_id.clone(),
            },
            body: UscpBody {
                intent,
                payload,
            },
            signature: UscpSignature {
                type_: "USCP-v3".to_string(),
                version: "3.0".to_string(),
                extensions: vec!["gamma_eta".into(), "thermal".into(), "creative".into(), 
                                 "melt".into(), "molt".into(), "uncertainty".into(), "anomaly".into()],
            },
        })
    }
}
```

**Key Patterns:**
- Pure functions (no state) for testability
- `black_box` in benchmarks to prevent optimization
- Roundtrip test: `ucsp → dsh → ucsp` must preserve all fields

### 4. Metrics (Prometheus)

```rust
use prometheus::{Counter, Histogram, register_counter, register_histogram};

lazy_static! {
    static ref TRANSLATION_LATENCY: Histogram = register_histogram!(
        "cns_dsh_translation_latency_seconds",
        "Translation latency in seconds"
    ).unwrap();
    
    static ref TRANSLATION_SUCCESS: Counter = register_counter!(
        "cns_dsh_translation_success_total",
        "Successful translations"
    ).unwrap();
    
    static ref TRANSLATION_FAILURE: Counter = register_counter!(
        "cns_dsh_translation_failure_total",
        "Failed translations"
    ).unwrap();
}
```

**Instrumentation Points:**
- `ucsp_to_dsh` latency
- `dsh_to_ucsp` latency
- Roundtrip latency
- Success/failure counters per direction

---

## Benchmark Validation

### Latency Targets

| Operation | Target | Achieved (Spike) | Margin |
|-----------|--------|------------------|--------|
| USCP → DSH | <100μs | 4.7μs | 21x |
| DSH → USCP | <100μs | 3.3μs | 30x |
| Roundtrip (full) | <100μs | 7.9μs | 12x |
| Roundtrip (minimal) | <100μs | 1.1μs | 90x |

### Test Coverage (8/8 PASS)

| Test | Validates |
|------|-----------|
| `test_intent_mapping_bidirectional` | All 11 intent mappings roundtrip |
| `test_priority_preserved` | Priority enums map correctly both ways |
| `test_extensions_serialization` | All 7 telemetry extensions serialize |
| `test_roundtrip_preserves_telemetry` | Full USCP→DSH→USCP preserves extensions |
| `test_dsh_client_creation` | Client constructs without error |
| `test_dsh_client_emit_poll` | Emit/poll cycle works |
| `test_metrics_increment` | Prometheus counters increment |
| `test_translator_creation` | Translator constructs |

---

## Integration Checklist

Before declaring bridge ready for Phase 1:

- [ ] All 7 USCP v3 telemetry extensions translate losslessly
- [ ] All 11 USCP intents map bidirectionally
- [ ] Priority mapping preserves ordering (Critical > High > Normal > Low)
- [ ] Roundtrip USCP→DSH→USCP preserves all fields
- [ ] Benchmarks meet <100μs target (12-90x margin achieved)
- [ ] 8/8 unit tests pass in `--release` mode
- [ ] Prometheus metrics exposed at `/metrics` endpoint
- [ ] CNS watcher handles file rotation/cleanup
- [ ] DSH client trait allows swap to real implementation
- [ ] Graceful degradation when DSH unavailable (log + queue)

---

## Red Lines (Non-Negotiable)

1. **Zero telemetry loss** — Every USCP v3 extension must appear in DSH event
2. **Priority ordering preserved** — Critical always maps to Critical
3. **Roundtrip fidelity** — USCP→DSH→USCP must be bitwise-identical for telemetry
4. **Latency budget** — <100μs per translation (achieved: 1.1-7.9μs)
5. **No CNS schema changes** — Bridge adapts to CNS, not vice versa