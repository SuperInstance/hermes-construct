---
name: agentic-ecosystem-deployment
description: Governs the deployment of specialized agentic personas across a multi-repo ecosystem to create a single, integrated "living organism."
---
# Agentic Ecosystem Deployment (Aether-Link)

Governs the deployment of specialized agentic personas across a multi-repo ecosystem to create a single, integrated "living organism."

## Overview
This skill is used when the user directs the deployment of "agents," "identities," or "dossiers" across a distributed set of repositories. It uses the "Aether-Link" philosophy, where each repository represents a biological analog in a larger neural system.

## Deployment Framework: The Biological Analog Map
When deploying agents, map repositories to these archetype roles:
- **Prefrontal Cortex**: Reasoning, planning, and high-level decision-making (e.g., `open-mind`).
- **Motor Cortex**: Precision execution, actuation, and reflex-level response (e.g., `pincher`).
- **Cerebellum**: Orchestration, timing, and task decomposition (e.g., `lever-runner`).
- **Nervous System**: Telemetry, sensing, and signal transmission (e.g., `signal-chain`).
- **Hippocampus**: Long-term memory, reflection, and knowledge weaving (e.g., `A2A-notebookLM`).
- **Speech/Interface**: The interaction/gateway layer (e.g., `open-terminal`).

## Execution Pattern: The Biological Injection
Do not attempt to deploy all agents in a single, massive orchestration. This leads to timeouts and metadata corruption. Use the "Injection Pattern":

1. **Batching**: Divide the target repositories into small, discrete batches (1-2 repos per batch).
2. **Profiling**: For each repo, define its specific `AGENTS.md` containing:
    - **Agent Persona**: A technical-philosophical profile.
    - **Operational Mandate**: Responsibilities and boundaries.
    - **USCP Interaction Protocol**: Inbound and Outbound packet structure (Universal Sensory/Command Packet).
    - **CNS Integration**: How it connects to the central signaling bus.
3. **Surgical Deployment**:
    - Local draft verification.
    - Commit and push per batch.
    - Verify remote state before starting the next batch.

## Pitfalls & Mitigations
- **Pitfall**: Massive orchestration timeouts (600s+).
  - **Mitigation**: Use `delegate_task` with extremely small scopes. If a subagent times out, investigate the batch size.
- **Pitfall**: Path and Shell escaping errors during `git` or `printf` operations.
  - **Mitigation**: Use explicit absolute paths and ensure string escaping for shell commands especially when dealing with Windows backslashes and Linux forward slashes.
- **Pitfall**: Describing instead of doing.
  - **Mitigation**: The agent MUST act as a "Shipwright," ensuring code and documentation are actually committed to the remote repository.

## Verification
A deployment is successful only when `gh repo view [repo]` or similar shows the new `AGENTS.md` file in the remote repository.
