---
name: schema-population-workflow
description: Governs the detection, mapping, and population of stub/empty schemas using production-grade source schemas from a reference archive. Applies when a project has canonical "gold standard" schemas that are placeholder stubs and a parallel archive contains the real definitions. Covers inventory, mapping, population, backup, commit, and push.
---

# Schema Population Workflow

This skill governs the process of detecting empty schema stubs, mapping them to production source schemas, and populating them with real definitions — closing the gap between "the code validates against a schema" and "the schema actually enforces anything."

## When to Load

Load this skill when:
- A repo has `schemas_gold_standard/` or `schemas/` files with byte counts under 200 bytes (e.g., `{"title": "...", "type": "object"}`)
- An audit has revealed that schema validation is passing vacuously
- A reference archive exists (local or remote) containing production-grade schemas that map to the stubs
- User directs "populate the schemas", "fill the stubs", or "the schema is empty"

## The Stub Detection Rule

A schema file is a stub if:
1. File size < 200 bytes, AND
2. Content matches `{"title": "...", "type": "object"}` or similar minimal form

Common stub patterns:
- `{"title": "X Schema", "type": "object"}`
- `{"title": "X", "$schema": "..."}`
- `{"type": "object"}` (no title at all)

**Byte-count thresholds:**
- < 200 bytes: definite stub
- 200–1000 bytes: likely incomplete — inspect content
- > 1000 bytes: probably production-ready — verify against Checklist

## Source Schema Inventory

Before populating, catalog all available source schemas:
1. Scan the reference archive (`/path/to/archive/schemas/` or similar)
2. For each schema file: record name, byte count, title field, `$id` field
3. Identify which source schemas map to which stubs by title, domain, and field overlap

**Mapping signals:**
- Title keyword match: `smp_cell_schema.json` → `claw_agent.json`
- Domain match: `conditional_geometry_schema.json` → `claw_seed.json` (Ψ: (X,P,C) → (X',P',C'))
- Composite match: multiple source schemas may map to one stub (e.g., `cold_logic`, `channel_depth`, `foldable_tensor`, `folding_group` all contribute to `claw_equipment.json`)

## The Population Plan

Before writing, produce a Schema Population Plan document:
1. **Stub table:** file, byte count, current content, target fields needed
2. **Source mapping table:** source file, size, maps to stub, key fields to extract
3. **Cross-reference table:** Checklist invariant → schema field → source field
4. **Effort estimate:** per stub, with priority ranking
5. **Backup plan:** copy original stub to `.bak` before overwriting

## Population Sequence

### 1. Backup
```bash
cp schema_gold_standard/claw_agent.json schema_gold_standard/claw_agent.json.bak
```

### 2. Copy Base Schema
- Use the most production-ready source schema as the base
- If a direct 1:1 mapping exists (e.g., `smp_cell_schema.json` → `claw_agent.json`), copy it directly

### 3. Augment with Invariant Fields
- Add fields from the theoretical framework that are not in the source schema:
  - Conservation law fields: `gamma`, `eta`, `delta`
  - Thermal fields: `temperature`, `semanticDistance`, `crystallizationRate`
  - Molt fields: `moltCount`, `capability`, `lastValidation`
  - Identity fields: `shell.identityHash`
  - Execution context: `executionContext.{kvCacheFingerprint, attentionPatternHash, lastInputHash, lastOutputHash}`

### 4. Update `$id`
- Set the populated schema's `$id` to its canonical name (e.g., `claw_agent.json`, not the source's `smp_cell_schema.json`)
- This ensures downstream references resolve correctly

### 5. Lint
- Run JSON syntax validation on the populated schema before committing
- All new schemas MUST lint clean before they enter version control

### 6. Commit + Push
- Commit message must list: (a) what stubs were replaced, (b) source files, (c) Checklist invariants now enforceable
- Push to origin immediately after commit (do not batch — each schema population is a deployable unit)

## Checklist Integration

After population, every field in the populated schema must map to at least one Checklist invariant:
- `state.gamma`, `state.eta`, `state.delta` → CONS-1, CONS-2
- `temperature`, `semanticDistance` → DRM-1, CK-1
- `moltCount`, `capability`, `shell.identityHash` → MOLT-1 through MOLT-6
- `lastValidation` → MLT-2 (staleness formula)
- `semanticDistance` creative zone bounds → P02, P56 creative zone

If a populated field has no Checklist mapping, flag it as "unverified" in the Population Plan.

## Post-Population Verification

After populating, pushing, and verifying the schema is live:
1. Run the Verification Checklist Runner against the new schema (or a live telemetry snapshot that uses it): `python3 scripts/verification_checklist_runner.py`
2. Confirm all CONS, MOLT, DRM, CK, OPT, UA, MLT rules pass before declaring the population complete.
3. If any rule fails, the schema is incomplete — patch the schema, re-run, re-push.