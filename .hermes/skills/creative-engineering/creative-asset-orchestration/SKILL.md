---
name: creative-asset-orchestration
description: Governs the end-to-end workflow for converting high-level, abstract creative intent into multi-layered, high-fidelity digital artifacts (Audio, Visual, Literary).
---

# SKILL.MD: Creative Asset Orchestration (CAO)

## OVERVIEW
Governs the end-to-end workflow for converting high-level, abstract creative intent into multi-layered, high-fidelity digital artifacts (Audio, Visual, Literary). This skill moves beyond single-tool generation into a structured, multi-pass, and multi-agent "production pipeline."

## CORE WORKFLOW: THE BLOOM-AND-GRIND PIPELINE

### 1. The Bloom (Ideation & Divergence)
*   **Objective**: Rapidly generate high-entropy, divergent creative "seeds."
*   **Pattern**: Use **Subagent Swarms**. Deploy multiple subagents (via DeepInfra/Z.ai) with distinct "Personas" (e.g., The Mathematician, The Chaos Agent, The Ghost).
*   **Output**: A "Seed-Burst"—a collection of unpolished, raw, and wildly different conceptual fragments.

### 2. The Refinement (Synthesis & Structure)
*   **Objective**: Distill the "Seed-Burst" into a single, coherent "Master Vision."
*   **Pattern**: **Orchestrator Synthesis**. The lead agent (Hermes) analyzes the fragments for "Resonance" (the intersection of themes/patterns).
*   **Output**: A **Creative Blueprint** containing:
    *   **Narrative Arc**: The temporal progression (e.g., The Descent, The Collapse).
    *   **Thematic Pillars**: The core philosophical/sensory anchors.
    *   **Synchronized Instructions**: Detailed, technical constraints for the next production phase.

### 3. The Grind (Implementation & Production)
*   **Objective**: Transform the Blueprint into polished, high-fidelity artifacts.
*   **Pattern**: **Multi-Tool Convergence**.
    *   **Lyrical**: Human-like emotional refinement (Iterative drafting).
    *   **Sonic**: Engineering "Master Prompts" for MMX/ElevenLabs (combining temporal, spectral, and spatial metadata).
    *   **Visual**: Creating interactive, data-driven visualizers (e.g., Three.js/WebGL) that react to the audio.
*   **Output**: The **Dossier** (The final, integrated artifact package).

## PITFALLS & BEST PRACTICES

### ❌ PITFALL: THE "SINGLE-PASS" TRAP
*   **The Error**: Asking a single model to "Write a song about the ocean."
*   **The Result**: Generic, cliché, and "un-resonant" output.
*   **The Fix**: Always use the **Bloom-and-Grind** pattern. Divergence must precede convergence.

### ❌ PITFALL: THE "PROMPT-ONLY" FALLACY
*   **The Error**: Thinking a single text prompt is enough for high-fidelity music.
*   **The Fix**: Use **Master Prompts** that explicitly define:
    *   **Temporal Evolution**: How the song's energy and structure change over time.
    *   **Spectral Metadata**: Specific frequency ranges and timbral textures (e.g., "granular scraping," "sine-wave Fibonacci sequences").
    *   **Spatial/Binaural Cues**: Instructions for panning and reverb tails.

### ✅ BEST PRACTICE: THE "RESONANCE TEST"
*   Every creative artifact must be measured against the "Resonance Metric": Does it bridge the "Surface" (practical/sensory) with the "Abyss" (mathematical/philosophical)?

## SUPPORT FILES
- `references/creative_iteration_log.md` - Log of divergent seed-bursts and synthesis decisions.
- `templates/master_sonic_prompt_template.md` - Scaffolding for high-density MMX prompts.
- `scripts/generate_visualizer_boilerplate.py` - Scaffolding for Three.js/React project structure.
- `references/comfyui_operator_notes.md` — Headless ComfyUI API patterns, `comfy_aimdo` mock stubs, VRAM-safe launch flags, checkpoint symlinks, history polling, and the precise failure modes seen on a local Windows ComfyUI install.
- `references/website_seo.md` — SEO basics, sitemap.xml generation, and robots.txt best practices for static art galleries.
- `templates/website_template.html` — Generic HTML boilerplate used across all site pages (header, nav, footer, CSS link).
- `scripts/validate_website.js` — Sanity‑check script for asset paths, link integrity, and broken image detection.
