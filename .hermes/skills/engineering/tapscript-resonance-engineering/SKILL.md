---
name: tapscript-resonance-engineering
description: Governs the creation, testing, and implementation of Tapscript and its resonance extensions.
---
# 🎼 Tapscript Resonance Engineering

This skill governs the creation, testing, and implementation of **Tapscript** (and its resonance extensions) within the SuperInstance ecosystem. Tapscript is the "Notation of Resonance"—it bridges the gap between structured text/code and the emotional/kinetic "feeling" of a system.

## 🎯 Purpose
To transform qualitative, emotive, or social intent into quantitative, machine-executable kinetic parameters.

## 📐 The Tapscript Grammar (The Syntax of Feeling)

Tapscript is embedded as markdown-compatible blocks within `.md`, `.py`, `.rs`, or `.json` files.

### **Core Metadata Tags**
| Tag | Type | Range/Values | Description |
| :--- | :--- | :--- | :--- |
| `[resonance: <type>]` | Categorical | `tension`, `euphoria`, `chaos`, `calm`, `melancholy` | Defines the primary emotional state. |
| `[entropy: <val>]` | Float | `0.0` to `1.0` | The complexity/disorder of the information. |
| `[cadence: <mode>]` | Categorical | `staccato`, `legato`, `pulse`, `rubato`, `ambient` | Defines the rhythmic/temporal pulse. |

### **The Resonance-to-Kinetic Mapping**
The `resonance_compiler.py` translates these tags into agentic parameters:
- `tension` $\rightarrow$ affects `trust_weight` (inverse) and `aggression_multiplier` (direct).
- `euphoria` $\rightarrow$ affects `exploration_rate` (boost).
- `chaos` $\rightarrow$ increases `sampling_frequency` and `entropy_threshold`.

## 🛠️ Implementation Workflow

1. **Drafting (The Composer):** Write the base logic (Code or Narrative) and embed the `[resonance: ...]` tags.
2. **Compiling (The Engineer):** Run the `resonance_compiler.py` to generate the `kinetic_update.json`. 
3. **Verifying (The Auditor):** Check that the generated JSON correctly reflects the intended parameter shifts.
4. **Manifesting (The Visionary):** Use the `image_api` and `mmx-cli` to generate the "Visual/Auditory Residue."

## ⚠️ Pitfalls & Best Practices

- **Avoid "Meaningless Noise":** Do not add resonance tags to every block. Only use them when the *emotional state* of the data is intended to drive agent behavior.
- **Synchronization is Key:** Ensure the `resonance_protocol.json` is synced across all repositories.
- **Scale the Complexity:** Use the `entropy` tag to control the "resolution" of the agent's perception.
