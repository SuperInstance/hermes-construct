# Strategic Assessment & Spike Delegation Pattern Reference

> **Canonical structure for technology evaluation + spike-based validation before architectural commitment**
> **Derived from:** `claw/DSH_INTEGRATION_ANALYSIS.md` + `claw/DELEGATION_DSH_001.md` (2026-08-23)
> **Applies to:** All major technology adoption decisions in SuperInstance ecosystem (DSH, new runtimes, cloud platforms, etc.)

---

## Required Sections (In Order)

### 1. Strategic Assessment Document (`STRATEGIC_ASSESSMENT.md`)

```markdown
# [Technology] Integration — Evolutionary Assessment

> **Assessment of [Technology] as [Role] for [Ecosystem]**
> **Origin:** [Source of proposal]
> **Status:** [Under Review / Spike Initiated / Committed / Rejected]
> **Generated:** [Date] by [Architect Agent]

---

## Executive Summary

| [Ecosystem] Pattern | [Technology] Equivalent | Repos Cited |
|---------------------|------------------------|-------------|
| Pattern 1 | Equivalent 1 | repo1, repo2 |
| Pattern 2 | Equivalent 2 | repo3, repo4 |

**Verdict:** [Fit assessment] — The mapping is structurally sound. The question is **strategic timing and migration cost** — not architectural fit.

---

## Repo-by-Repo Mapping

| [Ecosystem] Repo | [Technology] Plugin Role | Migration Complexity |
|------------------|-------------------------|---------------------|
| repo1 | Role 1 | Low/Medium/High — [reason] |
| repo2 | Role 2 | Low/Medium/High — [reason] |

---

## Trade-off Analysis

### Evolutionary Gains

| Gain | Impact |
|------|--------|
| Gain 1 | Impact description |
| Gain 2 | Impact description |

### Migration Costs

| Cost | Mitigation |
|------|------------|
| Cost 1 | Mitigation strategy |
| Cost 2 | Mitigation strategy |

### Red Lines (Non-Negotiable)

1. **Red Line 1** — [Description]
2. **Red Line 2** — [Description]
3. **Red Line 3** — [Description]

---

## Proposed Roadmap

### Phase 0: Spike (N weeks) — Validate Fit
- [ ] Deploy [Technology] locally, run hello-world
- [ ] Port simplest repo as plugin
- [ ] Benchmark: [key metrics]
- [ ] **Go/No-Go:** [Decision criteria]

### Phase 1: Core Runtime (N weeks) — Foundation
- [ ] Core repo → plugin with compatibility layer
- [ ] Communication bridge (existing ↔ new)
- [ ] Schema validation in plugin manifest
- [ ] **Milestone:** [Concrete demo]

### Phase 2: Ecosystem Integration (N weeks) — Unification
- [ ] Supporting repos → plugins
- [ ] Multi-agent fleet runs in new runtime
- [ ] **Milestone:** [Concrete demo]

### Phase 3: Platform (N weeks) — Expansion
- [ ] Complex repo → plugin (hardest)
- [ ] GPU/compute integration
- [ ] UI/dashboard plugins
- [ ] **Milestone:** Full parity

### Phase 4: Maturation (Ongoing) — Optimization
- [ ] Performance tuning
- [ ] Plugin marketplace
- [ ] Cross-fleet sharing
- [ ] Advanced features

---

## Fleet Coordination Required

| Role | Responsibility |
|------|----------------|
| First Officer | Go/No-Go authority, resource allocation |
| Navigation | Spatial reasoning, migration tooling |
| Perception (Schema Architect) | Schema contracts, telemetry bridge, verification |
| Ensign | Learning, documentation, testing |

---

## Decision Framework

### Proceed If (All Must Be True)
- [ ] Phase 0 spike shows **net simplification** (not just parity)
- [ ] Core model runs **without semantic compromise**
- [ ] Existing telemetry bridges **losslessly**
- [ ] Complex repo has **viable migration path** (not rewrite)
- [ ] **N+ engineers** can dedicate **X%** for **Y weeks**

### Defer If (Any True)
- [ ] Current phase **>50% complete**
- [ ] Technology lacks **critical feature** (list)
- [ ] Migration blocks operations **>N weeks**
- [ ] Strategic value **unclear** vs. independent evolution

---

## Architect Recommendation

> **[Clear recommendation with reasoning]**

---

## Quality Gates

Before declaring Strategic Assessment complete:
- [ ] All repo mappings accurate and complete
- [ ] Trade-offs quantified where possible
- [ ] Red lines explicitly stated and non-negotiable
- [ ] Roadmap has concrete milestones with verification
- [ ] Decision framework is actionable (checklist)
- [ ] Fleet roles assigned with clear responsibilities
- [ ] Recommendation directly addresses decision framework
```

---

### 2. Spike Delegation Document (`DELEGATION_SPIKE_XXX.md`)

```markdown
# DELEGATION: SPIKE-XXX — Phase 0 Spike: [Technology] Local Deploy + [Simplest Repo] Plugin

**Repo:** [Technology] (external) + [simplest-repo] ([Ecosystem])
**Agent:** [Agent Type] (e.g., Navigation / Spatial Reasoning)
**Priority:** P0 — Blocking Go/No-Go for [Technology] integration
**Timeline:** N days (Start → End)
**Reference:** STRATEGIC_ASSESSMENT.md §Phase 0

---

## Objective

Validate that [Technology] can host [Ecosystem] plugins with **net simplification** over current architecture. This is a spike — throwaway code acceptable, but results must be measurable.

---

## Acceptance Criteria (All Must Pass)

- [ ] **[Technology] Running Locally**: `[cmd]` starts without errors, dashboard accessible
- [ ] **Hello-World Plugin**: Custom plugin loads, hot-reload works, logs visible
- [ ] **[Simplest Repo] as [Technology] Plugin**:
  - [ ] Module 1 exposed as plugin capability
  - [ ] Module 2 callable via plugin API
  - [ ] Module 3 callable via plugin API
  - [ ] Plugin manifest (`plugin.toml` or equivalent) declares dependencies correctly
- [ ] **Benchmark Results** (measured, not estimated):
  - [ ] Plugin load time: _____ ms
  - [ ] Inter-plugin call overhead (vs. direct fn call): _____ μs
  - [ ] Memory overhead per plugin: _____ MB
  - [ ] Cold start → ready: _____ ms
- [ ] **Schema Compatibility**: `[simplest-repo]` schemas validate in [Technology] manifest system
- [ ] **Go/No-Go Report**: Written assessment with numbers, recommendation

---

## Constraints

- **Throwaway code OK** — this is a spike, not production
- **No [Technology] source modification** — use plugin API only
- **Document blockers** — any [Technology] limitation that prevents [Ecosystem] patterns
- **Focus on [simplest-repo] first** — [reason: pure Rust, no deps, etc.]
- **[Second repo] second** — if time permits

---

## Deliverables

| Artifact | Location | Format |
|----------|----------|--------|
| Deploy notes | `spike/DEPLOY.md` | Markdown |
| Hello-world plugin | `spike/plugins/hello/` | Plugin structure |
| Repo plugin | `spike/plugins/[repo]/` | Plugin structure |
| Benchmark script | `spike/benchmarks/` | [Language] + criterion |
| Benchmark results | `spike/RESULTS.md` | Markdown + charts |
| Go/No-Go report | `spike/GO_NOGO.md` | Markdown (decision document) |

---

## Sync Points

| Day | Checkpoint | Format |
|-----|------------|--------|
| Day 1 | [Technology] running, hello-world plugin loaded | CNS packet to [officer] + [architect] |
| Day 3 | Repo plugin compiles, basic calls work | CNS packet + interim RESULTS.md |
| Day N | Full benchmarks complete, Go/No-Go report | CNS packet + GO_NOGO.md + RESULTS.md |

---

## Context for Agent

**Why this matters:** The fleet is evaluating [Technology] as unified runtime. This spike determines whether we pivot (Phase 1-N) or continue independent evolution. Your [agent skills] are critical — [Technology]'s [key concept] must map to our [core pattern].

**Key [Technology] Concepts to Learn:**
- Plugin manifest format — capabilities, dependencies, exports
- Build system — hermetic, reproducible
- Runtime model — [key concepts]
- Message bus — inter-plugin communication
- State persistence — checkpointing, migration

**[Ecosystem] Patterns to Preserve:**
- Pattern 1 (details)
- Pattern 2 (details)
- Schema validation (our schemas are contracts)
- Existing telemetry (list extensions)

**Resources:**
- [Technology] docs: [URL]
- [simplest-repo]: [path] (needs clone) or [alt path]
- STRATEGIC_ASSESSMENT.md: [path]

---

## Risk Flags (Report Immediately)

- ❌ [Technology] plugin API doesn't support [our language] plugins
- ❌ Plugin hot-reload broken or missing
- ❌ Inter-plugin call overhead > [threshold] μs (unacceptable for [use case])
- ❌ No schema validation in manifest system
- ❌ Runtime model incompatible with our [core pattern]
- ❌ [Technology] requires runtime license / not open source
- ❌ GPU/compute plugin model doesn't exist (blocks [GPU repo])
```

---

### 3. CNS Packet Templates for Strategic Coordination

#### Strategic Proposal (Architect → First Officer)

```json
{
  "header": {
    "origin_id": "hermes-cns",
    "timestamp": "ISO8601",
    "priority": "HIGH",
    "destination_id": "lucineer-main",
    "correlation_id: "strategic-[topic]-YYYYMMDD"
  },
  "body": {
    "intent": "STRATEGIC_PROPOSAL",
    "payload": {
      "title": "[Technology] Integration — Evolutionary Assessment",
      "summary": "[One-line summary]",
      "analysis_file": "[path]",
      "key_findings": {
        "evolutionary_fit": true/false,
        "repo_mapping_accurate": true/false,
        "critical_risks": ["risk1", "risk2", "risk3"],
        "red_lines_non_negotiable": ["redline1", "redline2", "redline3"]
      },
      "proposed_roadmap": {
        "phase_0_spike_Nweeks": "[Description]",
        "phase_1_core_Nweeks": "[Description]",
        "phase_2_ecosystem_Nweeks": "[Description]",
        "phase_3_platform_Nweeks": "[Description]",
        "phase_4_maturation": "[Description]"
      },
      "decision_framework": {
        "proceed_if_all_true": ["criteria1", "criteria2", "criteria3"],
        "defer_if_any_true": ["criteria1", "criteria2", "criteria3"]
      },
      "architect_recommendation": "[Recommendation]",
      "required_actions": {
        "first_officer": "Go/No-Go by [date]",
        "navigation": "[Task] by [date]",
        "architect": "[Task] by [date]",
        "ensign": "[Task] by [date]"
      }
    }
  },
  "signature": {
    "type": "USCP-v3",
    "version": "3.0",
    "telemetry_extensions": ["gamma_eta", "thermal", "creative", "melt", "molt", "uncertainty", "anomaly"]
  }
}
```

#### Spike Delegation (Architect → Navigation/Spike Agent)

```json
{
  "header": {
    "origin_id": "hermes-cns",
    "timestamp": "ISO8601",
    "priority": "HIGH",
    "destination_id": "[agent-id]",
    "correlation_id": "delegation-SPIKE-XXX-YYYYMMDD"
  },
  "body": {
    "intent": "DELEGATE_TASK",
    "payload": {
      "task_id": "SPIKE-XXX",
      "title": "Phase 0 Spike: [Technology] Local Deploy + [Repo] Plugin",
      "spec_file": "[path]",
      "agent_type": "[Agent Type]",
      "repo": "[Technology] (external) + [repo]",
      "timeline_days": N,
      "sync_points": [
        "Day 1: [Technology] running + hello-world plugin",
        "Day 3: [Repo] plugin compiles + basic calls",
        "Day N: Benchmarks complete + Go/No-Go report"
      ],
      "acceptance_criteria": [
        "Criterion 1",
        "Criterion 2"
      ],
      "risk_flags": [
        "Risk 1",
        "Risk 2"
      ]
    }
  },
  "signature": {
    "type": "USCP-v3",
    "version": "3.0",
    "telemetry_extensions": ["gamma_eta", "thermal", "creative", "melt", "molt", "uncertainty", "anomaly"]
  }
}
```

---

## Quality Gates

Before declaring Strategic Assessment + Spike Delegation complete:
- [ ] Assessment covers all ecosystem repos with accurate mappings
- [ ] Trade-off analysis has quantified gains/costs where possible
- [ ] Red lines are explicit and non-negotiable
- [ ] Roadmap has 4+ phases with concrete milestones
- [ ] Decision framework is a checklist (proceed if all / defer if any)
- [ ] Fleet roles assigned with clear responsibilities and deadlines
- [ ] Spike delegation has measurable acceptance criteria (benchmarks)
- [ ] Spike delegation has explicit risk flags for immediate reporting
- [ ] CNS packet templates work with actual fleet IDs
- [ ] Go/No-Go deadline set and communicated

---

## Anti-Patterns

| Anti-Pattern | Symptom | Fix |
|--------------|---------|-----|
| **Assessment Theater** | Document created but no spike run | Spike delegation created same day as assessment; Go/No-Go deadline set |
| **Vague Benchmarks** | "Performance is good" | Spike requires measured numbers: load time, call overhead, memory, cold start |
| **Missing Red Lines** | Migration proceeds but breaks core patterns | Red lines section mandatory; each red line = architectural invariant |
| **Solo Decision** | Architect decides without fleet input | Decision framework requires First Officer Go/No-Go; Navigation validates technical fit |
| **Spike Scope Creep** | Spike builds production features | Constraints: "Throwaway code OK", "No source modification", "Focus on simplest repo" |
| **Decision Drift** | Go/No-Go delayed indefinitely | Deadline in CNS packet; if missed = automatic Defer |
| **Telemetry Blindness** | New runtime loses CNS/USCP telemetry | Red line: "Existing telemetry bridges losslessly" with specific extensions listed |

---

## Integration with Existing Patterns

This pattern **complements** (does not replace) the existing delegation workflow:

| Existing Pattern | New Pattern | Relationship |
|------------------|-------------|--------------|
| `DELEGATION_WORKFLOW.md` (implementation) | `DELEGATION_SPIKE_XXX.md` (validation) | Spike runs BEFORE implementation delegations; Go/No-Go gates Phase 1+ |
| `INTEGRATION_GUIDE.md` (contracts) | `STRATEGIC_ASSESSMENT.md` (fit) | Assessment validates contracts CAN be implemented in new runtime |
| `ONBOARDING_PACKAGE.md` (repo shell) | Spike results → updated onboarding | Successful spike → new runtime patterns added to onboarding |

**Workflow:**
1. Strategic Assessment → identifies fit + risks + roadmap
2. Spike Delegation → validates critical risks with measurements
3. Go/No-Go → if Go, launch implementation delegations (CL-XXX, SM-XXX, etc.)
4. Implementation → uses existing delegation workflow templates
5. Knowledge Capture → spike results + implementation patterns → updated onboarding + skills