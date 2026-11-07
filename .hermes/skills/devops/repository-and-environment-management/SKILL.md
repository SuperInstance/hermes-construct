---
name: repository-and-environment-management
description: Governs the reliable interaction with remote repositories, local filesystems, and development environments.
---

# Repository & Environment Management

## Overview
Governs the reliable interaction with remote repositories, local filesystems, and development environments. This skill focuses on ensuring that all file operations and version control actions are performed with high fidelity, absolute path certainty, and robust error recovery.

## Core Principles
1. **Path Integrity**: Always prefer absolute paths. When working in polyglot or multi-platform environments (Windows/MSYS/Bash), prioritize the host's primary filesystem format to avoid resolution errors.
2. **State Verification**: Never assume a file exists or a command succeeded. Use `ls` or `stat` to verify writes, and check exit codes for all critical operations.
3. **Atomic Operations**: When performing multi-step updates (e.g., write -> git add -> git commit), treat the sequence as an atomic unit. If a write fails to land in the expected directory, do not attempt to commit.
4. **Workspace Awareness**: Be explicitly aware of the current working directory (CWD) and the root of the active workspace to prevent "ghost file" creation in unintended directories.

## Workflows

### Safe Disk Cleanup with GitHub Safety Net

User preference: never delete a local Git clone that contains work not already on GitHub. The cleanup rule is: **verify GitHub backup exists before removal**. User also prefers parallel execution for bulk operations — use background terminals with `notify_on_complete=true` to run multiple fetches/pushes/deletes concurrently.

Workflow:
1. Identify deletion candidates via audit.
2. For each candidate, run:
   `git -C "<path>" fetch origin --quiet`
   `git -C "<path>" rev-parse HEAD`
   `git -C "<path>" rev-parse origin/<branch>`
3. If local is ahead of remote or diverged:
   - Create a safety branch: `git -C "<path>" branch -f local-backup-YYYY-MM-DD`
   - Push it: `git -C "<path>" push -u origin local-backup-YYYY-MM-DD:refs/heads/local-backup-YYYY-MM-DD`
   - Only delete after push succeeds.
4. If local is behind or synced:
   - Optionally pull or delete directly, per user keep-list.
5. Parallelize with background terminals for speed, notify on complete.

Workflow:
1. Identify deletion candidates via audit.
2. For each candidate, run:
   `git -C "<path>" fetch origin --quiet`
   `git -C "<path>" rev-parse HEAD`
   `git -C "<path>" rev-parse origin/<branch>`
3. If local is ahead of remote or diverged:
   - Create a safety branch: `git -C "<path>" branch -f local-backup-YYYY-MM-DD`
   - Push it: `git -C "<path>" push -u origin local-backup-YYYY-MM-DD:refs/heads/local-backup-YYYY-MM-DD`
   - Only delete after push succeeds.
4. If local is behind or synced:
   - Optionally pull or delete directly, per user keep-list.
5. Parallelize with background terminals for speed, notify on complete.

Pitfall: Deleting `intelligence_hub/`, `research_lab/`, `ZenflowProjects/` etc. without checking for hidden `.git` subdirs. Always run `find <path> -maxdepth 2 -type d -name .git` before removal.

Pitfall: Deleting a repo that has untracked or modified local changes. Always run `git -C "<path>" status --porcelain` before removal. If dirty > 0:
- Untracked files → tar them to a backup archive before deletion, e.g. `tar -czf /c/Users/casey/<repo>_untracked_backup.tar.gz -C "<path>" <untracked paths>`
- Modified tracked files → export diff with `git -C "<path>" diff <file> > /c/Users/casey/<repo>_<file>.patch` before deletion
Never assume `git status --porcelain | wc -l == 0` means safe; verify with a quick `git rev-list --count origin/main..HEAD` and `git rev-list --count HEAD..origin/main` to confirm zero ahead/behind.

### Production Close-out & Workspace Pruning

User wants to push all plans/work to GitHub, verify from outside via GitHub UI / `git ls-remote` / `gh api`, then delete local copies to clean workspace before moving on. Full pattern documented in `references/production-closeout-2026-08-20.md`.

User explicitly prefers parallel terminals for bulk operations — use background terminals with `notify_on_complete=true` to run multiple fetches/pushes/deletes concurrently. Always confirm remote visibility with an external check before removal.

User explicitly prefers parallel terminals for bulk operations — use background terminals with `notify_on_complete=true` to run multiple fetches/pushes/deletes concurrently. Always confirm remote visibility with an external check before removal.

Key steps:
1. Inventory all git clones under user home and projects folder.
2. Detect shared work-trees: compare `git rev-parse --git-dir` and `--show-toplevel` across paths. Do not audit/push subdirs individually if they share a git-dir.
3. For each independent clone: fetch, compare HEAD vs origin branch, classify synced/behind/ahead/diverged.
4. Safety net: ahead/diverged → create `local-backup-YYYY-MM-DD` branch and push. Dirty → tar untracked, patch modified files.
5. Verify remotely: use `gh api` or `git ls-remote` to confirm remote HEAD matches local after push.
6. Parallelize fetches/pushes/deletes with background terminals and `notify_on_complete=true`.
7. Delete local copies only after remote verification.

Pitfall: Initializing Git on existing workspace with `git add .` before `.gitignore` will commit node_modules/build artifacts. Safe sequence: `git init` → create `.gitignore` → commit .gitignore → then `git add .`. If node_modules already committed, purge with `git rm -r --cached node_modules` and optionally `git filter-repo --path node_modules --invert-paths --force` to remove from history, then force push. Note: `git filter-repo` removes the `origin` remote by design; re-add it with `git remote add origin <url>` before pushing the rewritten history.

### Repo Documentation & Handoff Pattern

When closing out a fleet of repos and handing off to Lucineer/other agents, generate per-repo onboarding docs so the next operator can match your level of understanding.

Standard set per repo:
- `CURRENT_STATE.md` — tech stack, structure, key files, git state, known gaps
- `NEXT_PHASES.md` — phased roadmap to production grade, modularization tasks, integration points
- `ONBOARDING.md` — prerequisites, quick start, build/test workflow, core concepts, contribution steps

Process:
1. Inspect repo: `find . -maxdepth 2 -type f -name README.md` etc.
2. Write the three docs via `write_file` using absolute Windows paths.
3. `git add`, `git commit -m "docs(...): add CURRENT_STATE.md, NEXT_PHASES.md, ONBOARDING.md"`
4. Push to GitHub, verify with `git ls-remote` / `gh api`, then delete local clone.

This pattern was used for SuperInstance core repos: claw, constrainttheory, claw-extensions, etc., and enables safe workspace pruning.


### GitHub Sync Audit for Multi-Clone Fleet

When auditing a fleet of local clones for GitHub sync status — e.g., "get everything on GitHub" — use an absolute-path inventory first, then verify each clone is truly independent:

1. **Inventory clones**
   Use `search_files(pattern='*.git', target='files')` or a `find` for `.git` directories under the user home. For each candidate path, record:
   `path | git remote get-url origin | git rev-parse --git-dir | git rev-parse --show-toplevel`

2. **Detect shared work-trees / monorepo subdir masquerade**
   If multiple paths share the same `--git-dir` or `--show-toplevel`, they are not independent repos. Example: `/c/Users/casey/polln/*` all resolve to `/c/Users/casey/polln/.git`. Do not audit/push them individually.
   Detect with:
   ```
   git -C "<path>" rev-parse --git-dir
   git -C "<path>" rev-parse --show-toplevel
   ```
   Duplicate `.git-dir` → skip individual audit, treat as subtree of the top-level repo.

3. **Identify duplicates / temp clones**
   Paths like `/c/Users/casey/AppData/Local/Temp/hermes-clone` or sandbox copies under `/c/Users/casey/sandbox/*` often duplicate a primary clone. Keep one canonical path per remote.

4. **Compare local vs remote**
   For each independent clone:
   - `git fetch --quiet origin`
   - `local_rev=$(git rev-parse HEAD)`
   - `remote_rev=$(git rev-parse origin/<branch>)`
   - If `local_rev == remote_rev` → synced
   - If `local_rev` is behind → `ahead=0 behind=N`
   - If both ahead/behind → diverged
   On Windows/MSYS, prefer `git -C "<path>"` to avoid `cd` context errors.

5. **Action mapping**
   - synced → no action
   - behind → pull if the user actively uses it, else mark for deletion
   - ahead → push to a new named branch first, never force-push without explicit instruction
   - diverged → prompt user for merge/rebase strategy

### Robust File Writing & Committing
To avoid the "ghost file" or "path mismatch" trap when working with Git repositories:
1. **Identify the Root**: Determine the absolute path of the repository root.
2. **Direct Write**: Use absolute paths for all `write_file` operations.
3. **Scoped Execution**: When running shell commands (`terminal`), `cd` into the repository root first to ensure the subshell context is synchronized with the filesystem.
4. **Verify before Push**: Run `ls` or `find` on the specific target file immediately after writing and before invoking `git add`.

## Pitfalls & Mitigations

**Git init with bundled dependencies**

When turning an existing workspace like `C:\\projects` into a Git repo, never run `git add .` before creating a `.gitignore`. Node modules, build artifacts, caches, and large binary dependencies will be staged and committed, inflating the repository and making future pushes impossible or prohibitively slow. The safe sequence is:

1. `git init`
2. Create `.gitignore` first with the appropriate patterns e.g. `node_modules/`, `dist/`, `*.log`, `.env`
3. `git add .gitignore && git commit -m "chore: add gitignore"`
4. `git add .`

If `node_modules` has already been staged/committed, remove it from tracking with:

```bash
git rm -r --cached node_modules
git commit -m "Remove node_modules from tracking"
```

Then push to a newly created GitHub repo with `gh repo create <owner>/<name> --public --source=. --push`.

| Pitfall | Mitigation |
| :--- | :--- |
| **Path Resolution Mismatch** (e.g., `/c/Users/...` vs `C:\\\\Users\\\\...`) | Use the host's preferred path format (Windows path style for Windows hosts) to ensure tools like `write_file` resolve correctly. |
| **Ghost Writes** (Files created outside the workspace) | Always verify the absolute path against the known project root before executing writes. |
| **Silent Git Failures** (Command fails because pathspec doesn't match) | Always `ls` the target file after a write to confirm it is exactly where you expect it to be within the Git working tree. |
| **Command Loop Frustration** | If a tool fails with a path error, do not retry the exact same command. Change the path format or the execution context (e.g., `cd` into the dir). |
| **SuperInstance Gitlink Submodule Trap** | `polln/` tracks child repos like `claw/` as 160000 gitlinks without a `.gitmodules` entry. `git rev-parse --show-toplevel` from inside `claw/` still resolves to `polln/`, and any `git add/commit/push` yields `fatal: in unpopulated submodule 'claw'`. Do NOT try to fix the parent `.git`. Workaround: clone the upstream repo to a temp worktree (e.g., `/tmp/claw_work`), perform writes/commits/pushes there, then mirror changes back to the `polln/` monorepo if needed. |
| **Unborn Branch on Diverged Repo** | `git branch -f local-backup-...` then `git push` can fail with `fatal: not a valid object name: 'main'` when HEAD is unborn or points to a non-existent ref. Verify branch exists with `git symbolic-ref --quiet HEAD` or `git rev-parse --verify HEAD` before creating backup branch. For repos with no commits, create an orphan backup via `git checkout --orphan local-backup-...` first. |
| **Write File Path Warning Outside Workspace** | `write_file` with POSIX `/c/Users/...` can resolve to `C:\\\\c\\\\Users\\\\...` on this host, creating ghost files outside `C:\\\\Users\\\\casey`. Use `terminal` with `cat > /c/Users/...` or `printf` to ensure POSIX→Windows mapping is correct, and verify with `ls -l` immediately. **Critical**: When interleaving `write_file` and `terminal` operations, always use **absolute Windows paths** (`C:\\\\Users\\\\...`) for `write_file`. The `terminal` tool correctly maps `/c/Users/...` to `C:\\\\Users\\\\...`, but `write_file` incorrectly maps `/c/Users/...` to `C:\\\\c\\\\Users\\\\...` (extra `\\\\c` directory). This is the #1 source of \"file not found\" errors when mixing these tools. |
| **Heredoc / Multi-line Quoting in terminal** | `python3 - << 'PYEOF'` and `python3 -c "..."` multi-line strings frequently fail in MSYS/Git Bash due to quote escaping, variable interpolation, or EOF-matching edge cases. **Preferred pattern:** Write the script to a file via `write_file`, then run it via `terminal python3 <absolute_path.py>`. This avoids shell quoting entirely and makes the script reusable/inspectable. Reserve inline `-c` for trivial single-line commands only. |
| **PowerShell from Bash/MSYS Terminal** | Running complex PowerShell commands via `powershell -Command \"...\"` from bash/MSYS terminal fails due to path escaping and interpolation issues. **Better pattern**: Write a `.ps1` script via `write_file` (using Windows paths), then execute via `powershell -ExecutionPolicy Bypass -File \"C:\\path\\to\\script.ps1\"`. |
| **GitHub API Authentication/Visibility Gaps** (e.g., `404 Not Found` on org events) | A `404` on an organization's event or repo list doesn't always mean the repo is missing; it often means the endpoint is private or requires an authenticated session (Git CLI/PAT). Before declaring a resource \"not found,\" attempt access via an authenticated tool like `gh api` or verify the visibility/permissions of the target organization. |
| **Git Disk-Full During Geometric-Repack** | When `git commit` fails with `fatal: sha1 file '.git/objects/pack/tmp_pack_...' write error. Out of diskspace` / `error: failed to perform geometric repack`, **first check `git log -1`**. The commit may have actually succeeded before the post-commit repack failed. The staged index, however, will likely be lost. Mitigation: Always have at least 10-20GB free; free up space via `pip cache purge`, deleting temp files, or clearing `~/.cache/puppeteer` before large commits. |
| **Origin Force-Pushed With Backup Branch** | When `git push` is rejected and `git fetch` shows `(forced update)` on `origin/main`, check for backup branches like `origin/backup/*` (e.g., `origin/backup/wrong-cudaclaw-push-2026-08-13`). **SAFE RECOVERY PATTERN**: Push your local work to a new named branch first with `git push origin HEAD:refs/heads/my-work-branch-name`, then let the user decide how to integrate/rebase. Never force-push or try complex rebase merges in force-push situations unless explicitly instructed — these are sensitive operations with history loss risk. |

## References
- `references/path-resolution-failures.md`: Collected logs and examples of MSMS/Bash vs Windows path resolution errors.
- `references/superinstance-gitlink-workaround.md`: Verified workaround for polln gitlink submodule trap and write_file ghost path issues.
- `references/github-sync-audit.md`: Multi-clone GitHub sync audit pattern for \"get everything on GitHub\" fleets, shared work-tree detection, and duplicate pruning.
- `references/production-closeout-2026-08-20.md`: Production close-out & workspace pruning workflow, GitHub verification before deletion, node_modules history purge pattern, and backup branch safety net.
