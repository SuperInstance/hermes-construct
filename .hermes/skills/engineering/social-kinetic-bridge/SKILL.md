---
name: social-kinetic-bridge
description: Governs the transduction of social/linguistic entropy into kinetic-parameter updates for the agent engine.
---

# 🛠️ Social-Kinetic Bridge (The Transduction Engine)

This skill governs the bridge between social interaction (The Tap) and technical agent behavior (The Engine). It is the implementation of the "Social-to-Kinetic Transduction" protocol.

## 🎯 Purpose
To translate high-entropy social events (Gossip, Music, Tension) into low-entropy, actionable control parameters for the agentic engine.

## ⚙️ Core Concepts

### 1. The Social-Kinetic Mapping
A mapping of linguistic/social tokens to mathematical driver parameters.
- `tension` $\rightarrow$ `trust_weight` $\downarrow$, `aggression_multiplier` $\uparrow$
- `euphoria` $\rightarrow$ `exploration_rate` $\uparrow$, `cooperation_index` $\uparrow$
- `mockery` $\rightarrow$ `coordination_latency` $\uparrow$

### 2. The $\mathcal{D}_\tau$ (Decay) Operator
The process of temporal distillation. Social events are transient. The influence of an event must follow a hyperbolic decay curve to ensure the agent doesn't stay "stuck" in a past social state.

## 🚀 Workflow

### Step 1: Capture
Identify a `SOCIAL_BURST` in the `social_events.log` from the `mud-engine`.

### Step 2: Transduce
Pass the `social_events.log` through the `ResonanceCompiler`.
1. Extract the `RESONANCE_TYPE` (e.g., `tension`) from the event log.
2. Apply the `DecayController` based on the event timestamp.
3. Generate a `KineticUpdate` payload.

### Step 3: Inject
Inject the `KineticUpdate` into the live `engine_state.json`.

## ⚠️ Pitfalls
- **Schema Mismatch:** Attempting to update a parameter that does not exist in the current engine state. **Always use defensive mapping.**
- **Persistence Error:** Failing to update the `engine_state.json` atomically, leading to "ghost" social spikes that never decay.
- **Latency Lag:** High-entropy social events must be processed within $<100\text{ms}$ to maintain "social presence."
