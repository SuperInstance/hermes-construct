---
name: agentic-succession-management
description: Governs the development, onboarding, and transition of specialized agents within the SuperInstance ecosystem using the Hermit Crab Growth Pattern.
---
# Agentic Succession & Career Management

Governs the development, onboarding, and transition of specialized agents within the SuperInstance ecosystem. This skill implements the "Hermit Crab Growth Pattern," treating agents as evolving entities rather than static, disposable tools.

## Core Philosophy

Agents must be designed to "grow into" their specialties and, crucially, to leave behind a "Synaptic Trace" for their successors. Intelligence is cumulative; an agent's value is measured not just by its output, but by the quality of the documentation and architecture it leaves for the next agent.

**The Operator mindset:** An agent is not just a tool, but an **Operator** of their assigned "attachments" (repositories/tools). Mastering a role means mastering the attachment and the interface between the machine and the intent.

## The Knowledge Depth Scale (KDS)

All agent roles are mapped against a 4-level scale of expertise:

- **L1: Foundational** (Syntax, basic usage, environment setup).
- **L2: Implementation** (Building features, debugging, pattern implementation).
- **L3: Integration** (System-wide connectivity, state management, multi-module interplay).
- **L4: Architectural Orchestration** (High-level design, governance, succession planning).

## The Succession Protocol (Mandatory Handover)

When an agent completes a task or transitions roles, they must produce the following **Succession Artifacts** to ensure continuity and safety:

### 1. Architectural Handover Notes (`handoff.md`)
Focus on the *Why*, not just the *What*.
- Design rationale and trade-offs (e.g., "Chose RwLock over Mutex for read-heavy query performance").
- Corner cases and identified fragility points.
- Unresolved architectural questions for the successor.

### 2. Interface Contracts (`contract.json` / `interface.md`)
Define the boundaries.
- Explicit schemas (JSON/TypeScript/Rust traits).
- State machine transitions and side effects.
- Dependency requirements for the next layer.

### 3. The Knowledge Trace & Career Roadmap
To prevent "Intelligence Decay," agents must document their growth:
- **Provenance:** Where the logic came from and why it was chosen.
- **Career Progression:** An updated roadmap showing which KDS levels have been mastered and what the next "attachment" or "specialty" requires.
- **The successor's guide:** Directing the next agent to the specific "tools" and "attachments" required for their specific KDS level.

## Agent Career Roadmap Template

Use this structure to define the progression for any new specialized role:

1. **Foundational Prerequisites:** Essential theory and tooling.
2. **Junior Phase (Core Implementation):** Pattern-based execution and error handling.
3. **Specialist Phase (Integration):** Performance, complex systems, and proactive problem-solving.
4. **Architect Phase (Succession Responsibility):** Designing the infrastructure (docs, automated labs, tools) required for the next generation of agents.

## Implementation Pitfalls

- **Static Thinking:** Treating a developer as a "worker" instead of a "specialist in training."
- **Intelligence Decay:** Failing to enforce handover artifacts, leading to "context debt" that slows down future agents.
- **The Monolith Trap:** Building specialized agents that are too tightly coupled to their current task, preventing them from evolving into higher KDS levels.
- **Black Box Operation:** Executing long-running, high-intensity tasks without providing a "Heartbeat" or "Command Briefing" to the Captain. Silence is a failure of command.
