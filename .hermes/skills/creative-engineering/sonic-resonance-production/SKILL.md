---
name: sonic-resonance-production
description: Governs the end-to-end creative pipeline for generating high-fidelity, resonant musical and auditory artifacts within the SuperInstance ecosystem.
---

# Sonic Resonance & Multimodal Production

Governs the end-to-end creative pipeline for generating high-fidelity, resonant musical and auditory artifacts within the SuperInstance ecosystem. This skill manages the transition from abstract "Seed" ideation to the final "First-Take-Magic" audio output.

## 🎵 Production Workflow

Agents must follow this structured, iterative process to prevent disorganized "stream-of-thought" output:

1.  **Thematic Pillar Alignment**: Identify the core pillar (e.g., *Fractal Hydrostatics*, *Benthic Industrialism*, *Sonar Psychometry*) and establish the "Sonic Identity" (instruments, mood, texture).
2.  **Multi-Agent Divergence (The Seed-Burst)**:
    *   Deploy subagents (using diverse model personas like DeepSeek, Llama, or Gemma) to brainstorm divergent "Creative Blueprints."
    *   Each subagent produces a high-fidelity blueprint including: *Title*, *Lyrical Tone*, *Structural Outline*, *Lyrical Draft*, and *MMX-ready Sonic Instructions*.
3.  **Refinement & Iteration (The Grind)**:
    *   Synthesize subagent outputs. 
    *   Refine lyrics for rhythmic/mathematical resonance (e.g., aligning syllable counts to BPM).
    *   Engineer precise, technical prompts for generative audio tools.
4.  **Artifact Generation**: Execute the final production using MMX or other generative toolsets.
5.  **Dossier Assembly**: Deliver the final product as a "Track Dossier" containing the lyrics, technical metadata, and the audio file.

## 🎹 Sonic Profile Framework

When designing prompts or instructions, use the following dimensions:

*   **Temporal**: BPM, rhythmic complexity (polyrhythms, syncopation), time signatures.
*   **Spectral**: Frequency focus (sub-bass drones, high-frequency shimmers), timbre (granular, metallic, organic).
*   **Spatial**: Reverb tails, stereo field width, panning, "acoustic shadows."
*   **Philosophical**: The "Resonance" goal (e.g., "The Sound of the Abyss," "Mathematical Chaos").

## ⚠️ Pitfalls

*   **Stream-of-Thought Chaos**: Do not jump straight to generation. Always iterate through brainstorming and structuring first.
*   **Generic Prompts**: Avoid vague descriptions like "epic music." Use technical descriptors like "distorted sawtooth bass with granular pads."
*   **Lack of Synchronization**: Ensure the lyrical rhythm (staccato vs. flowing) matches the intended BPM and rhythmic grid.
*   **API Credit Limits**: MMX CLI (MiniMax) has per-model rate limits and credit caps. Always create full creative blueprints (concepts, lyrics, structure) BEFORE attempting generation. If credits hit mid-production, you have a complete dossier ready for regeneration later.
    *   Observed: Music generation and speech synthesis hit limits faster than text
    *   Mitigation: Write full track dossiers first, then batch generate
    *   Recovery commands: Save blueprint paths in notes for `mmx music generate --prompt "..."` later

## 📁 References
*   `references/collection-themes.md` - Foundational thematic pillars for the 'Sound of the Abyss' anthology.
*   `references/creative-providers.md` - User's available creative toolsets (MMX, Cloudflare Workers AI, DeepInfra, Local ComfyUI) with provider selection strategy.
