# Production Close-out & Workspace Pruning — 2026-08-20

## Session Goal
Push all local plans / work to GitHub, verify from the outside that repos are up to date, then delete local clones to clear workspace for next phase.

## Workflow Executed
1. Identify all git clones under C:\Users\casey and C:\projects
2. Detect shared work-trees / monorepo subdir masquerade
   - `polln/` is a single git repo; subpaths like polln/claw, polln/spreadsheet-moment share /c/Users/casey/polln/.git
3. Verify sync status per independent clone:
   - `git fetch --quiet origin`
   - Compare HEAD vs origin/<branch>
   - Classify synced / behind / ahead / diverged
4. Safety net before deletion:
   - If ahead/diverged → create `local-backup-YYYY-MM-DD` branch and push
   - If dirty → tar untracked files, export patches for tracked modifications
5. Parallel execution for speed:
   - Use `terminal` with `background=true` and `notify_on_complete=true` for bulk fetch/push/delete
6. Verify from outside:
   - After push, use `gh api repos/<owner>/<repo>/git/refs/heads/<branch>` or `git ls-remote` to confirm remote HEAD matches local
7. Prune:
   - Delete synced / backed-up local clones
   - Remove empty directory shells

## Key Actions Taken
- Created GitHub repo for previously unversioned workspace `C:\projects` → `SuperInstance/projects`
- Pitfall encountered: `git init && git add .` committed `node_modules` (~hundreds of MB)
- Recovery:
  1. `git rm -r --cached node_modules`
  2. Create `.gitignore` first
  3. `git filter-repo --path node_modules --invert-paths --force` to purge history
  4. Force push cleaned history
  5. Remove `node_modules` locally
- Deleted safe synced clones:
  - activelog-ai, boat-agent, hermes-avatar, perception-cascade, provenance-log, trinity-agent, tzpro-agent, vessel-agent, vessel-quest, hermes-roblox-construct, hermit-crab
- Pruned `_si_research` tiny clones after confirming GitHub mirrors
- Backed up untracked changes for `flux-runtime` and `SuperInstance` before deletion

## Production Readiness Checklist Observed
User wants repos production-grade and shippable before moving on. Checklist:
- Tests green & CI enforced
- Version tags vX.Y.Z on main
- CHANGELOG / Release notes
- Docs & onboarding
- Dependabot, license, .gitignore sane
- Modularization complete, no bloat in core
- Integration tests pass
- All local diverged work pushed to backup branches

## Pitfalls Recorded
- Do not `git add .` before `.gitignore` in Node workspaces
- `git filter-repo` removes origin remote, must re-add before push
- `write_file` with POSIX /c/Users/... maps to C:\\c\\Users... — use Windows paths for write_file
- Shared work-tree detection is mandatory before auditing/pushing subdirs

## Artifacts
- Backup branches: local-backup-2026-08-20 on casting-call, cudaclaw, conservation-enforcer-rs, claw
- Tar backups: _si_research_flux-runtime_untracked_backup.tar.gz, _si_research_SuperInstance_open-mind.patch
- New GitHub repo: SuperInstance/projects
