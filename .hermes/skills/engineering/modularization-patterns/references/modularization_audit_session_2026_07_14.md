# Modularization Audit: Session [2026-07-14]

## Scenario: Windows Git MAX_PATH Failure
**Observation**: Attempting to `git add .` in a large `node_modules` directory on a Windows host failed with `error: open(...): Filename too long`.

## Reproduction Steps
1. Attempt a broad `git add .` in a directory contains deep nested structures (e.g., `node_modules/.pnpm/...`).
2. Observe `fatal: adding files failed` and `Filename too long`.

## Resolution / Workaround
**Do not attempt a broad `git add .` when deep dependency trees are present in a large monorepo.**
Instead, use **Selective Staging**:
1. Identify the specific files/directories belonging to the workspace (e.g., `packages/*`, `src/*`, `DEBRIEF_*.md`).
2. Use `git add <path/to/file1> <path/to/file2>` explicitly.
3. Ensure a robust `.gitignore` is in place to prevent accidental indexing of `node_modules`.

## Identified Extraction Candidates
**`constrainttheory`**:
- Research/Reports: `docs/reports/*.pdf` (Target: `SuperInstance-papers`)
- Research Folders: `docs/`, `research/`

**`spreadsheet-moment`**:
- Hardcoded Hardware Logic: `workers/src/lucineer_integration.ts` (Target: `IEquipmentModule` pattern in a new `equipment-lucineer` package)
- Monolithic math/tensor ops: Move to the specialized module.
