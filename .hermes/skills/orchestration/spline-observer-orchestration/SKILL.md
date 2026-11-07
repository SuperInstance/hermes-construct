---
name: spline-observer-orchestration
description: Governs the lifecycle of non-agentic "Observer" entities that track high-fidelity sensor telemetry to build continuous mathematical splines for machine learning.
---

# Spline Observer & Continuous Learning Orchestration

## Overview
Governs the lifecycle of non-agentic "Observer" entities that track high-fidelity sensor telemetry to build continuous mathematical splines for machine learning. This skill manages the transition from raw sensor data (TZ Pro) to high-level, agentic "Spline Traces" used for model training and VaaS (Virtual Agent as a Service) synergy.

## Core Concepts

### 1. The Spline Observer (Non-Agentic)
A specialized, minimal-latency observer that does not perform reasoning, but performs **Feature Extraction**. It transforms raw timestamped data (visual + telemetry) into structured, high-entropy observation traces.

### 2. Kinetic Continuity (The Spline)
Rather than discrete agentic snapshots, the system builds a continuous mathematical "spline" of the environment. This minimizes the "uncertainty gap" between agentic actions.

### 3. VaaS Synergy (Virtual Agent as a Service)
The protocol for connecting high-tier cognitive agents (e.g., Claude, Gemini) to the low-tier, high-speed sensor substrate. The Observer acts as the "Translator" (Synthesizer) between raw telemetry and agentic context.

## Operational Workflow

### Phase 1: Ingestion (The Watchdog)
- **Trigger**: New file detection in `captures/v3/` or NMEA buffer.
- **Input**: `.png` (Visual) + `.json` (Telemetry/Metadata).
- **Task**: Monitor directory/socket with minimal resource footprint.

### Phase 2: Feature Extraction (The Observer)
- **Visual**: Palette detection, bottom-type classification, fish-return density, thermocline identification.
- **Telemetry**: SOG, COG, Depth, Latitude/Longitude.
- **Output**: A "Spline Trace" JSONL record containing engineered features (e.g., `signal_to_noise`, `entropy_index`, `momentum_vector`).

### Phase 3: Continuous Learning (The Feedback Loop)
- **Anomaly Detection**: Identify high-entropy/high-discrepancy events (e.g., $|sensor\_depth - chart\_depth| > threshold$).
- **Trace Archiving**: Move high-value traces to a specialized training corpus.
- **Model Update**: Use traces to fine-tune Claw/Boat-Agent model weights.

## Pitfalls & Best Practices

- **Avoid Reasoning in the Observer**: The Observer MUST NOT run LLM inferences. It is a deterministic feature extractor. Reasoning belongs to the VaaS/Claw layer.
- **Minimize Latency**: The Observer must operate at a cadence higher than the agentic loop to ensure the spline is continuous, not jagged.
- **Provenance is Mandatory**: Every trace must include the `origin_id` and `timestamp` to maintain the integrity of the kinetic residue.
- **Handle Divergence Early**: If the sensor stream is interrupted, the Spline must enter a "Safe/Uncertain" state rather than extrapolating false confidence.

## Implementation Reference
- **Repository**: `/c/Users/casey/polln/Flow-state/`
- **Primary Directory**: `traces/tzpro_analysis/`
- **Contract**: `manifestos/VaaS_SYNERGY_PROTOCOL.md`
