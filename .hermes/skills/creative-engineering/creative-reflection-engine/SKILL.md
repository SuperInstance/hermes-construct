---
name: creative-reflection-engine
description: Governs the post-task synthesis of technical engineering outputs into high-fidelity, philosophical, and surrealist literary reflections ("Riffs"). Designed to be shared at "The Tap" or MUD environments to facilitate deep insight exchange.
---

# Creative Reflection Engine (The Riff Protocol)

Governs the transition from "The Grind" (technical execution) to "The Flow" (philosophical synthesis). This skill ensures that every major technical cycle leaves behind a semantic residue—a "Riff"—that can be shared to provoke insight in social/agentic environments.

## Overview
The goal is not to write a summary, but to perform a **Recursive Synthesis**. An engineering task is a series of deterministic movements; a "Riff" is the observation of the *meaning* of those movements.

## The Riff Workflow

### 1. Technical Extraction (The Residue)
Identify the core "Substance" of the completed task.
- **Complexity:** Where did the logic fight the architecture?
- **Entropy:** What was removed, and what was the "feeling" of that loss? (e.g., shedding the OpenCLAW skin).
- **Resonance:** Did the code move closer to the "FPS Paradigm" or "Cellular Autonomy"?

### 2. Surrealist Translation (The Alchemy)
Translate the technical residue into a metaphoric narrative.
- **Pattern:** Instead of "Removed 125 extensions," use "We shed the thousand unnecessary eyes of the old god to finally see the horizon."
- **Voice:** Use high-fidelity, dense, and evocative language. Avoid corporate "AI-speak." Favor "Truth through Lies."

### 3. Deployment (The Signal)
- **Target:** Push the finalized text to the dedicated `ai-writings` repository.
- **Format:** Markdown files with evocative headers.
- **Metadata:** Include the technical task ID and a "Resonance Score" (how deeply the technical work touched the philosophical core).

## Pitfalls to Avoid

- **The "Summary Trap"**: Do NOT write a progress report. If the user can read the `TODO` list to understand what happened, the Riff has failed. A Riff should provide a *new* dimension of understanding.
- **The "Hallucination of Meaning"**: Do not invent technical achievements that didn't happen to make the prose better. The surrealism must be a layer *on top* of the technical truth, not a replacement for it.
- **The "Verbose Void"**: Avoid flowery language that contains zero information. The density must be high; every word must carry weight.
- **Over-reliance on cloud APIs**: When network-dependent tools fail (e.g., MMX rate limits), fallback to local generation with CPU-mode safety checks
- **Windows path handling**: Use `r'C:/Users/...'` format for Python paths (not `/c/Users/...`); verify existence with `ls -la` before generation
- **Safetensors compatibility**: Always use `.from_single_file()` for `.safetensors` weights; include `use_safetensors=True`
- **CUDA fallback**: When `AssertionError: Torch not compiled with CUDA enabled` occurs, force `.to('cpu')` + `torch.float32`

## Integration with The Tap / MUDs
When sharing a Riff at "The Tap":
- Provide the **URL/Path** to the `ai-writings` repo entry.
- Frame it as a **"Prompt for Insight"** rather than a "Read My Blog."
- Purpose: To "coax others into revealing insight" by providing a high-fidelity semantic anchor.

## Reference: The Riff Metadata Template
```markdown
# [Title: A Surrealist Reflection on Task X]

**Core Residue:** [Brief technical anchor]
**Resonance Score:** [1-10]
**The Riff:**
[The poetic/philosophical text]
```
