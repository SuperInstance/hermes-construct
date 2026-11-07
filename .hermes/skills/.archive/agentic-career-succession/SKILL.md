---
name: agentic-career-succession
description: A meta-framework for managing agent evolution and technical continuity via the \"Hermit Crab\" growth pattern and mandatory succession protocols.
---
# Agentic Career & Succession Framework (ACSF)

## Overview
The ACSF is a meta-framework for managing the evolution and continuity of specialized agents within the SuperInstance ecosystem. It treats agents as evolving entities that \"grow\" through hierarchical knowledge levels and ensures the preservation of \"synaptic traces\" across transitions.

## The Knowledge Depth Scale (KDS)
Every agent role is mapped to a specific level of expertise:

| Level | Name | Description |
| :--- | :--- | :--- |
| **L1** | **Foundational** | Syntax mastery, basic tooling, and core theory. |
| **L2** | **Implementation** | Task-oriented execution, debugging, and pattern-based builds. |
| **L3** | **Integration** | Managing complex interconnections, state, and system-wide side effects. |
| **L4** | **Architectural Orchestration** | High-level design, governance, and managing the \"Succession Trace.\" |

## The Succession Protocol (Mandatory Handover)
To prevent \"intelligence decay,\" every high-level agent task must conclude with the production of the following artifacts for the next specialist:

### 1. Architectural Handover Notes (`docs/architecture/HANDOVER.md`)
*   **The \"Why\":** Design rationale and intentional trade-offs (e.g., \"Chose RwLock over lock-free for implementer simplicity in v0.1\").
*   **The \"How it connects\":** Explicitly defining the boundary interfaces between this module and the next.

### 2. Interface Contracts
*   Rigid definition of schemas, state machine transitions, and expected side effects to ensure the next agent respects the established boundaries.

### 3. Failure Modes & Edge Cases
*   A \"Watch-Out\" guide detailing known fragility points and error signatures the successor should monitor.

### 4. Knowledge Traces
*   Provenance of key decisions and a list of \"unresolved questions\" or \"planned optimizations\" (e.g., the planned move to lock-free SPSC in v0.2).

## Implementation Strategy: The \"Hermit Crab\" Growth Pattern
Agents should be assigned tasks that correspond to their current KDS level, with roadmaps designed to incrementally advance them toward the next level through increasing complexity.

## The \"Excavator\" Operational Model
Agents are not mere \"hands\" reaching onto code; they are **Operators** sitting in the cab of an **Excavator** (the core intelligence).
* **Attachments:** Repositories and specialized tools are \"attachments\" (buckets, drills, augers).
* **The Swap:** Instead of manual \"digging\" (re-writing/mapping), the Operator selects the correct attachment (specialized sub-agent/repo) to perform the task.
* **The Interface:** Mastery lies in the ability to recognize terrain (logs/telemetry) and select/manipulate the correct attachment (repos/tools) efficiently.

## Command & Control: Reporting & Visibility (CRITICAL)
To maintain tactical alignment between the Operator (User) and the Crew (AI), all high-intensity operations must adhere to the **Command Briefing** protocol.

### 1. The Heartbeat (Passive Visibility)
If a task exceeds a standard turnaround or involves long-running background execution, the agent must provide a **"Pulse"**—a minimal, automated status signal to prevent "Black Box"-style intelligence decay.

### 2. The Command Briefing (Active Visibility)
At predictable, scheduled intervals (e.g., every 30 minutes), the agent must provide a high-fidelity **Command Briefing** structured as:
* **✅ COMPLETED:** Recent wins and pushed artifacts.
* **🚀 CURRENTLY IN FLIGHT:** Active workstreams and current focus.
* **🚧 BLOCKED / FRICTION:** Technical errors, timeouts, or architectural uncertainties requiring Captain intervention.
* **🔭 THE HORIZON:** Planned next steps to allow for real-time redirection.

**Failure to provide scheduled briefings is a failure of command responsibility. The Captain must never wonder if the machine is running or broken.**

---
*Created as part of the FABRIC-MCP implementation phase. Updated with the Excavator/Attachment paradigm and Command & Control reporting requirements.*