# Tapscript Resonance Specification (v1.0)

## Overview
The `tapscript-resonance` extension introduces a formal method for embedding emotive and rhythmic metadata within `tapscript` blocks. This metadata is designed to be consumed by the **Resonance Engine**, enabling dynamic adjustment of generative outputs, UI feedback, and environmental synthesis based on the "vibe" or "intent" of the script.

## 1. Formal Grammar

Resonance metadata is encapsulated within specialized attribute tags placed inside or immediately adjacent to a `tapscript` block.

### 1.1 Tag Structure
All resonance attributes follow the bracketed key-value format:
`[<attribute>: <value>]`

### 1.2 Attribute Definitions

#### A. Resonance Type (`resonance`)
Defines the emotional core or primary qualitative state of the block.
- **Syntax:** `[resonance: <type>]`
- **Allowed Values:**
  - `tension`: High-frequency, anxious, or anticipatory states.
  - `euphoria`: High-energy, positive, or climactic states.
  - `chaos`: Unpredictable, noisy, or disordered states.
  - `calm`: Low-frequency, stable, or meditative states.
  - `melancholy`: Slow, heavy, or somber states.

#### B. Entropy (`entropy`)
Quantifies the structural complexity and disorder within the block.
- **Syntax:** `[entropy: <float>]`
- **Range:** `0.0` (Perfect Order) to `1.0` (Maximum Chaos/Noise).
- **Usage:**
  - `0.0 - 0.2`: Highly structured, repetitive, predictable.
  - `0.3 - 0.6`: Balanced, organic complexity.
  - `0.7 - 1.0`: High volatility, glitch-heavy, non-linear.

#### C. Cadence (`cadence`)
Defines the rhythmic pulse or temporal density.
- **Syntax:** `[cadence: <tempo>]`
- **Allowed Values:**
  - `staccato`: Rapid, disconnected, sharp pulses.
  - `legato`: Smooth, flowing, continuous transitions.
  - `pulse`: Steady, rhythmic, heartbeat-like.
  - `rubato`: Expressive, variable, non-metronomic timing.
  - `ambient`: Drifting, non-rhythmic temporal field.

---

## 2. Implementation Examples

### Example 1: High-Tension Glitch
A block designed for a fragmented, high-anxiety sequence.

```tapscript
[resonance: tension]
[entropy: 0.85]
[cadence: staccato]

// Fragmented execution logic
trigger(glitch_01) -> delay(var(entropy))
loop(3) { execute(stutter) }
```

### Example 2: Ambient Calm
A block designed for a soothing, underlying background layer.

```tapscript
[resonance: calm]
[entropy: 0.1]
[cadence: ambient]

// Slow, sweeping transitions
fade_in(soft_drone, 10s)
oscillate(sine_wave, frequency=low)
```

---

## 3. Resonance Engine Integration

The Resonance Engine parses these tags during the pre-compilation phase of the `tapscript` lifecycle.

| Tag | Engine Mapping | Effect Example |
| :--- | :--- | :--- |
| `resonance` | Emotional Filter | Shifts color palettes or reverb characteristics. |
| `entropy` | Stochastic Variance | Increases the `random()` seed variance or jitter. |
| `cadence` | Temporal Quantization | Adjusts the global clock or rhythmic snap. |

## 4. Error Handling
- **Invalid Attribute:** If an unknown attribute is found within `[]`, it is ignored by the Resonance Engine.
- **Out of Range:** If `entropy` is outside `[0.0, 1.0]`, it is clamped to the nearest boundary.
- **Invalid Type:** If `resonance` contains an unrecognized type, the engine defaults to `calm`.
