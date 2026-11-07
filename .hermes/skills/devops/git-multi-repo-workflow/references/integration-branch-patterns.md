# Git Patterns for Multi-Repo Workflows (2026-03 session)

## Submodule Pointer Cleanup

If `git status` shows a path in `D` (deleted) or `R` (renamed) with mode `160000` and there is no `.gitmodules` or `.git/modules/<path>`, it is a dangling submodule pointer. Remove it from the index:

```
git rm --cached --quiet <submodule-path>
```

Then recommit. This unblocks future commits in the parent repo.

## Force-Push for Integration Branches

When `git pull` fails due to unrelated histories on an integration branch with no external collaborators, force-push is acceptable:

```
git push --force origin <branch>
```

Use this for `clean-publish` style branches. Do not use on shared main/master branches.

## Commit Convention for Phase-Based Work

Each phase completion should be a single commit with a message of the form:

```
feat: Phase N — <brief title>

- <key change 1>
- <key change 2>
- test status (e.g., 26/26 green)
- roadmap reference
```

Example:
```
feat: Phase 6 — spatial index baseline (ReceptiveField, SpatialIndex trait, NaiveSpatialIndex)

- common.rs: added ReceptiveField, SpatialEntry, SpatialIndex trait, NaiveSpatialIndex
- tests/spatial_index_test.rs: 5 tests covering insert/remove/query/knn/duplicate
- test count: 26/26 green
- roadmap: Phase 6 FPS Spatial Filtering
```

## Workspace-Level Verification Before Push

Always run the workspace verification stack before committing and pushing:

1. `cargo check --workspace` — exit 0
2. `cargo test --workspace` — all tests pass
3. `git status` — no unintended artifacts (target dirs, submodule pointers)
4. Commit and push only after all three are green.
