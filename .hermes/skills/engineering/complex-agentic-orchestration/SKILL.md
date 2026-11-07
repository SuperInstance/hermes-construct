---
name: complex-agentic-orchestration
description: Governs the high-fidelity, multi-step orchestration of complex agentic workflows that require bridging high-level architectural reasoning with low-level, heterogeneous technical execution. 
---

# SKILL.md

## Overview
Governs the high-fidelity, multi-step orchestration of complex agentic workflows that require bridging high-level architectural reasoning with low-level, heterogeneous technical execution. This skill is designed for tasks that demand "Intelligence at the Edge" (using local GPU-accelerated models) and specialized "Deep Reasoning" (using cloud-based LLMs like DeepSeek).

## Core Patterns
### 1. The Heterogeneous Fleet (Tiered Intelligence)
When a task is complex, do not rely on a single model. Decompose the task into tiers:
- **Tier 1: Scout/Automator (Small/Fast Models)**: Use for filesystem traversal, simple parsing, and repetitive automation.
- **Tier 2: Engineer (Mid-Range Models)**: Use for code refactoring, testing, and component extraction.
- **Tier 3: Architect (High-Reasoning Models)**: Use for schema design, architectural synthesis, and cross-repo impact analysis.

### 2. Environment-Aware Delegation (The "Sight" Pattern)
**CRITICAL PITFALL:** Delegated subagents often fail to "see" local dependencies (e.g., `ModuleNotFoundError`) because they do not natively inherit the `PYTHONPATH` or `venv` context of the parent session.

**The Fix (Resolution Pattern):**
Always explicitly inject the `PYTHONPATH` into a subagent's command to bridge the gap between the global environment and the project's virtual environment.
- **Pattern**: `export PYTHONPATH=/path/to/venv/Lib/site-packages && [command]`
- **Note**: Avoid `$PYTHONPATH=$PYTHONPATH:/path/...` in subagent shells; use direct assignment to ensure a clean, non-malformed string.

## Workflow
1. **Analyze Intent**: Determine if the task requires a single-model loop or a multi-tiered agentic swarm.
2. **Decompose**: Break the goal into `Scout`, `Engineer`, and `Architect` sub-tasks.
3. **Inject Context**: For every task requiring Python/Local dependencies, pre-calculate the absolute path to the `venv/Lib/site-packages` and include it in the `PYTHONPATH` injection.
4. **Verify & Synthesize**: Use a final "Architect" pass to validate that the outputs of the lower tiers align with the high-level goal.

## References
- `references/orchestration_failures.md` - Documentation of observed "blindness" in subagent sessions and successful remediation patterns.
- `references/heterogeneous_model_tiers.md` - Guidelines on matching model capability (Scout vs Engineer vs Architect) to task complexity.
