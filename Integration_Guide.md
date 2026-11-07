# Integration Guide: Cell-to-Claw Framework

## Overview
This document provides the technical specifications and integration procedures for connecting biological monitoring systems ("Cells") to the automated reasoning and execution engine ("Claw"). This integration enables high-fidelity, real-time response to cellular-level telemetry.

---

## 1. The Cell-to-Claw Lifecycle

The lifecycle defines the transition from raw biological telemetry to autonomous agent intervention.

| Phase | Stage | Description |
| :--- | :--- | :--- |
| **1. Detection** | `CELL_SIGNAL` | A biological sensor or monitoring node detects a threshold breach or specific pattern. |
| **2. Packaging** | `SIGNAL_ENCAPSULATION` | The raw signal is wrapped into a standardized JSON payload including metadata (sensor ID, timestamp, confidence). |
| **3. Transmission** | `INGESTION` | The encapsulated signal is pushed via WebSocket to the Claw Ingestion Gateway. |
| **4. Triggering** | `AGENT_SPAWN` | The Claw Engine parses the signal, identifies the relevant intent, and spawns a specialized sub-agent. |
| **5. Execution** | `ACTION_LOOP` | The agent executes tool calls to mitigate the signal or investigate, providing real-time telemetry updates. |
| **6. Resolution** | `STATE_STABILIZATION` | The agent validates the environment has returned to nominal parameters and terminates the session. |

---

## 2. Communication Protocol

All real-time communication between Cells and the Claw engine must occur via **Secure WebSockets (WSS)**.

### Message Shapes

#### A. Inbound Signal (Cell $\rightarrow$ Claw)
Sent when a biological event requires attention.

```json
{
  "type": "SIGNAL_EVENT",
  "header": {
    "source_id": "uuid-sensor-001",
    "protocol_version": "2.1.0",
    "timestamp": "2026-08-17T10:00:00Z"
  },
  "payload": {
    "metric": "ATP_FLUX_RATE",
    "value": 0.42,
    "unit": "mmol/s",
    "status": "CRITICAL",
    "context": {
      "location": "sector-7-nucleus",
      "deviation": "+45%"
    }
  }
}
```

#### B. Agent Feedback (Claw $\rightarrow$ Cell/Monitor)
Sent by the agent to log ongoing investigative steps or command acknowledgments.

```json
{
  "type": "AGENT_UPDATE",
  "header": {
    "agent_id": "agent-alpha-9",
    "correlation_id": "uuid-signal-xyz"
  },
  "payload": {
    "status": "INVESTIGATING",
    "action_taken": "RECALIBRATING_SENSORS",
    "confidence_score": 0.89
  }
}
```

---

## 3. Data Schema

The Claw Engine expects strict adherence to the following schema for all incoming telemetry.

### Input Schema (Signal)
| Field | Type | Required | Description |
| :--- | :--- | :--- | :--- |
| `source_id` | `string` (UUID) | Yes | Unique identifier for the biological sensor. |
| `metric` | `string` | Yes | The specific biological variable being reported. |
| `value` | `float` | Yes | Numerical reading of the metric. |
| `status` | `enum` | Yes | `NOMINAL`, `WARNING`, `CRITICAL`, `FAILURE`. |
| `context` | `object` | No | Arbitrary key-value pairs for environmental metadata. |

### Output Schema (Engine Response)
| Field | Type | Description |
| :--- | :--- | :--- |
| `action_status` | `string` | The result of the agent's intervention (`SUCCESS`, `PENDING`, `FAILED`). |
| `mitigation_log` | `array[string]` | Sequence of actions taken by the agent. |
| `residual_risk` | `float` | Calculated probability of signal recurrence (0.0 - 1.0). |

---

## 4. Testing & Validation

To ensure integration stability, implementations must pass two distinct testing tiers.

### 4.1 Unit Testing (Protocol Level)
Validate that the Cell-side library correctly serializes data and handles network jitter.
* **Validation Criteria:**
    * Schema compliance against the JSON specification.
    * Correct handling of `SIGPIPE` and reconnection logic.
    * Proper timestamp synchronization (NTP alignment required).

### 4.2 E2E Integration Testing (Lifecycle Level)
Simulate a full "Cell-to-Claw" loop in a sandboxed environment.
* **Test Case: `TC-001_CRITICAL_RECOVERY`**
    1. **Inject:** Send a `CRITICAL` metric payload via WebSocket.
    2. **Observe:** Verify an `AGENT_SPAWN` event is logged in the Claw orchestrator.
    3. **Validate:** Verify the agent issues at least one `ACTION` command within 500ms.
    4. **End State:** Ensure the `STATE_STABILIZATION` signal is emitted following successful mitigation.
* **Success Metric:** Convergence of the biological metric to `NOMINAL` within the defined TTL (Time-to-Live).
