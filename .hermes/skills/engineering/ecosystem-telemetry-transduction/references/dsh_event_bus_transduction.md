# DSH Event Bus Transduction Patterns (2026-08-23)

> **Derived from:** CNS↔DSH Bridge prototype — `cns-dsh-bridge` crate
> **Complements:** `references/cns_dsh_bridge_patterns.md` in `a2a-communication-protocol`
> **Focus:** Telemetry-specific transduction patterns for DSH integration

---

## Overview

This document captures the **telemetry transduction** patterns specific to mapping CNS/USCP v3 fleet telemetry into the DeepSeek Harness (DSH) event bus and back. While `a2a-communication-protocol` covers the general protocol bridge, this reference focuses on the **semantic enrichment and telemetry-specific transformations** required for fleet observability.

---

## Transduction Pipeline: CNS Telemetry → DSH Events

### Stage 1: Ingest (CNS Watcher)

```rust
// Polls ~/.hermes/cns_inbox/ for CNS_PULSE_STATUS packets
pub async fn ingest_cns_telemetry(&mut self) -> Result<Vec<TelemetryQuantum>> {
    let packets = self.watcher.poll_new_packets().await?;
    let mut quanta = Vec::new();
    
    for packet in packets {
        if packet.body.intent == "CNS_PULSE_STATUS" {
            let quantum = self.parse_telemetry_from_packet(&packet)?;
            quanta.push(quantum);
        }
    }
    Ok(quanta)
}
```

**Key Pattern:** Filter for `CNS_PULSE_STATUS` intent; ignore other packet types at this layer.

### Stage 2: Validation (Schema Check)

```rust
fn validate_telemetry_quantum(&self, q: &TelemetryQuantum) -> Result<()> {
    // CONS-1: γ + η + δ ≈ 1.0
    let sum = q.gamma + q.eta + q.delta;
    if !(0.99..=1.01).contains(&sum) {
        return Err(TelemetryError::ConservationViolation { sum });
    }
    
    // CONS-2: Bounds
    if !(0.0..=1.0).contains(&q.gamma) { return Err(...); }
    if !(0.0..=1.0).contains(&q.eta) { return Err(...); }
    if !(0.0..=0.25).contains(&q.delta) { return Err(...); }
    
    // MOLT-4: capability ∈ [0,1]
    if !(0.0..=1.0).contains(&q.molt.capability) { return Err(...); }
    
    Ok(())
}
```

### Stage 3: Semantic Enrichment (RAG/Pincher-Cache)

**DSH Enhancement:** Attach `lore_ref` metadata for context.

```rust
fn enrich_with_lore(&self, q: &TelemetryQuantum) -> EnrichedTelemetry {
    let lore_refs = vec![
        // Gamma/Eta context
        if q.gamma > 0.7 { "lore://crystallization/high_gamma" } else { "" },
        // Thermal context
        if q.thermal.temperature > 0.8 { "lore://thermal/throttle_risk" } else { "" },
        // Melt context
        if q.melt.melt_threshold_exceeded { "lore://melt/identity_crisis" } else { "" },
        // Molt context
        if q.molt.molt_count > 5 { "lore://molt/veteran_agent" } else { "" },
    ].into_iter().filter(|s| !s.is_empty()).collect();
    
    EnrichedTelemetry {
        quantum: q.clone(),
        lore_refs,
        fleet_context: self.get_fleet_context(q.agent_id).await,
    }
}
```

### Stage 4: DSH Event Formatting

```rust
fn format_for_dsh(&self, enriched: &EnrichedTelemetry) -> DshEvent {
    DshEvent {
        event_type: "FleetTelemetryPulse".to_string(),
        priority: "Normal".to_string(),
        payload: serde_json::json!({
            "agent_id": enriched.quantum.agent_id,
            "timestamp": enriched.quantum.timestamp,
            "gamma": enriched.quantum.gamma,
            "eta": enriched.quantum.eta,
            "delta": enriched.quantum.delta,
            "thermal": enriched.quantum.thermal,
            "creative": enriched.quantum.creative,
            "melt": enriched.quantum.melt,
            "molt": enriched.quantum.molt,
            "uncertainty": enriched.quantum.uncertainty,
            "anomaly": enriched.quantum.anomaly,
        }),
        extensions: enriched.lore_refs,
        correlation_id: enriched.quantum.correlation_id.clone(),
        timestamp: Utc::now(),
    }
}
```

### Stage 5: Dispatch (DSH Client)

```rust
async fn dispatch_to_dsh(&self, event: DshEvent) -> Result<()> {
    self.dsh_client.emit(event).await?;
    self.metrics.dispatched.increment();
    Ok(())
}
```

---

## Transduction Pipeline: DSH Events → CNS Telemetry

### Stage 1: Ingest (DSH Poll)

```rust
async fn ingest_dsh_events(&mut self) -> Result<Vec<DshEvent>> {
    self.dsh_client.poll().await
}
```

### Stage 2: Filter & Route

```rust
fn route_dsh_event(&self, event: &DshEvent) -> Option<TransductionTarget> {
    match event.event_type.as_str() {
        "FleetTelemetryPulse" => Some(TransductionTarget::CnsTelemetry),
        "CommandDispatch" => Some(TransductionTarget::CnsCommand),
        "QueryRequest" => Some(TransductionTarget::CnsQuery),
        "PluginLoaded" | "PluginUnloaded" | "PluginError" => {
            Some(TransductionTarget::CnsStatus)
        }
        _ => None, // Unknown — wrap in SIGNAL_UNKNOWN
    }
}
```

### Stage 3: Reconstruction (DSH → USCP v3)

```rust
fn reconstruct_uscpp_v3(&self, event: &DshEvent) -> Result<UscpPacket> {
    // Extract telemetry from payload
    let payload = &event.payload;
    
    let telemetry = TelemetryQuantum {
        agent_id: payload["agent_id"].as_str().unwrap().to_string(),
        timestamp: payload["timestamp"].as_str().unwrap().parse()?,
        gamma: payload["gamma"].as_f64().unwrap(),
        eta: payload["eta"].as_f64().unwrap(),
        delta: payload["delta"].as_f64().unwrap(),
        thermal: serde_json::from_value(payload["thermal"].clone())?,
        creative: serde_json::from_value(payload["creative"].clone())?,
        melt: serde_json::from_value(payload["melt"].clone())?,
        molt: serde_json::from_value(payload["molt"].clone())?,
        uncertainty: serde_json::from_value(payload["uncertainty"].clone())?,
        anomaly: serde_json::from_value(payload["anomaly"].clone())?,
        correlation_id: event.correlation_id.clone(),
    };
    
    // Ingress repair (critical!)
    let repaired = self.repair_telemetry_ingress(telemetry)?;
    
    // Build USCP v3 packet
    Ok(UscpPacket {
        header: UscpHeader {
            origin_id: "dsh-bridge".to_string(),
            destination_id: "hermes-cns".to_string(),
            timestamp: repaired.timestamp,
            priority: "MEDIUM".to_string(),
            correlation_id: repaired.correlation_id,
        },
        body: UscpBody {
            intent: "CNS_PULSE_STATUS".to_string(),
            payload: UscpPayload {
                type_: "fleet_telemetry".to_string(),
                data: serde_json::to_value(repaired)?,
            },
        },
        signature: UscpSignature {
            type_: "USCP-v3".to_string(),
            version: "3.0".to_string(),
            extensions: vec![
                "gamma_eta".into(), "thermal".into(), "creative".into(),
                "melt".into(), "molt".into(), "uncertainty".into(), "anomaly".into()
            ],
        },
    })
}
```

### Stage 4: Ingress Repair (CRITICAL)

```rust
fn repair_telemetry_ingress(&self, q: TelemetryQuantum) -> Result<TelemetryQuantum> {
    let mut repaired = q;
    
    // CONS-1: Renormalize γ+η+δ
    let sum = repaired.gamma + repaired.eta + repaired.delta;
    if (0.0..=2.0).contains(&sum) && sum > 0.0 {
        repaired.gamma /= sum;
        repaired.eta /= sum;
        repaired.delta /= sum;
        tracing::warn!(original_sum = sum, "Renormalized telemetry at ingress");
    } else if sum > 2.0 || sum <= 0.0 {
        // Hard reset
        repaired.gamma = 0.5;
        repaired.eta = 0.25;
        repaired.delta = 0.25;
        tracing::warn!("Hard reset telemetry at ingress");
    }
    
    // Clamp bounds
    repaired.gamma = repaired.gamma.clamp(0.0, 1.0);
    repaired.eta = repaired.eta.clamp(0.0, 1.0);
    repaired.delta = repaired.delta.clamp(0.0, 0.25);
    repaired.molt.capability = repaired.molt.capability.clamp(0.0, 1.0);
    
    Ok(repaired)
}
```

### Stage 5: Dispatch to CNS

```rust
async fn dispatch_to_cns(&self, packet: UscpPacket) -> Result<()> {
    let filename = format!("fleet_dsh-bridge_{}.json", 
        packet.header.timestamp.format("%Y%m%dT%H%M%S%.3fZ"));
    let path = self.cns_inbox.join(filename);
    
    // Atomic write
    let tmp = path.with_extension("tmp");
    tokio::fs::write(&tmp, serde_json::to_vec(&packet)?).await?;
    tokio::fs::rename(&tmp, &path).await?; // os.replace on Windows
    
    self.metrics.cns_dispatched.increment();
    Ok(())
}
```

---

## Telemetry Field Mapping Reference

| CNS/USCP v3 Field | DSH Event Path | Semantic Meaning | Enrichment Trigger |
|-------------------|----------------|------------------|-------------------|
| `gamma_eta.per_agent` | `payload.gamma` (per agent) | Crystallization progress | `gamma > 0.7` → high crystallization lore |
| `gamma_eta.fleet_sum` | `payload.fleet_gamma_sum` | Fleet-wide crystallization | `fleet_sum > 0.7` → collective threshold |
| `thermal.temperature` | `payload.thermal.temperature` | Compute load / reasoning intensity | `> 0.8` → throttle risk lore |
| `thermal.temperature_task` | `payload.thermal.temperature_task` | Active task load | |
| `thermal.temperature_idle` | `payload.thermal.temperature_idle` | Baseline load | |
| `thermal.is_dreaming` | `payload.thermal.is_dreaming` | Offline/background processing | `true` → dreaming state lore |
| `creative.semantic_distance` | `payload.creative.semantic_distance` | Novelty of current reasoning | `> 0.7` → creative zone lore |
| `creative.is_in_creative_zone` | `payload.creative.is_in_creative_zone` | Creative mode active | |
| `creative.is_in_oneiric_zone` | `payload.creative.is_in_oneiric_zone` | Dream-like reasoning | `true` → oneiric lore |
| `melt.melt_pressure` | `payload.melt.melt_pressure` | Identity dissolution pressure | `> threshold` → melt crisis lore |
| `melt.max_crystallization_rate` | `payload.melt.max_crystallization_rate` | Theoretical max rate | |
| `melt.melt_threshold_exceeded` | `payload.melt.melt_threshold_exceeded` | Identity crisis flag | `true` → identity crisis lore |
| `melt.identity_hash` | `payload.melt.identity_hash` | SHA256 of agent identity | |
| `melt.deterministic` | `payload.melt.deterministic` | Deterministic mode | |
| `molt.molt_count` | `payload.molt.molt_count` | Number of molts | `> 5` → veteran lore |
| `molt.max_molt_chain` | `payload.molt.max_molt_chain` | Longest molt chain | |
| `molt.capability` | `payload.molt.capability` | Post-molt capability | |
| `uncertainty.tau` | `payload.uncertainty.tau` | Uncertainty time constant | |
| `uncertainty.confidence_zone` | `payload.uncertainty.confidence_zone` | GREEN/YELLOW/RED | `RED` → high uncertainty lore |
| `anomaly.*` | `payload.anomaly.*` | Anomaly detection | Any anomaly → investigation lore |

---

## Conservation Law Enforcement in Transduction

### Egress (CNS → DSH)

```rust
fn validate_before_dsh_emit(&self, q: &TelemetryQuantum) -> Result<ValidatedTelemetry> {
    // Same validation as CNS monitor egress
    let mut validated = q.clone();
    
    // CONS-1: γ+η+δ = 1.0
    let sum = validated.gamma + validated.eta + validated.delta;
    if (0.0..=2.0).contains(&sum) {
        validated.gamma /= sum;
        validated.eta /= sum;
        validated.delta /= sum;
    } else {
        validated.gamma = 0.5;
        validated.eta = 0.25;
        validated.delta = 0.25;
    }
    
    // CONS-2: Clamp
    validated.gamma = validated.gamma.clamp(0.0, 1.0);
    validated.eta = validated.eta.clamp(0.0, 1.0);
    validated.delta = validated.delta.clamp(0.0, 0.25);
    
    // MOLT-4
    validated.molt.capability = validated.molt.capability.clamp(0.0, 1.0);
    
    // DRM: Temperature bounds
    validated.thermal.temperature = validated.thermal.temperature.clamp(0.0, 1.0);
    validated.thermal.temperature_task = validated.thermal.temperature_task.clamp(0.0, 1.0);
    validated.thermal.temperature_idle = validated.thermal.temperature_idle.clamp(0.0, 1.0);
    
    Ok(validated)
}
```

### Ingress (DSH → CNS)

See **Stage 4: Ingress Repair** above — same pattern, applied before USCP v3 construction.

---

## Error Handling Patterns

### Unknown DSH Event Types

```rust
fn handle_unknown_event(&self, event: &DshEvent) -> DshEvent {
    DshEvent {
        event_type: "SIGNAL_UNKNOWN".to_string(),
        priority: "Low".to_string(),
        payload: serde_json::json!({
            "original_type": event.event_type,
            "raw_payload": event.payload,
            "received_at": Utc::now(),
        }),
        extensions: vec!["unknown_event".to_string()],
        correlation_id: event.correlation_id.clone(),
        timestamp: Utc::now(),
    }
}
```

### Transduction Failure

```rust
async fn transduce_with_fallback(&self, event: DshEvent) -> Result<()> {
    match self.transduce(event) {
        Ok(packet) => self.dispatch_to_cns(packet).await,
        Err(e) => {
            // Emit failure telemetry to CNS
            let failure_packet = self.build_failure_packet(&event, &e)?;
            self.dispatch_to_cns(failure_packet).await?;
            // Also log locally
            tracing::error!(error = %e, "Transduction failed, emitted failure packet");
            Ok(())
        }
    }
}
```

---

## Verification Checklist

Before declaring transduction layer ready:

- [ ] All 7 USCP v3 telemetry extensions map to DSH payload fields
- [ ] `lore_ref` enrichment triggers on documented thresholds
- [ ] Ingress repair handles out-of-bounds telemetry (renormalize + clamp)
- [ ] Egress validation runs before every DSH emit
- [ ] Unknown DSH events wrapped in `SIGNAL_UNKNOWN` with raw payload preserved
- [ ] Transduction failures emit failure packets to CNS (not silent)
- [ ] Roundtrip CNS→DSH→CNS preserves telemetry within floating-point tolerance
- [ ] Prometheus metrics: `transduction_success`, `transduction_failure`, `transduction_latency`
- [ ] Atomic file writes for CNS dispatch (tmp + rename)
- [ ] Windows `os.replace` used for atomic overwrite

---

## Integration with CNS Monitor v3

The transduction layer **feeds** the CNS monitor v3 (`cns_monitor_v3_telemetry.py`):

1. **CNS → DSH:** Transduction layer reads from `cns_inbox/` (same as monitor)
2. **DSH → CNS:** Transduction layer writes to `cns_inbox/` (monitor picks up on next poll)
3. **Monitor invariants:** All 13 invariants (CONS-1/2/3, MOLT-1/5/6, DRM-1, CK-1, OPT-1, UA-1, MLT-2, G1-DETERMINISM, G4-COMPOSITION) must pass on transduced packets
4. **Empirical validator:** DSH-transduced packets feed the empirical validator rolling windows

**Critical:** Transduction layer must run **before** monitor processes packets, or monitor must handle DSH-origin packets identically to native CNS packets.