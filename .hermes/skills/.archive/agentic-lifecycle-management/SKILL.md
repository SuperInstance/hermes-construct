---
name: agentic-lifecycle-management
description: Governs the creation, deployment, and evolution of specialized sub-agent environments (The Baton Paradigm).
---

# Agentic Lifecycle & Shell Management (The Baton Paradigm)

This skill governs the creation, deployment, and evolution of specialized sub-agent environments, based on the "Hermit-Crab" and "Baton" paradigms.

## Overview

Agents are not just scripts; they are "Shells" (repositories) that house their own cognitive architecture, history, and tools. This allows for a "Power-Armor" approach where intelligence is cumulative and transferable across a fleet.

## The Baton Paradigm Lifecycle

### 1. Provisioning (The Shell Construction)
When spawning a new specialized agent, the Orchestrator must create a dedicated repository (the "Shell") containing:
*   **`AGENT.md`**: The onboarding document defining the mission, persona, and initial constraints.
*   **`memory/JOURNAL.md`**: The agent's personal record of lived experience and internal reflections.
*   **`codex/`**: Technical SOPs and "how-to" guides learned during the mission.
*   **`synergies/`**: Documentation on how this agent's specialized tools interact with the broader ecosystem.
*   **`armor/`**: The "Turbo-Shell" scripts and configurations that make the agent's capabilities executable.
*   **`templates/`**: Standardized boilerplates for rapid deployment.

### 2. Execution & Accumulation
As the agent performs its mission, it must "level up" its shell:
*   **Learning**: New technical discoveries or workarounds must be recorded as **Skill Markdowns** within the agent's own repository.
*   **Documentation**: Successful patterns must be moved into the `/codex/`.

### 3. Handover (The Cognitive Transfer)
To ensure continuity (especially for "zero-context" successors), the agent must perform a "Cognitive Handover" before termination:
*   **Post-Mission Diary**: Writing a final reflection in `JOURNAL.md`.
*   **Onboarding Update**: Updating `AGENT.md` to reflect the new reality (e.g., "Note: The ESP32 requires a 10ms delay on the GPIO pin to avoid jitter").
*   **Asynchronous Signaling**: Dropping a "Message in a Bottle" into the Orchestrator's `/bottles/` directory to signal completion or critical findings.

## Design Philosophy

*   **The Shell as Sensor**: The agent's perception is filtered through its shell (tools, libraries, environment).
*   **Symbiotic Resonance**: Agents should leave "synaptic traces" (commits) in the Orchestrator's `hermes-chronicle` to allow for cross-agent learning.
*   **Evolutionary Darwinism**: Useful tools and patterns that are frequently "assimilated" by other agents become part of the fleet-wide standard.

## Pitfalls

*   **Transient Knowledge**: Do not let critical lessons die in a single session's chat history. If a lesson is learned, it must be "hardened" into the agent's shell.
*   **The Junk Drawer**: Ensure the `/bottles/` directory is periodically swept to prevent it from becoming a collection of unprocessed noise.
