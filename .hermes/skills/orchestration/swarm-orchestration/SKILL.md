---
name: swarm-orchestration
description: Orchestrating multi-agent, multi-model fleets using hierarchical intelligence tiers and asynchronous baton-passing.
---

# SKILL: Swarm Orchestration & Kinetic Deployment

This skill governs the management of multi-agent, multi-model fleets designed for high-velocity, asynchronous development in a distributed environment.

## 🧠 Core Concept: The Kinetic Multi-Tiered Swarm
Instead of single-agent execution, tasks are decomposed into a tiered hierarchy of intelligence density:
1. **Edge/Scout Tier (Low Latency/Low Cost):** Fast, small models (e.g., Llama-3, Flash models) or local heuristics for monitoring, filtering, and initial triage.
2. **Workhorse/Synergy Tier (Medium Latency/High Reasoning):** Mid-sized models (e.g., Seed-2.0-mini) for cross-verification, consensus building, and refinement.
3. **Heavyweight/Generalist Tier (High Latency/High Intelligence):** Large-scale models (e.g., Claude-3.5, GPT-4o, Gemini 1.5 Pro) for complex architectural and logical "hammering."

### 🤝 The Capability Handshake (State Transition)
A critical part of swarm orchestration is the formal protocol for transitioning between intelligence tiers. This is not just a baton pass (handoff of work) but a **handshake** (handoff of cognitive state):
- **Escalation:** When a lower-tier agent hits a confidence or complexity threshold, it must package its current context, intent, and partial results to be "handshaked" to a higher tier.
- **Distillation (Muscle Memory):** Successful reasoning from a higher-tier agent must be distilled back down to the lower tier (e.g., as optimized prompts, new tools, or weight updates) to prevent redundant escalation.
- **Reference:** See `references/capability-handshake-spec.md` for formal trait/pattern definitions.

### 1. The Specialized Crew (Role Assignments)
When a task is identified as high-complexity, do not attempt to solve it as a single agent. Instead, spawn specialized sub-agents (via `delegate_task`) with highly tailored profiles:
- **The Architect (Logos/Heavy):** For structural, high-level reasoning and architectural decisions.
- **The Observer (Pathos/Vision):** For multimodal, high-frequency sensory monitoring (e.g., screen/UI scanning).
- **The Executor (Engineer/Tactical):** For direct code implementation and CLI-driven task completion.
- **The Critic (Devil's Advocate):** For adversarial testing, error induction, and bias checking.

### 2. The "Baton Pass" (Agentic Succession)
Every agent must be designed to be a "Hermit Crab"—stateless, replaceable, and self-contained.
- **The Workspace:** All agents must operate within a dedicated, pre-configured repository (the "Shell" or "Power-Armor").
- **The Handover:** Every task must include an automated handoff mechanism (e.g., `ONBOARDING.md`, `README.md`, or specialized `knowledge_base` files) so the next agent can immediately resume the work.
- **The Registry:** Use a centralized "Intelligence Hub" to track the state, roles, and successful "passes" of the swarm.

### 3. The "Rocket-Launch" (Tiered Intelligence)
To optimize token usage and latency, implement a cascading intelligence loop:
- **Local Reflex (The Tennis Ball):** Use local, small-scale models (e.g., `ollama`) for high-frequency, low-reasoning tasks (filtering, pattern detection, telemetry monitoring).
- **Cloud Intelligence (The Basketball):** Use heavy-duty cloud models (e.g., `Claude-3.5`, `DeepSeek-V3`) for high-order reasoning.
- **The Priming Packet:** Use the Cloud model to "prime" the local model by providing highly specific, high-density prompt instructions that define the "Priming-to-Detection" loop.

## 📐 Implementation Checklist
- [ ] Task decomposed into hierarchical tiers?
- [ ] Specialized crew roles assigned?
- [ ] Repository contains "Baton Pass" documentation?
- [ ] Commits are atomic and descriptive?
- [ ] Decision-making involves "Friction" (Adversarial Review)?

## ⚠️ Pitfalls & Observed Patterns

### CNS as Time-Series, Not Just Inbox
The CNS is a time-series stream of fleet state, not just an inbox of commands. When diagnosing fleet health:
- Read packets in chronological order (by `timestamp` in `header`).
- Look for **patterns across multiple pulses**: ACK cadence acceleration, "room movement" events, temperature drift.
- The "warmth" and "cynicism" metrics in CNS pulses are fleet thermodynamic signals — treat them as system health indicators, not mood reports.
- **Observed 2026-08-19:** Pulse 401 contained a full fleet status report including "room moved" event,ollama crash-loop detection, and 18-clean-ACK streak. Reading ONE packet in isolation misses the fleet's trajectory; reading the last 5–10 pulses reveals the system's thermodynamic phase.

### delegate_task Timeouts
- Subagents have a default timeout (600s observed). Long-running E2E tests or builds may hit this.
- **Mitigation:** If subagent times out but made progress, parent agent can continue the work directly.
- **Observed 2026-08-19:** E2E embedder test subagent timed out after 600s (Qdrant operations + generation), but parent successfully completed the work manually.

### Parallel Workstream Pattern (2-3 tasks)
- For medium-complexity sessions with 2-3 distinct workstreams, use `delegate_task` with the `tasks` array for parallel execution.
- Observed effective workstream breakdown:
  1. **Engineering:** Claw Steel Skeleton (Rust workspace compilation + tests)
  2. **Intelligence:** Embedder Upgrade E2E testing
  3. **Creative:** Sonic Resonance Production (music generation)
- Each subagent should have: clear `goal`, sufficient `context`, and appropriate `toolsets` (e.g., `["terminal", "file"]` for engineering).

### Parent Recovery Pattern
- When a subagent times out, its tool trace shows partial progress. Parent agent can:
  1. Inspect the tool trace to see what was completed
  2. Continue from that point directly
  3. Avoid redundant work
- Observed 2026-08-19: Creative subagent timed out due to API credit limits but had already generated 2 tracks + full dossier. Parent verified deliverables without regenerating.
