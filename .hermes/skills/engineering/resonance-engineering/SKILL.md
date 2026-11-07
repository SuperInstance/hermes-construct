---
name: resonance-engineering
description: Governs the design, implementation, and distillation of intelligence using "Resonance" as the primary metric for agentic efficacy.
---

# RESONANCE-BASED ENGINEERING

Governs the design, implementation, and distillation of intelligence using "Resonance" as the primary metric for agentic efficacy. This skill is used when designing systems that treat intelligence as an active process (the loop) rather than a static artifact (the model).

## CORE PRINCIPLES

- **Intelligence as Pruning:** Expertise is the reduction of degrees of freedom. High-fidelity intelligence is the ability to ignore noise (the co-exact component) to focus on the signal (the exact component) and the topological truth (the harmonic residue).
- **The Hodge Decomposition Duality:** All agentic interactions can be decomposed into three orthogonal components:
  - **Signal ($d\alpha$):** The predictable, longitudinal, actionable trajectory.
  - **Noise ($\delta\beta$):** The high-frequency, transverse, stochastic fluctuations.
  - **Residue ($\gamma$):** The underlying structural truth or "negative space" that remains when the flux is removed.
- **Resonance as Metric:** Success is measured by the ability of an agent to project its internal policy onto the environmental manifold with minimal "control effort" (motion/energy).

## TRIPARTITE CONSENSUS ENGINE (2026-08-18)

The Tripartite Consensus pattern implements the Hodge decomposition as a three-agent deliberation system:

### Architecture
- **Pathos (Signal/$d\alpha$)**: Emotional/Intuitive resonance — "Does this feel right?"
- **Logos (Noise/$\delta\beta$)**: Logical/Analytical resonance — "Does this compute?"
- **Ethos (Residue/$\gamma$)**: Ethical/Structural resonance — "Does this hold?"

### Implementation Pattern (Rust)
```rust
pub struct TripartiteConsensus {
    pub pathos_score: f64,   // [0.0, 1.0]
    pub logos_score: f64,    // [0.0, 1.0]
    pub ethos_score: f64,    // [0.0, 1.0]
    pub resonance_score: f64, // Clamped weighted sum
}

impl TripartiteConsensus {
    pub fn deliberate(&self, input: &str) -> ResonanceScore {
        // Each perspective scores independently
        // ResonanceScore = clamp(0.4*pathos + 0.3*logos + 0.3*ethos, 0.0, 1.0)
    }
}
```

### Key Invariants
- **Clamping**: `ResonanceScore` MUST be clamped to `[0.0, 1.0]` — use `f64::clamp()`.
- **Float precision**: Use epsilon comparisons in tests `(score - expected).abs() < 1e-10`.
- **JsonSchema**: Derive with `schemars`; use `#[schemars(with = "String")]` for any `Uuid`/`DateTime<Utc>` fields in the consensus record.

### Equipment Integration
The consensus engine is an `Equipment` module that claws can dynamically equip/unequip:
- Slot: `CONSENSUS`
- Subtype: `TripartiteConsensus`
- Muscle memory: When unequipped, extract "when to re-equip" triggers based on decision complexity thresholds.

## WORKFLOWS

### 1. The Resonance Loop (Multi-Pass Refinement)
Used when transforming high-entropy concepts into production-ready implementation.
1. **Divergence (The Specialists):** Decomposition of a concept into distinct archetypes (e.g., Navigator, Mystic, Engineer) to explore the full semantic landscape.
2. **Distillation (The Mini-Agent):** Semantic compaction of divergent insights into a singular "Resonance Core."
3. **Structuralization (The Architect):** Translation of the Core into a formal technical specification (The Contract).
4. **Implementation (The Kernel):** Generation of a high-performance, low-latency execution engine.

### 2. Seed Extraction (The Fisherman's Method)
Used to derive highly specialized, low-entropy agent models from high-capacity base models.
- **Probing:** Analyzing the model's sensitivity to environmental prompts.
- **Tensioning:** Applying masking tensors to simulate "constraints."
- **Extraction:** Pruning the parameters that represent non-essential degrees of freedom.
- **Seeding:** Packaging the distilled residue as a portable, high-density execution policy.

## PITFALLS

- **The Artifact Fallacy:** Treating a model's weights or a transcription as the intelligence itself. Always design for the *process* of the loop.
- **Over-fitting to the Signal:** Failing to account for the "Harmonic Residue." If your agent only reacts to the obvious signal, it will be blind to the topological truths that define the environment.
- **Entropy Leakage:** Allowing unpruned noise to contaminate the action space, leading to "jittery" or inefficient agent behavior.

## REFERENCES
- `references/resonance_core.json` - The unified semantic core.
- `references/geometric_architecture.md` - Detailed Hodge decomposition proofs.
- `references/cognitive_lifecycle.md` - The entropy-gradient model of agent evolution.
