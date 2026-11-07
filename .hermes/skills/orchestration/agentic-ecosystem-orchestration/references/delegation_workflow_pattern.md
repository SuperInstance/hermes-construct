# Delegation Workflow Pattern Reference

> **Canonical structure for `DELEGATION_WORKFLOW.md` — the Command & Control pattern for coordinating implementation agents**
> **Derived from:** `claw/AGENT_DELEGATION_WORKFLOW.md` (2026-08-23)
> **Applies to:** All multi-agent implementation efforts in SuperInstance ecosystem

---

## Required Sections (In Order)

### 1. Delegation Philosophy
```markdown
# Agent Delegation Workflow

> **[One-line philosophy statement]**
> **Audience:** [Schema Architect, Implementation Agents, Fleet Officers]
> **Status:** [Operational/Design]
> **Generated:** [Date] by [Schema Architect]
```

**Must include:**
- Visual hierarchy diagram (Architect → Agent types)
- Clear statement of Architect role (what/why) vs Agent role (how)
- Task type taxonomy

### 2. Delegation Lifecycle
```markdown
## 🔄 Delegation Lifecycle

### 1. Task Specification (Architect → Agent)
```markdown
## DELEGATION: [TASK-ID] — [Title]

**Repo:** [repo]
**Agent Type:** [Rust/TS/ML/QA Engineer]
**Priority:** [P0/P1/P2]
**Schema Reference:** [file + section]
**Contract Reference:** [guide + section]

### Acceptance Criteria (Verifiable)
- [ ] Criterion 1: measurable, testable
- [ ] Criterion 2: measurable, testable

### Constraints
- Must pass schema validation
- Must meet performance target: <Xms
- Must not modify locked schemas

### Deliverables
- Code: [path/files]
- Tests: [path/files] with coverage %
- Benchmarks: [results]
- Documentation: [updates needed]

### Sync Points
- Day 1: Architecture review (15 min)
- Day 3: Midpoint check (async)
- Day 5: Demo + handoff (30 min)
```

### 2. Agent Execution (Autonomous)
- Works in repo/branch `feature/task-{id}`
- Commits locally, runs tests + schema validation
- Updates task status in shared tracker

### 3. Verification (Architect → QA)
- Architect reviews PR against acceptance criteria
- QA runs integration tests + benchmarks
- **PASS:** Merge to main, update onboarding
- **FAIL:** Return with specific violations

### 4. Knowledge Capture
- Agent writes implementation notes
- Architect updates onboarding package with patterns
- New pitfalls → skills for future agents
```

**Must include:**
- All 4 phases with concrete steps
- Verifiable acceptance criteria template
- Explicit sync points (Day 1/3/5)
- Knowledge capture requirement

### 3. Active Delegations Table
```markdown
## 📦 Current Active Delegations (Phase N)

| Task ID | Title | Agent | Repo | Status | Sync |
|---------|-------|-------|------|--------|------|
| CL-001 | [Component] | Rust Eng #1 | claw | 🔄 In Progress | Day 3 |
| SM-001 | [Component] | TS Eng #1 | spreadsheet-moment | 🔄 In Progress | Day 3 |
```

**Must include:**
- All active tasks with ID, title, agent, repo, status, next sync
- Updated at each sync point

### 4. CNS/Git Protocols
```markdown
## 🛠 Delegation Tools & Protocols

### CNS Packet Delegation (Fleet-Native)
```json
{
  "header": {
    "origin_id": "hermes-cns",
    "destination_id": "rust-engineer-1",
    "correlation_id": "delegation-CL-001",
    "priority": "HIGH"
  },
  "body": {
    "intent": "DELEGATE_TASK",
    "payload": {
      "task_id": "CL-001",
      "spec_markdown": "...",
      "repo": "claw",
      "branch": "feature/task-CL-001"
    }
  }
}
```

### Git-Based Delegation (Current)
```bash
# Architect creates task issue
gh issue create --repo SuperInstance/claw \
  --title "[DELEGATION] CL-001: [Title]" \
  --body-file delegation-CL-001.md \
  --label "delegation,rust,phase-2"

# Agent claims task
gh issue comment --repo SuperInstance/claw 123 --body "Claimed. Starting Day 1."

# Agent pushes work
git push origin feature/task-CL-001

# Architect reviews
gh pr review --repo SuperInstance/claw 456 --approve --body "PASS: All criteria met."
```

**Must include:**
- CNS packet format for fleet-native delegation
- GitHub CLI commands for issue/PR workflow
- Label conventions

### 5. Copy-Paste Templates

#### Template A: Rust Core Implementation
```markdown
## DELEGATION: CL-XXX — [Component Name]

**Repo:** claw
**Agent:** Rust Engineer
**Schema:** schemas_gold_standard/claw_agent.json §[X.Y]

### Acceptance Criteria
- [ ] Structs/enums match schema exactly (serde roundtrip test passes)
- [ ] Validation function returns errors for all CONS-1..3, MOLT-1..6, G1, DRM-1
- [ ] Unit tests: 90%+ coverage on new code
- [ ] `cargo test` passes, `cargo clippy` clean
- [ ] Benchmark: [specific metric] < [target]

### Files to Create/Modify
- claw-core/src/[component].rs
- claw-core/tests/[component]_test.rs
- claw-core/benches/[component]_bench.rs

### Constraints
- No external deps beyond: serde, schemars, uuid, tokio, thiserror
- Must compile with `--cfg feature="schema-validation"`
- No `unwrap()`/`expect()` in hot paths — use `Result`
```

#### Template B: TypeScript Integration
```markdown
## DELEGATION: SM-XXX — [Component Name]

**Repo:** spreadsheet-moment
**Agent:** TypeScript Engineer
**Contract:** SPREADSHEET_MOMENT_INTEGRATION_GUIDE.md §[X.Y]

### Acceptance Criteria
- [ ] Component integrates with Univer cell system (no console errors)
- [ ] WebSocket connects to claw-engine:8080, handles reconnection
- [ ] Real-time telemetry updates cell badge < 100ms latency
- [ ] Formula `=CLAW(...)` parses and spawns claw
- [ ] Unit tests: 80%+ coverage, Vitest passes

### Files to Create/Modify
- packages/sheets/src/cell-types/ClawCell.ts
- packages/sheets/src/services/ClawWebSocketManager.ts
- packages/sheets/src/ui/ClawConfigModal.tsx
- packages/sheets/tests/claw-integration.test.ts

### Constraints
- Follow existing Univer plugin patterns
- Use `@univerjs/core` APIs only
- No direct DOM manipulation — use React components
- TypeScript strict mode, no `any`
```

#### Template C: ML/Seed Engineering
```markdown
## DELEGATION: ML-XXX — [Component Name]

**Repo:** claw (seed-engine crate)
**Agent:** ML Engineer
**Schema:** schemas_gold_standard/claw_seed.json

### Acceptance Criteria
- [ ] SeedCompiler: natural language → prompt template (deterministic)
- [ ] TrainingInterface trait implemented for [reinforcement|replay|distillation]
- [ ] Default equipment auto-equip on spawn verified
- [ ] Creative zone monitoring (Δ tracking) functional
- [ ] Integration test: seed → spawn → trigger → learn → stabilize

### Files to Create/Modify
- claw-seed/src/compiler.rs
- claw-seed/src/training.rs
- claw-seed/src/creative_zone.rs
- claw-seed/tests/seed_lifecycle_test.rs

### Constraints
- Use `candle` or `burn` for ML ops (no PyTorch/TensorFlow deps)
- Deterministic compilation: same seed + same input = same prompt
- Training interface must be async, cancellable
```

#### Template D: QA/Benchmark
```markdown
## DELEGATION: QA-XXX — [Benchmark/Test Suite]

**Repo:** claw / spreadsheet-moment
**Agent:** QA Engineer
**Target:** [Performance target from MVP_DEFINITION.md]

### Acceptance Criteria
- [ ] Benchmark harness runs `cargo bench` / `pnpm test:bench`
- [ ] Agent creation: p99 < 10ms (1000 iterations)
- [ ] Trigger response: p99 < 100ms (1000 iterations)
- [ ] Spatial query: p99 < 100μs (10000 iterations)
- [ ] Concurrent 100 agents: < 500ms total startup
- [ ] Chaos test: network partition, OOM, trigger storm — all recover
- [ ] Report: HTML with charts, committed to repo

### Files to Create/Modify
- claw-core/benches/[suite].rs
- claw-core/tests/chaos_[scenario].rs
- scripts/run_benchmarks.sh
- benchmarks/report.html (generated)

### Constraints
- Use `criterion` for Rust, `vitest` + custom for TS
- Run on CI hardware spec: 4 CPU, 8GB RAM
- No flaky tests — deterministic seeds
```

**Must include:**
- All 4 templates with verifiable criteria
- File paths matching repo conventions
- Constraints specific to each agent type

### 6. Tracking & Metrics
```markdown
## 📊 Tracking & Visibility

### Task Board (Kanban)
```
BACKLOG → CLAIMED → IN_PROGRESS → REVIEW → VERIFIED → DONE
```

### Daily Async Standup (CNS Packet)
```json
{
  "intent": "DAILY_STANDUP",
  "payload": {
    "agent": "rust-engineer-1",
    "task": "CL-001",
    "yesterday": "Defined ClawActor struct, 15/20 fields done",
    "today": "Finish state machine, start TriggerManager",
    "blockers": "Need clarification on MOLT-7 phase enum values",
    "confidence": "HIGH"
  }
}
```

### Weekly Sync (Voice/Video)
- **When:** Monday 09:00 AKDT
- **Who:** Schema Architect + all active agents
- **Agenda:** Demo completed work, resolve blockers, plan next week
- **Output:** Updated task board, new delegations, retired tasks

### Delegation Velocity Metrics
| Metric | Target | Current |
|--------|--------|---------|
| Tasks delegated/week | 3-5 | — |
| Time to first sync | < 24h | — |
| Time to verification | < 5 days | — |
| First-pass verification rate | > 80% | — |
| Rework cycles per task | < 1.5 | — |

### Quality Gates
- Schema compliance: 100% (automated validation)
- Test coverage: 90% core, 80% integration
- Benchmark regression: 0% (must improve or equal)
- Documentation completeness: All public APIs documented
```

**Must include:**
- Kanban board with column definitions
- Daily standup CNS packet format
- Weekly sync protocol
- Velocity metrics with targets
- Quality gates

### 7. Anti-Patterns
```markdown
## 🚫 Anti-Patterns (What NOT To Do)

| Anti-Pattern | Why It Fails | Correct Approach |
|--------------|--------------|------------------|
| **Vague acceptance criteria** | "Make it fast" → endless debate | "p99 < 10ms on CI hardware" |
| **Architect writes code** | Bottleneck, loses architectural view | Delegate, review, capture patterns |
| **Agents modify locked schemas** | Breaks contracts, cascades failures | Request schema change via Architect |
| **No sync points** | Drift, integration hell | Day 1/3/5 checkpoints mandatory |
| **Skipping benchmarks** | "Works on my machine" → prod failure | Benchmarks in CI, gate on merge |
| **Private knowledge** | Bus factor = 1 | Implementation notes → onboarding → skills |
```

**Must include:**
- Table with anti-pattern, failure reason, correct approach
- Minimum 6 anti-patterns covering spec, execution, verification, knowledge

---

## Quick Reference Card
```markdown
DELEGATION CHECKLIST (Schema Architect)
□ Task spec written with verifiable criteria
□ Schema/contract references included
□ Agent type matched to task
□ Priority labeled (P0/P1/P2)
□ Sync points scheduled (Day 1/3/5)
□ GitHub issue created with labels
□ Agent claimed task (comment)
□ Day 1 architecture review done
□ Day 3 midpoint check done
□ Day 5 demo + verification done
□ QA benchmarks pass
□ Implementation notes captured
□ Onboarding updated with patterns
□ New pitfalls → skills created
□ Task closed, merged to main
```

---

## Quality Gates

Before declaring a Delegation Workflow complete:
- [ ] All 7 sections present and complete
- [ ] 4 templates cover all agent types (Rust, TS, ML, QA)
- [ ] Active delegations table matches reality
- [ ] CNS and Git protocols documented with working examples
- [ ] Tracking metrics have targets and current values
- [ ] Anti-patterns table has 6+ entries with corrective actions
- [ ] Quick reference card is actionable (checkboxes)

---

## Anti-Patterns

| Anti-Pattern | Symptom | Fix |
|--------------|---------|-----|
| **Template Drift** | Templates don't match current repo structure | Review templates at each phase boundary; update on schema/contract changes |
| **Sync Theater** | Day 1/3/5 happen but no real review | Require artifact at each sync: Day 1=arch diagram, Day 3=WIP code, Day 5=benchmarks |
| **Metric Vacuum** | Velocity metrics never filled | Automate: CI populates metrics; Architect reviews weekly |
| **Knowledge Leak** | Implementation notes lost | Mandatory: notes → onboarding PR → skill patch in same merge |
| **Agent Mismatch** | ML task given to Rust engineer | Task spec must declare required agent type; Architect validates before delegating |