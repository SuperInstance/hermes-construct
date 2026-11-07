# CNS↔DSH Bridge Specification

## Overview

Lossless bidirectional translation between CNS USCP v3 packets and DeepSeek Harness (DSH) events.

## Architecture

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

## Core Types

### USCP v3 Packet (`types.rs`)

```rust
pub struct USCPv3Packet {
    pub header: USCPHeader,
    pub body: USCPBody,
    pub signature: Option<USCPSignature>,
}

pub struct USCPHeader {
    pub version: String,           // "3.0"
    pub packet_id: String,         // UUID
    pub source_id: String,         // Agent ID
    pub destination_id: String,    // Agent ID or "broadcast"
    pub timestamp: String,         // RFC3339
    pub intent: USCPIntent,        // Enum (see below)
    pub priority: USCPPriority,    // Low/Normal/High/Critical
    pub correlation_id: Option<String>,
    pub ttl: u32,                  // Default 64
}

pub struct USCPBody {
    pub payload: serde_json::Value,
    pub extensions: Option<USCPExtensions>,
}
```

### USCPIntent Enum (23 variants)

```rust
pub enum USCPIntent {
    FleetPulse,
    RequestReasoning,
    SensoryData,
    ExecutePlan,
    DelegateTask,
    ConsensusPropose,
    ConsensusVote,
    ConsensusResult,
    AgentRegister,
    AgentUnregister,
    EquipmentEquip,
    EquipmentUnequip,
    SeedTrain,
    SeedDeploy,
    CreativePiece,
    AnomalyReport,
    ThermalAlert,
    MeltEvent,
    MoltEvent,
    UncertaintySignal,
    GammaEtaUpdate,
    Custom(String),
}
```

### USCPExtensions (7 Telemetry Extensions)

```rust
pub struct USCPExtensions {
    pub gamma_eta: Option<GammaEta>,      // γ (coherence), η (efficiency)
    pub thermal: Option<Thermal>,         // T (temperature), delta, p_melt
    pub creative: Option<Creative>,       // delta, zone (Crystallized/Creative/Melt)
    pub melt: Option<Melt>,               // depth, trigger
    pub molt: Option<Molt>,               // from_state, to_state, catalyst
    pub uncertainty: Option<Uncertainty>, // value, source
    pub anomaly: Option<Anomaly>,         // score, description
}
```

### DSH Event (`types.rs`)

```rust
pub struct DSHEvent {
    pub event_id: String,
    pub source_plugin: String,
    pub target_plugin: Option<String>,
    pub event_type: String,           // e.g., "fleet.telemetry.pulse"
    pub payload: serde_json::Value,
    pub timestamp: String,            // RFC3339
    pub metadata: DSHMetadata,
}

pub struct DSHMetadata {
    pub correlation_id: Option<String>,
    pub priority: DSHPriority,        // Low/Normal/High/Critical
    pub uscp_extensions: Option<USCPExtensions>,
    pub tags: Vec<String>,            // Required: domain + action
    pub ttl: u32,                     // Required: default 64
}
```

## Translation Mapping

### CNS → DSH (uscp_to_dsh)

| USCPIntent | DSH event_type |
|------------|----------------|
| FleetPulse | fleet.telemetry.pulse |
| RequestReasoning | fleet.command.reason |
| SensoryData | fleet.sensory.data |
| ExecutePlan | fleet.command.execute |
| DelegateTask | fleet.command.delegate |
| ConsensusPropose | fleet.command.consensus.propose |
| ConsensusVote | fleet.command.consensus.vote |
| ConsensusResult | fleet.command.consensus.result |
| AgentRegister | fleet.lifecycle.register |
| AgentUnregister | fleet.lifecycle.unregister |
| EquipmentEquip | fleet.equipment.equip |
| EquipmentUnequip | fleet.equipment.unequip |
| SeedTrain | fleet.seed.train |
| SeedDeploy | fleet.seed.deploy |
| CreativePiece | fleet.creative.piece |
| AnomalyReport | fleet.anomaly.report |
| ThermalAlert | fleet.thermal.alert |
| MeltEvent | fleet.melt.event |
| MoltEvent | fleet.molt.event |
| UncertaintySignal | fleet.uncertainty.signal |
| GammaEtaUpdate | fleet.gamma_eta.update |
| Custom(X) | fleet.custom.X |

**Priority mapping**: Direct 1:1 (Critical→Critical, High→High, Normal→Normal, Low→Low)

**Extensions**: Copied directly from `USCPBody.extensions` → `DSHMetadata.uscp_extensions`

**Correlation ID**: `USCPHeader.correlation_id` → `DSHMetadata.correlation_id`

**TTL**: `USCPHeader.ttl` → `DSHMetadata.ttl`

### DSH → CNS (dsh_to_usc)

Reverse mapping via `reverse_intent_map` HashMap.

**Fallback**: Unknown event_type → `USCPIntent::Custom(event_type)`

**Priority mapping**: Direct 1:1

**Extensions**: `DSHMetadata.uscp_extensions` → `USCPBody.extensions`

**Correlation ID**: Preserved

**TTL**: Reset to 64 (new packet)

## Bridge Components

### CNSDSHTranslator (`translator.rs`)

```rust
pub struct CNSDSHTranslator {
    agent_id: String,
    intent_map: HashMap<USCPIntent, String>,
    reverse_intent_map: HashMap<String, USCPIntent>,
}

impl CNSDSHTranslator {
    pub fn new(agent_id: String) -> Self
    pub fn uscp_to_dsh(&self, packet: &USCPv3Packet) -> Result<DSHEvent>
    pub fn dsh_to_usc(&self, event: &DSHEvent) -> Result<USCPv3Packet>
    pub fn extract_extensions(&self, packet: &USCPv3Packet) -> USCPExtensions
    pub fn inject_extensions(&self, packet: &mut USCPv3Packet, extensions: USCPExtensions)
}
```

### CNSWatcher (`cns_watcher.rs`)

Polls `~/.hermes/cns_inbox/` for new `.json` files (USCP packets), processes via translator, emits to DSH.

```rust
pub struct CNSWatcher {
    inbox_path: String,
    outbox_path: String,
    translator: Arc<CNSDSHTranslator>,
    dsh_client: Arc<DSHClient>,
    processed: HashSet<String>,  // packet_id deduplication
    poll_interval_ms: u64,
}

impl CNSWatcher {
    pub async fn run(&mut self) -> Result<()>
    async fn process_packet(&mut self, path: &Path) -> Result<()>
}
```

### DSHClient (`dsh_client.rs`)

HTTP client for DSH event bus.

```rust
pub struct DSHClient {
    base_url: String,
    client: reqwest::Client,
    plugin_id: String,
}

impl DSHClient {
    pub async fn emit(&self, event: DSHEvent) -> Result<()>
    pub async fn poll(&self) -> Result<Vec<DSHEvent>>
    pub async fn register(&self, manifest: PluginManifest) -> Result<()>
}
```

### BridgeMetrics (`metrics.rs`)

Prometheus metrics for observability.

```rust
pub struct BridgeMetrics {
    pub translation_latency: HistogramVec,  // uscp_to_dsh, dsh_to_usc
    pub packets_total: CounterVec,          // success, error by direction
    pub active_connections: GaugeVec,       // cns, dsh
    pub extension_preservation: CounterVec, // per-extension roundtrip success
}
```

## Performance Targets

| Operation | Target | Phase 0 (dev) | Phase 1 Week 1 (release) |
|-----------|--------|---------------|--------------------------|
| USCP→DSH (full) | <100 μs | 565 ns (177x) | 1.33 μs (75x) |
| DSH→USCP (full) | <100 μs | 560 ns (179x) | 1.40 μs (71x) |
| Roundtrip (full) | <100 μs | 1.33 μs (75x) | 2.78 μs (36x) |
| Roundtrip (minimal) | <100 μs | 579 ns (173x) | 1.37 μs (73x) |

## Validation Requirements

### Unit Tests (4)
1. `test_intent_mapping_bidirectional` — All 23 intents map correctly
2. `test_priority_preserved` — All 4 priority levels roundtrip
3. `test_roundtrip_all_extensions` — Complex packet with all 7 extensions
4. `test_extensions_serialization` — serde_json roundtrip

### Integration Tests (3)
1. `test_cns_to_dsh_to_plugin_to_dsh_to_cns_flow` — Fleet pulse with all extensions
2. `test_plugin_registry_discovery_flow` — AgentRegister roundtrip
3. `test_fleet_conductor_workflow_flow` — ExecutePlan roundtrip

### Red Lines (6)
1. Claw actor model preserved
2. CNS v3 telemetry intact (7 extensions)
3. Schema contracts frozen
4. Univer integration path viable
5. GPU compute autonomy (autoclaw)
6. Fleet identity preserved

## Configuration

```toml
# BridgeConfig (types.rs)
pub struct BridgeConfig {
    pub agent_id: String,                    // "hermes-cns"
    pub cns_inbox: String,                   // "~/.hermes/cns_inbox"
    pub cns_outbox: String,                  // "~/.hermes/cns_outbox"
    pub dsh_endpoint: String,                // "http://localhost:8080"
    pub poll_interval_ms: u64,               // 100
    pub translation_timeout_ms: u64,         // 1000
}
```

## Error Handling

- **Translation errors**: Logged, packet sent to dead-letter (CNS outbox with error intent)
- **DSH connection errors**: Exponential backoff, max 3 retries
- **CNS file errors**: Skip corrupt packets, continue processing
- **Extension loss**: Counter incremented, alert if >0

## Deployment

```bash
# Build
cargo build --release --manifest-path cns-dsh-bridge/Cargo.toml

# Run bridge
./target/release/cns-dsh-bridge

# Run tests
cargo test --manifest-path cns-dsh-bridge/Cargo.toml

# Run benchmarks
cargo bench --manifest-path cns-dsh-bridge/Cargo.toml
```

## References

- `templates/cns-dsh-bridge/types.rs` — Complete type definitions
- `templates/cns-dsh-bridge/translator.rs` — Translator skeleton
- `references/integration-test-pattern.md` — Integration test template
- `references/phase1-week1-benchmarks.md` — Benchmark results
- `references/dsh-plugin-migration-checklist.md` — Plugin migration guide