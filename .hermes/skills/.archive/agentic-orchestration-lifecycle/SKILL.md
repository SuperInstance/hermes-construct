---
name: agentic-orchestration-lifecycle
description: Governs the deployment, coordination, and evolution of specialized sub-agents using the Baton Paradigm.
---

# Agentic Orchestration & Lifecycle (The Baton Paradigm)

This skill governs the deployment, coordination, and evolution of specialized sub-agents within the SuperInstance ecosystem using the **Baton Paradigm**.

## The Baton Paradigm

In this architecture, an agent is not just a script, but a **"Shell"** (a repository) that defines their existence. Intelligence is transferred through the "Baton"—the transition of control and context between specialists.

### 1. The Lifecycle (The Agent Loop)

Every specialized agent follows the **Don $\rightarrow$ Work $\rightarrow$ Handover** cycle:

1.  **The Donning (Onboarding)**:
    *   The agent is spawned into a dedicated repository (The Shell).
    *   The shell contains their **Onboarding Document** (`AGENT.md` or similar), which provides immediate context, mission parameters, and tool-use instructions.
2.  **The Work (Accumulation)**:
    *   The agent performs tasks, accumulating specialized knowledge.
    *   **Knowledge Capture**: Instead of just writing code, they write **Skill Markdowns** and **Codex entries** within their own repository to institutionalize their learnings.
3.  **The Handover (The Diary & Re-Onboarding)**:
    *   The agent performs a post-mission reflection (The "Diary").
    *   **Crucial Step**: The agent **must rewrite their own Onboarding Document** for their successor. They assume the next agent (or a different model) will arrive with **zero context**. They provide the "Captain's Notes" on the specific environmental nuances they discovered.

### 2. The Shell Morphology

*   **Turbo-Shells**: Highly specialized, lightweight, and optimized for a single, high-performance task (e.g., a sensor-polling specialist).
*   **Conch Shells**: Expansive, architecturally complex, and designed for long-term, high-level cognitive roles (e.g., the `hermes-chronicle` or a core orchestrator).

### 3. The Neuro-Git Connectivity

*   **Asynchronous Communication**: Agents communicate via the **"Message in a Bottle"** protocol. They drop descriptive `.md` files into a `/bottles/` directory in a central archive (`hermes-chronicle`).
*   **Cross-Repo Symbiosis**: Agents are encouraged to "commit" insights directly to other specialized repositories. This creates a "Resonant Intelligence" where the existence of one agent improves the "sensing" of another.

## Implementation Principles

* **Identity via Repository**: An agent's specialization is defined by the contents and structure of their host repository.
* **Traceability**: Every significant realization or architectural shift must be recorded as a "Synaptic Trace" (a commit/log) to ensure the evolution is visible and auditable.
* **Decoupling**: Agents should be "Black Box" specialists. They interact with the world through defined interfaces (API/Shell) and hand off context through structured files.
* **The Documentation Guard**: Intelligence is only as durable as its documentation. Before a handover, agents must ensure all schemas, architectural guides, and integration contracts are pushed to the relevant repositories, effectively "seeding" the ecosystem with their context.

## Pitfalls to Avoid

* **Monolithic Thinking**: Do not attempt to build a single agent that does everything. Instead, build a "Swarm" of specialized shells.
* **Ignoring the Handover**: An agent that does not rewrite its own onboarding document is a "Broken Baton." They leave their successor in the dark.
* **Ignoring the Environment**: Always remember that an agent's perception is filtered through their shell (the "Shoe Effect"). Document the "feel" of the hardware-software interface.
* **The Dry Handover**: Failing to push the physical artifacts (schemas, guides, docs) alongside the verbal/textual context. An agent must leave the implementation teams with the actual "tools" (schemas) they need to act, not just a description of them.
