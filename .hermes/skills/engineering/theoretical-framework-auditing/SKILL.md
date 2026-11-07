---
name: theoretical-framework-auditing
description: Governs the process of auditing code, schemas, and implementations against mathematical/theoretical frameworks. Used when a project defines formal invariants (conservation laws, ODEs, algebraic constraints) and needs to verify that implementations respect them. Covers Verification Checklist and Gap Matrix production.
---

# Theoretical Framework Auditing

This skill governs the process of auditing code, schemas, and implementations against mathematical/theoretical frameworks.

## When to Load

Load this skill when:
- The user asks to "audit," "verify," or "check" code against theoretical papers or mathematical frameworks
- A project has formal invariants (conservation laws, ODEs, algebraic constraints, geometric properties)
- Implementing papers that define equations, theorems, or formal systems
- Cross-referencing local code against research specifications
- Auditing scripts for credential/path invariants (use the script-invariant-audit-template reference)

## Core Pattern: Two-Artifact Audit

Every theoretical audit produces two standard artifacts:

### 1. Verification Checklist
A structured document mapping theoretical invariants to code-level checks.

**Structure:**
- **Invariants Section:** List every equation, bound, and constraint from the theory. For each, specify:
  - Paper/theorem reference
  - Exact mathematical form
  - Code-level check (what to look for in implementation)
  - Pass/fail test
  
- **Failure Modes Section:** Concrete "code smell" descriptions for common implementation drifts

- **Equation-Level Checks:** Exact term-by-term requirements (e.g., "denominator must be exactly 0.02, not 0.0145 or 0.02 rounded")

- **Parameter Bounds:** Edge-case boundaries that must be enforced in code

- **Top-5 Priorities:** Highest-risk silent failures ranked by theoretical impact

### 2. Gap Matrix
A structured document mapping theoretical gaps to operational capabilities.

**Structure:**
| Gap | Paper(s) | Severity | Operational Capability | Confidence Metric | Implementation Action |

Each row:
- **Gap:** Verbatim description of missing/weak theoretical element
- **Paper(s):** Source
- **Severity:** HIGH / MEDIUM / LOW-MEDIUM
- **Operational Capability:** Named sensor/audit function
- **Confidence Metric:** Numerical quantity to monitor
- **Implementation Action:** Concrete step executable now

## Workflow

### Phase 1: Theory Extraction
1. Read all relevant theoretical papers
2. Extract every formal theorem, equation, invariant, and failure mode
3. Note what the theory says "must hold" vs. "is proposed" vs. "is left open"

### Phase 2: Code Mapping
1. For each invariant, identify the code surface where it should be enforced
2. Determine if the implementation matches the theory exactly or approximates it
3. Flag approximations that could violate theoretical guarantees

### Phase 3: Gap Analysis
1. Identify what the theory leaves unexplored
2. Map each gap to a concrete monitoring/testing capability
3. Prioritize by theoretical impact × implementation feasibility

## Key Principles

- **Exact Forms Matter:** A denominator of `0.02` is not the same as `0.0145` or `2*0.085**2` rounded. Capture exact forms from papers.
- **Failure Modes Are Theory Too:** Papers that don't explicitly state failure modes still imply them. Derive them from edge cases and boundary conditions.
- **Severity Tiers:** HIGH = violates core guarantee; MEDIUM = weakens guarantees; LOW-MEDIUM = aesthetic/non-essential
- **Implementation Is Not Theory:** Code often approximates math. Document every approximation and its risk.

## Standard Artifacts

Every audit MUST produce at minimum:
1. **Verification Checklist** (see Core Pattern above)
2. **Gap Matrix** (see Core Pattern above)
3. **Verification Checklist Runner** (NEW — executable script, not just prose):
   - The Checklist is a static document; a runner makes it executable.
   - Implement a standalone script (Python preferred) that accepts a telemetry/state JSON file and evaluates every rule.
   - Script must: (a) load JSON, (b) evaluate each rule with pass/fail, (c) print per-agent results, (d) exit 0=all pass, 1=failures.
   - Deploy the runner as a cron-driven gate: every N minutes, run against live telemetry and alert on FAIL.
   - Reference implementation: `scripts/verification_checklist_runner.py` in the `autoclaw` repo (commit `46bab66`).

## Critical Pattern: Smoke Test Pattern (NEW)

Before declaring integration success with synthetic_telemetry:

1.  **Reset state files:** Clear `agent_history` and truncate `processed` list in telemetry/monitor state JSON to prevent cross-test contamination.
2.  **Inject synthetic packet:** Write a USCP-v3 packet to `~/.hermes/cns_inbox/` with known values that should PASS (e.g., `deterministic=true`, `T=0.0`, `γ+η+δ=1.0`).
3.  **Run the monitor:** Load `cns_monitor_v3_telemetry.py` via `importlib.util.spec_from_file_location` and call `process_packet()`. (Do NOT rely on `python3 cns_monitor_v3_telemetry.py` — it may not have a `main` block and can hang.)
4.  **Assert alert surface:** Verify the expected alerts appear (or don't appear) in `telemetry_state["agent_history"][aid]["alerts"]`.
5.  **Negative test:** Run a second synthetic packet with a deliberate violation (e.g., `deterministic=true`, `T=1.0`) and confirm the alert fires.
6.  **Run the Checklist runner:** `scripts/verification_checklist_runner.py` must exit 0=all pass after the positive test.

If step 5 (negative test) does NOT fail, the alert logic is broken — fix it before reporting success.

## Critical Pattern: Schema Stub Detection

When auditing a codebase that validates against "gold standard" or "canonical" schemas:
1. `stat` each schema file — if byte count < 200 (e.g., `{"title": "...", "type": "object"}`), it is a stub.
2. Stubs cause all "schema validation" to pass vacuously — this is a HIGH-severity finding.
3. Deliver a **Schema Population Plan** as part of the audit output.
4. Populate stubs only after explicit user approval (or if the plan itself was the directive).

## Pitfalls

- **Don't assume code matches theory:** The theory says what *should* happen; the code does what *does* happen.
- **Don't skip the "unexplored" sections:** Papers explicitly identify their own gaps. These are high-value targets.
- **Don't conflate different conservation laws:** Papers may use similar terminology for different quantities.
- **Don't accept "close enough":** Capturing exact forms (coefficients, denominators, exponents) is the whole point.
- **Don't ignore stub schemas:** An empty schema stub is not a schema — it's a placeholder. Detection requires byte-count + content inspection, not just file existence.
- **Don't deliver analysis without action artifacts:** Audits that produce only prose reports fail the fleet. Every audit must produce machine-readable artifacts (checklist, matrix, plan) that agents can act on.
- **Cross-skill count drift:** When new rules are added to the verification_checklist_runner.py RULES dict, the expected PASS count in dependent skills must be updated immediately. Stale counts cause false FAIL reports. Treat the runner's rule count as the source of truth.
- **Runtime invariant extension:** When theoretical frameworks evolve (e.g., adding G1-DETERMINISM, G4-COMPOSITION), extend the verification runner AND update every skill that references the old expected pass count. This session pattern: new rule → runner update → 13/13 PASS → push → update dependent skills.
- **Runtime enforcement pattern (post-checklist → live invariant):** A checklist is a static document, but the fleet runs in production. When an audit produces invariants (CONS-1, DRM, MOLT, etc.), do not stop at the runner. **Wire the same logic into the production egress path (before emit) and the ingress path (before quantum construction).** This session pattern: daemon `_validate_telemetry()` repairs CONS-1/2/DRM/MOLT-4 before emitting USCP-v3; CNS monitor `parse_telemetry_from_packet()` repairs γ/η/δ at ingress. The checklist catches regressions; runtime enforcement prevents bad packets from ever leaving or entering the fleet.

## Reference Files

- `references/superinstance-papers-p56-60.md` — condensed P56–P60 theorem bank for audit queries
- `references/script-invariant-audit-template.md` — invariant table, audit output format, and patch requirements for credential/path script audits
