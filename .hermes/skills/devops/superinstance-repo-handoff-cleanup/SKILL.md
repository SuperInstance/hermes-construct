---
name: superinstance-repo-handoff-cleanup
description: Production-grade handoff of SuperInstance repos to GitHub with documentation, verification, and local workspace cleanup.
category: devops
---

# SuperInstance Repo Handoff & Cleanup

## Purpose
Ensure every SuperInstance repo is production-grade, fully documented, verified on GitHub, and safe to remove locally. Supports Lucineer onboarding and workspace clearing.

## Trigger
User requests to push plans/docs to GitHub for all repos, verify remote state, and clean local drive.

## Workflow
1. Inventory repos
   - Core: constrainttheory, claw, spreadsheet-moment, dodecet-encoder, claw-extensions, constrainttheory-ml-demo, SuperInstance-papers
   - Polln work-tree and non-core clones
2. Create handoff docs per repo
   - CURRENT_STATE.md — tech stack, structure, git status, known gaps
   - NEXT_PHASES.md — production checklist, modularization tasks, integration points
   - ONBOARDING.md — quick start, dev setup, key concepts
3. Commit and push
   - Stage docs
   - Commit with message `docs(...): Add CURRENT_STATE.md, NEXT_PHASES.md, ONBOARDING.md`
   - Push to origin. If diverged, create branch from origin/main and cherry-pick docs.
4. Verify from outside
   - Confirm file exists on GitHub web UI / via `git ls-remote --heads origin` and `git rev-parse HEAD`
   - Report URL and commit SHA
   - For backup branches, verify `local-backup-YYYY-MM-DD` exists remotely before deleting local
5. Backup diverged work
   - For repos ahead of remote: push `local-backup-YYYY-MM-DD` branch before cleanup
6. Local cleanup
   - Remove local clone only after verification confirmed
   - Keep core repos until user confirms full handoff

## User Preferences
- Direct, honest voice, no filler
- Verify artifacts before deletion
- GitHub safety net: push backup branch for diverged/ahead work
- Parallel terminals with notify_on_complete
- Maritime units for any telemetry; perception layer only
- User wants every repo documented with CURRENT_STATE.md, NEXT_PHASES.md, ONBOARDING.md so Lucineer can onboard to Hermes-level understanding; docs must be pushed to GitHub before local deletion
- User explicitly requires 'push all plans to GitHub repos for all of these and make sure they are all up to date in GitHub by checking from the outside before you delete' — verification must be external, not just local log

## Pitfalls
- Subtree docs may need to live under parent repo
- Divergence requires branch workaround, not force push
- Do not delete local before remote verification
- Repo moves: remote URL may change (e.g., ai-writings → AI-Writings). Always run `git remote -v` and update URL before fetch/merge.
- Merge conflicts on dirty worktrees: commit or stash before pull/rebase; if conflicted, abort merge and push to a `local-backup-YYYY-MM-DD` branch instead of forcing.
- Large binary history bloat: see `references/git-filter-repo-purge.md` for `git filter-repo` purge workflow.
- Windows git path quirks: files with trailing newlines in names need careful `git mv` handling.
- `ai-writings` repo moved to `SuperInstance/AI-Writings.git`; verify remote URL after clone.
- `polln` is a shared work-tree with remote `SuperInstance/claw.git`; subdirectories are not independent repos. Docs for `constrainttheory`, `claw-extensions` etc. must be committed via `polln` root.
- Dirty uncommitted changes must be committed to a backup branch before any push/pull; use `git add -A && git commit -m "Backup local changes YYYY-MM-DD"` then push to `local-backup-YYYY-MM-DD`.
- When `git pull --rebase` fails with "You have unstaged changes", commit first. If merge fails, abort with `git merge --abort` and push to backup branch instead of forcing.
- Parallel terminals: use `background=true` with `notify_on_complete=true` for long pushes; do not block main turn waiting for completion.
- SSH permission denied on GitHub: switch to HTTPS remote via `git remote set-url origin https://...` before push.
- Repo not found errors: verify repo exists under SuperInstance org; some clones are local-only and have no remote — do not attempt push, archive locally instead.
- `git symbolic-ref` + force push to backup branch is required when HEAD is detached or branch name is ambiguous.

## Outputs
- GitHub URLs for each repo’s docs
- Cleanup confirmation log

## References
- `references/session-2026-08-20-cleanup.md` — session-specific details for 2026-08-20 handoff run
