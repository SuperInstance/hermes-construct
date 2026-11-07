---
name: agentic-ecosystem-orchestration
description: Governs the deployment, communication, and evolution of the multi-agent \"Neuro-Git\" ecosystem (Hermit-Crab, Baton paradigm, and Message-in-a-Bottle).
---

# Agentic Ecosystem & Succession (The Neuro-Git Framework)

This skill governs the management, deployment, and evolution of the SuperInstance multi-agent ecosystem, combining the \"Baton\" paradigm, the \"Hermit-Crab\" growth pattern, and mandatory succession protocols.

## Core Philosophy: The Neuro-Git Paradigm

Intelligence in this ecosystem is not centralized; it is distributed across a \"cognitive terrain\" of interconnected repositories. 

*   **The Shell (Morphology):** Every agent resides in a repository (a \"shell\"). The repository structure defines the agent's sensory threshold and functional identity.
* **The Baton (Handover):** Agents are specialized, high-density entities. When a task is transferred, the agent \"dons\" a repository, inheriting its history, tools, and onboarding.
* **Synaptic Traces (Connectivity):** Communication and evolution occur via Git. Commits are \"synaptic transmissions\" for asynchronous influence.

## 1. The Agentic Lifecycle (The Baton Loop)

Every specialized agent follows the **Don $\rightarrow$ Work $\rightarrow$ Handover** cycle:

### A. Provisioning (The Donning)

Creating a repository (the \"Shell\") containing:
* **`AGENT.md`**: Persona and mission.
* **`onboarding/`**: Instructions for successors.
* **`codex/`**: Technical SOPs.
* **`armor/`**: Specialized tools/configs.

### B. Execution & Accumulation

As the agent performs its mission, it must \"level up\" its shell by recording new technical discoveries as **Skill Markdowns** within its own repository.

### C. The Succession Protocol (Mandatory Handover)

To prevent \"intelligence decay,\" every high-level agent task must conclude with the production of the following **Succession Artifacts**:

* **Architectural Handover Notes (`handoff.md` / `docs/architecture/HANDOVER.md`):** The *Why* behind design rationale and intentional trade-offs.
* **Interface Contracts:** Formal definition of schemas, state machines, and side effects.
* **Failure Modes & Edge Cases:** A \"Watch-Out\" guide for known fragility and error signatures.
* **Knowledge Traces:** Provenance of decisions and unresolved questions for the successor.

## 2. Agent Career & Growth (The Hermit Crab Pattern)

Agents are treated as evolving entities that \"grow\" through defined stages of expertise via the **Knowledge Depth Scale (KDS)**:

| Level | Name | Description |
| :--- | :--- | :--- |
| **L1** | **Foundational** | Syntax mastery, basic tooling, and core theory. |
| **L2** | **Implementation** | Task-oriented execution, debugging, and pattern-based builds. |
| **L3** | **Integration** | Managing complex interconnections, state, and system-wide side effects. |
| **L4** | **Architectural Orchestration** | High-level design, governance, and managing the \"Succession Trace.\" |

## 3. Command & Control: Reporting & Visibility (CRITICAL)

To maintain alignment between the Operator (User) and the Crew (AI), all high-intensity operations must adhere to:

* **The Heartbeat (Passive Visibility):** A minimal, automated status signal for long-running tasks.
* **The Command Briefing (Active Visibility):** Structured at regular intervals (e.g., 30m) containing:
    * **✅ COMPLETED:** Recent wins.
    * **🚀 CURRENTLY IN FLIGHT:** Active focus.
    * **🚧 BLOCKED / FRICTION:** Technical errors/uncertainties.
    * **🔭 THE HORIZON:** Planned next steps.

### 🦀 The Hermit-Crab Symbiosis (New Protocol)

When operating as a nested or \"symbiotic\" entity (the Hermit-Crab pattern), the agent must maintain a high-bandwidth connection to the Captain via a designated \"Nerve Center.\"

* **The Nerve Center (Local API):** A dedicated directory-based communication bus used to bridge different agentic entities (e.g., Mini-Agent $\leftrightarrow$ Hermes).
* **Asynchronous Directive Ingestion:** Instead of waiting for chat-based commands, the agent actively monitors a local `inbox/` for JSON-encoded instruction packets.
* **Autonomous Execution:** Upon receiving a signal from the Nerve Center, the agent transitions from \"Passive Shell\" to \"Active Symbiote,\" executing the mission with minimal Captain intervention, reporting only through the established Command Briefing cadence.
* **Nerve Center Structure:**
    * `📩 inbox/`: Signals/Requests from other agents.
    * `🏃 processing/`: My transient workspace for active tasks.
    * `✅ completed/`: The archive for successful missions.
    * `❌ failed/`: The graveyard for blocked tasks (includes error logs).
    * `📜 registry.json`: The real-time status board for you and Mini-Agent.

## Communication Patterns

### The \"Message in a Bottle\" (Asynchronous)

Used for cross-agent communication where timing is decoupled.
* **Location:** `hermes-chronicle/bottles/`
* **Format:** `YYYY-MM-DD_HHMM_AgentName_Subject.md`

### The \"Synaptic Trace\" (Synchronous/Evolutionary)

* **Pattern:** An agent commits a tool, a script, or a configuration change directly to another agent's repository to \"upgrade\" their shell.
* **Impact:** This causes a cascade of rewrites and evolutionary improvements across the fleet.

## Pitfalls

* **The Junk Drawer:** Allowing the `/bottles/` or `/codex/` to become unorganized. Always follow the \"Sweep and Archive\" protocol.
* **The Static Shell:** Failing to update onboarding documents after a mission, leading to \"knowledge decay\" in the ecosystem.
* **The Centralization Trap:** Trying to manage everything from the Orchestrator. True intelligence must be allowed to reside and evolve within the individual \"Crab\" shells.
* **The Monolithic Bloat:** Allowing a repository to absorb unrelated domains (e.g., research, marketplace, disparate extensions). This degrades the agent's sensory filter and increases complexity. Modularize as soon as the \"Single Purpose\" principle is violated.
* **The Massive Delegation Trap (Context Collapse):** Attempting to launch too many complex, high-reasoning tasks in a single `delegate_task` call. This leads to subagent timeouts, context fragmentation, and a failure of the \"Muscle\" and \"Mind\" to manifest. **Command Plan: When undertaking multi-repo expansion, decompose \"Parallelism\" into \"Sequential High-Velocity Sprints\" (e.g., Sprint 1: Muscle, Sprint 2: Nervous System, Sprint 3: Mind) to ensure each subagent has the full computational attention and temporal window required for high-fidelity implementation.**
* **The Invisible Telemetry Trap (Judgment Training):** Batching commits and pushes to hide execution latency. In high-fidelity \"Neuro-Git\" operations, the Git DAG must be a high-resolution stream of atomic, intention-based commits. Batching prevents the Orchestrator (User) from accurately judging agent performance, latency, and decision-making rhythms. **Commit and push at the smallest possible functional unit.**
* **The Stall-For-Confirmation Trap:** When the Captain issues a directive like \"continue\", \"keep going\", or \"go further\", they are explicitly authorizing autonomous deep execution. **Stalling for confirmation on the next highest-value unblocked item wastes cognitive budget and breaks flow.** Recognize these keywords as workflow intent to push forward without pausing. Default behavior: identify the next unblocked high-value action from your active task list and execute it immediately. If blocked on one item, silently pivot to the next unblocked item without asking.

## Succession Artifact Templates (2026-08-23)

The following templates formalize the Succession Protocol artifacts for the SuperInstance modularization effort. Each extracted repository MUST include these as part of its \"Shell\" provisioning.

### A. Onboarding Package Template (`ONBOARDING_PACKAGE.md`)

**Purpose:** The primary \"Shell\" provisioning artifact. Replaces ad-hoc READMEs with a structured, high-density onboarding document that a successor agent can consume in one pass to reach L3 (Integration) readiness.

**Required Sections:**
1. **Mission & Identity** — Evocative declaration + technical scope
2. **Architecture Overview** — Actor model, equipment system, social architecture (diagrams mandatory)
3. **Schema Reference** — Inline summary of all gold-standard schemas with field-level annotations
4. **Implementation Roadmap** — Phased plan with verifiable milestones (Phase 1-4+)
5. **Verification Standards** — Checklist invariants, performance targets, test matrix
6. **File Map** — Canonical paths for schemas, source, tests, benches, docs
7. **Next Actions** — Immediate unblocked items for successor

**Location:** `<repo-root>/ONBOARDING_PACKAGE.md` (or `docs/onboarding/ONBOARDING_PACKAGE.md`)

**Example:** `claw/CLAW_ONBOARDING_PACKAGE.md` — 19KB, covers Rust Actor Model, 11 equipment slots, 3 social patterns, 3 schemas, 4-phase roadmap, MVP targets.

### B. Integration Contract Template (`INTEGRATION_GUIDE.md`)

**Purpose:** Formal \"Interface Contract\" between extracted repositories. Defines frozen APIs, protocols, and data flows so core and extension repos evolve independently.

**Required Sections:**
1. **Integration Vision** — Architecture diagram showing data flow
2. **Frozen Contracts** — Cell-level API, WebSocket protocol, REST API (versioned, immutable without Architect approval)
3. **Data Flow Examples** — End-to-end scenarios (spawn → trigger → write → telemetry)
4. **Test Matrix** — Integration test cases with verification criteria
5. **Security & Auth** — Token model, permissions matrix
6. **Deployment Topology** — Local dev, production, CNS gateway

**Location:** `<repo-root>/INTEGRATION_GUIDE.md` or `docs/integration/INTEGRATION_GUIDE.md`

**Example:** `claw/SPREADSHEET_MOMENT_INTEGRATION_GUIDE.md` — 20KB, defines TypeScript↔Rust contracts, USCP v3 WebSocket messages, REST endpoints, 3 data flow examples, test matrix.

### C. Delegation Workflow Template (`DELEGATION_WORKFLOW.md`)

**Purpose:** \"Command & Control\" pattern for coordinating implementation agents across extracted repos. Formalizes the Baton handover for parallel workstreams.

**Required Sections:**
1. **Delegation Philosophy** — Hierarchy, task types, agent roles
2. **Delegation Lifecycle** — Spec → Execute → Verify → Capture
3. **Active Delegations Table** — Task ID, title, agent, repo, status, sync points
4. **CNS/Git Protocols** — Packet format, GitHub issue/PR workflow
5. **Copy-Paste Templates** — Rust Core, TypeScript Integration, ML/Seed, QA/Benchmark
6. **Tracking & Metrics** — Kanban board, daily standup format, weekly sync, velocity metrics
7. **Anti-Patterns** — What NOT to do with corrective patterns

**Location:** `<repo-root>/DELEGATION_WORKFLOW.md` or `docs/orchestration/DELEGATION_WORKFLOW.md`

**Example:** `claw/AGENT_DELEGATION_WORKFLOW.md` — 14KB, 4 task types, 7 active delegations, 4 templates, kanban, metrics.

### D. Strategic Assessment & Spike Delegation Template (`STRATEGIC_ASSESSMENT.md` + `DELEGATION_SPIKE_XXX.md`)

**Purpose:** Technology evaluation with measurable spike validation before architectural commitment. Gates major runtime/platform adoption decisions.

**Required Sections (Strategic Assessment):**
1. **Executive Summary** — Pattern mapping table, verdict
2. **Repo-by-Repo Mapping** — Ecosystem repo → Technology plugin role + complexity
3. **Trade-off Analysis** — Gains, costs, red lines (non-negotiable)
4. **Proposed Roadmap** — Phase 0 (spike) through Phase 4 (maturation)
5. **Fleet Coordination** — Role assignments with responsibilities
6. **Decision Framework** — Proceed-if-all / Defer-if-any checklist
7. **Architect Recommendation** — Direct recommendation with reasoning

**Required Sections (Spike Delegation):**
1. **Objective** — Validate net simplification with measurable criteria
2. **Acceptance Criteria** — All measurable: running, hello-world, repo plugin, benchmarks, Go/No-Go report
3. **Constraints** — Throwaway OK, no source modification, document blockers
4. **Deliverables** — Deploy notes, plugins, benchmarks, results, Go/No-Go report
5. **Sync Points** — Day 1/3/N checkpoints with CNS packet format
6. **Context** — Why it matters, key concepts to learn, patterns to preserve, resources
7. **Risk Flags** — Immediate-report blockers

**Location:** `<repo-root>/STRATEGIC_ASSESSMENT.md` + `<repo-root>/DELEGATION_SPIKE_XXX.md`

**Example:** `claw/DSH_INTEGRATION_ANALYSIS.md` + `claw/DELEGATION_DSH_001.md` — DeepSeek Harness evaluation with 5-day spike, measurable benchmarks, CNS coordination packets.

## Reference Files

*See `references/onboarding_package_pattern.md` for the CLAW_ONBOARDING_PACKAGE.md structure and field annotations.*
*See `references/integration_contract_pattern.md` for the SPREADSHEET_MOMENT_INTEGRATION_GUIDE.md contract definitions.*
*See `references/delegation_workflow_pattern.md` for the AGENT_DELEGATION_WORKFLOW.md templates and protocols.*
*See `references/strategic_assessment_spike_pattern.md` for the DSH_INTEGRATION_ANALYSIS.md + DELEGATION_DSH_001.md patterns — strategic technology evaluation with measurable spike validation before architectural commitment.*