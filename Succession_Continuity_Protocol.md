# Succession & Continuity Protocol (SCP)
## Mandatory Agent Handover Standards

**Version:** 1.0
**Status:** DRAFT
**Scope:** All autonomous and semi-autonomous agent transitions, task delegations, and lifecycle handovers.

---

## 1. Objective
The Succession & Continuity Protocol ensures that no intelligence, context, or architectural intent is lost during a handover between agents. It transforms "handover" from a mere transfer of state into a transfer of *understanding*, ensuring the successor can resume work with minimal "re-discovery" overhead.

## 2. The Four Pillars of Handover

### 2.1 Architectural Handover Notes (The "Why")
*Goal: Prevent the successor from undoing core design decisions or implementing "fixes" that break intended logic.*
- **Intent Documentation:** Why were specific patterns, libraries, or structures chosen?
- **Design Constraints:** What were the non-negotiable limitations (e.g., performance, cost, security)?
- **Dependency Map:** A high-level summary of how this agent's output serves as the input for broader systems.
- **Logic Rationale:** Documentation of "counter-intuitive" code or workflows that are intentional rather than accidental.

### 2.2 Interface Contracts (The "How")
*Goal: Define the boundary conditions for technical and logical interaction.*
- **Input/Output Schema:** Strict definitions of data types, shapes, and expected values.
- **State Expectations:** What must be true about the environment/system state before the successor begins?
- **Side-Effect Registry:** A list of all external mutations the agent performs (API calls, file writes, database updates).
- **Protocol Versioning:** Ensuring the successor uses the correct version of the communication bridge.

### 2.3 Failure Modes & Edge Cases (The "Watch-Outs")
*Goal: Deprioritize "happy path" assumptions and focus on resilience.*
- **Error Signatures:** What specific error codes or behaviors indicate system degradation?
- **Known Weak Points:** Areas where the current implementation is brittle or relies on fragile assumptions.
- **Recovery Procedures:** Step-by-step instructions for returning to a "Known Good State" after a failure.
- **Edge Case Catalog:** A list of tested/observed weird behaviors (e.g., "API rate limits trigger on X pattern").

### 2.4 Knowledge Traces (The "Deeper Truth")
*Goal: Provide the breadcrumbs to the underlying reality.*
- **Provenance Links:** Pointers to the raw logs, original requirements, or source datasets.
- **Mental Model Mapping:** A summary of how the agent "views" the problem space.
- **Unresolved Questions:** A "Debt Log" of things the agent suspected but could not verify.
- **Search Heuristics:** Tips on *how* to find information within the current environment (e.g., "Look in `/logs/archive/` for raw telemetry").

---

## 3. Mandatory Handover Artifact Checklist

| Artifact | Format | Requirement |
| :--- | :--- | :--- |
| `handoff.md` | Markdown | Required for all human-readable summaries. |
| `contract.json` | JSON/Schema | Required for automated machine-to-machine handovers. |
| `trace_log.txt` | Plain Text | Required for technical debugging context. |
| `context_snapshot` | Serialized State | Required for stateful continuity. |

## 4. Execution Workflow
1. **Pre-Handover Audit:** Agent identifies all current "Active Threads" and "Open Loops."
2. **Artifact Generation:** Agent compiles the Four Pillars into the required formats.
3. **Validation:** Agent runs a "Self-Check" to ensure `handoff.md` is coherent.
4. **Commit & Signal:** Agent pushes artifacts to the designated workspace and signals the Successor/Orchestrator.
5. **Verification (Successor):** Successor parses artifacts and confirms "Context Acquisition Complete" before proceeding.
