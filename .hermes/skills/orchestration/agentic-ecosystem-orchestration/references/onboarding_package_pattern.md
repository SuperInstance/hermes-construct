# Onboarding Package Pattern Reference

> **Canonical structure for `ONBOARDING_PACKAGE.md` — the primary Shell provisioning artifact**
> **Derived from:** `claw/CLAW_ONBOARDING_PACKAGE.md` (2026-08-23)
> **Applies to:** All extracted SuperInstance repositories (claw, constrainttheory, spreadsheet-moment, dodecet-encoder, claw-extensions, constrainttheory-ml-demo)

---

## Required Sections (In Order)

### 1. Mission & Identity
```markdown
# [Repo Name] Onboarding Package

> **[One-line evocative tagline]**
> **Audience:** [Target agent types]
> **Status:** [Design/Implementation/Production]
> **Generated:** [Date] by [Architect Agent]
```

**Must include:**
- High-level purpose (the "Why")
- Technical scope (the "What")
- Target audience (who consumes this)
- Current phase status

### 2. Architecture Overview
```markdown
## 🏗 Architecture Overview

### [Core Model] (e.g., Actor Model)
[Diagram: ASCII or mermaid showing core abstractions]

### [Subsystem 1] (e.g., Equipment System)
[Table: Slot | Purpose | Example Module]

### [Subsystem 2] (e.g., Social Architecture)
[Table: Pattern | Use Case | Topology]
```

**Must include:**
- Visual diagram of core abstractions
- Tables for each major subsystem
- Cross-references to schema fields

### 3. Schema Reference
```markdown
## 📋 Schema Reference (Frozen — Do Not Modify Without Architect)

### [Schema Name] (source: `schemas_gold_standard/[file].json`)
**Size:** [X] bytes | **Fields:** [N] top-level | **$id:** [canonical]

#### Key Structures
| Field | Type | Constraints | Checklist Mapping |
|-------|------|-------------|-------------------|
| `field.path` | Type | Validation rules | CONS-1, MOLT-3 |

#### Critical Invariants
- **INV-1:** [Description] → enforced by `field.path`
- **INV-2:** [Description] → enforced by `field.path`
```

**Must include:**
- All gold-standard schemas summarized
- Field-level table with Checklist mappings
- Explicit "Frozen" warning

### 4. Implementation Roadmap
```markdown
## 🗺 Implementation Roadmap

### Phase 1: [Name] (Week 1-2) — [Goal]
- [ ] Task 1: [Verifiable criterion]
- [ ] Task 2: [Verifiable criterion]
- **Deliverable:** [Artifact path]
- **Verification:** [How to confirm]

### Phase 2: [Name] (Week 2-3) — [Goal]
...
```

**Must include:**
- Phased with week estimates
- Checkbox tasks with verifiable criteria
- Explicit deliverables and verification methods

### 5. Verification Standards
```markdown
## ✅ Verification Standards

### Schema Compliance
- All structs/enums match schema exactly (serde roundtrip test passes)
- Validation function returns errors for all CONS-1..3, MOLT-1..6, G1, DRM-1

### Performance Targets (from MVP_DEFINITION.md)
| Metric | Target | Measurement |
|--------|--------|-------------|
| Agent creation | p99 < 10ms | `cargo bench` |
| Trigger response | p99 < 100ms | `cargo bench` |
| Spatial query | p99 < 100μs | `cargo bench` |

### Test Requirements
- Unit: 90%+ coverage on new code
- Integration: 80%+ coverage
- Chaos: Network partition, OOM, trigger storm — all recover
```

**Must include:**
- Schema compliance criteria
- Performance targets with measurement method
- Test coverage requirements

### 6. File Map
```markdown
## 📁 File Map (Canonical Paths)

| Category | Path | Purpose |
|----------|------|---------|
| Schemas | `schemas_gold_standard/*.json` | Frozen contracts |
| Core | `src/[crate]/` | Implementation |
| Tests | `tests/*.rs` | Verification |
| Benches | `benches/*.rs` | Performance |
| Docs | `docs/*.md` | Architecture |
```

**Must include:**
- Canonical paths for all artifact types
- No ambiguity about where things live

### 7. Next Actions
```markdown
## 🎯 Next Actions (Immediate Unblocked Items)

1. **[Task]** — [Repo] — [Agent Type] — [Dependency: None]
2. **[Task]** — [Repo] — [Agent Type] — [Dependency: Task 1]
```

**Must include:**
- Prioritized, unblocked items
- Clear ownership and dependencies

---

## Quality Gates

Before declaring an Onboarding Package complete:
- [ ] All 7 sections present and complete
- [ ] Schema reference covers 100% of gold-standard schemas
- [ ] Every Checklist invariant mapped to at least one schema field
- [ ] Performance targets match MVP_DEFINITION.md
- [ ] File map matches actual repository structure
- [ ] Next actions are truly unblocked (no hidden dependencies)
- [ ] Document lints clean (markdownlint, no broken links)

---

## Anti-Patterns

| Anti-Pattern | Symptom | Fix |
|--------------|---------|-----|
| **README Drift** | `README.md` and `ONBOARDING_PACKAGE.md` diverge | Single source: `ONBOARDING_PACKAGE.md` IS the README |
| **Schema Drift** | Schema reference doesn't match actual JSON | Automate: generate reference table from schema in CI |
| **Vague Milestones** | "Implement actor system" (not verifiable) | "Actor loop processes 1000 messages/sec, p99 < 10ms" |
| **Missing Checklist Mapping** | Fields exist but no CONS/MOLT/DRM mapping | Every field must trace to a Checklist invariant |
| **Stale Next Actions** | Items blocked but listed as unblocked | Weekly review: move blocked items to "Blocked" section |