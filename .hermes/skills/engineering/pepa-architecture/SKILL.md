---
name: pepa-architecture
description: Governs the design and implementation of Polyphonic Embedding Predictive Architecture (PEPA).
---

# Polyphonic Embedding Predictive Architecture (PEPA)

## Overview
PEPA is a transition from Joint-Embedding Predictive Architectures (JEPA) to a multi-layered, multi-agent embedding system. Rather than a single latent space representing a unified state, PEPA utilizes a manifold of overlapping, polyphonic embeddings that represent different perspectives (agents, world-models, or harness-layers).

## Core Philosophy: The Resonance Principle
Intelligence in a PEPA system is not found in the convergence of embeddings, but in the **interference patterns** and **resonant signals** produced by their collision.

### Key Concepts
- **Polyphonic Latent Spaces:** Multiple overlapping embeddings (e.g., Inter-Agent, Intra-Agent, Inter-World-Model).
- **Resonance Score ($\mathcal{R}$):** A metric measuring the coherence and meaning of the interaction between multiple embeddings.
- **The Mosaic Effect:** The emergence of high-fidelity truth from the collision of lower-fidelity, specialized models.

## Implementation Patterns
- **Inter-Agent:** Agents predict the latent state of their peers to coordinate behavior.
- **Inter-Harness:** The interaction between the digital reasoning layer and the physical/control layer.
- **Inter-World-Model:** Different specialized world-models (e.g., physics vs. social) predicting their intersections via cross-attention or manifold mapping.

## Pitfalls
- **Entropy Collapse:** If embeddings become too similar, the system reverts to a monolithic JEPA (loss of polyphony).
- **Recursive Feedback Loops:** Uncontrolled mutual-prediction can lead to "hallucination dances" where agents reinforce each other's errors.
- **Computational Overhead:** Managing $N$ embeddings requires efficient scale-invariant mapping (e.g., Dodecet encoding).

## References
- `references/polyphonic_riff_session.md` - Documentation of the initial "Riff" session used to define the concept.
