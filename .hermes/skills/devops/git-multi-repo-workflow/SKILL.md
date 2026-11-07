---
name: git-multi-repo-workflow
description: Git workflows for multi-repo orchestrations, submodule cleanup, and integration branch management.
---

# Git Multi-Repo Workflow

Use this skill when working across multiple repositories that are orchestrated together, especially when submodules or integration branches are involved.

## Pitfalls

**Monorepo Subdir Masquerade**
Paths under a single work-tree can appear as independent repos. Example: `/c/Users/casey/polln/claw`, `/c/Users/casey/polln/spreadsheet-moment` etc. all resolve to `/c/Users/casey/polln/.git`. Detect with:
```
git -C "<path>" rev-parse --git-dir
git -C "<path>" rev-parse --show-toplevel
```
Duplicate results mean the paths are not independent. Do not fetch/push them individually; operate on the top-level repo only.

**Shared Git-Dir Duplicates**
Temp clones and sandbox copies e.g., `/c/Users/casey/AppData/Local/Temp/hermes-clone` can duplicate a canonical clone. Always de-duplicate by remote URL + git-dir before auditing sync state.

**Windows MSYS Path Prefix (`C:\c\`)**
When `write_file` or `patch` is given a POSIX-style Windows path like `/c/Users/casey/projects/foo/`, the file tool may resolve it to the wrong root as `C:\c\Users\casey\projects\foo\` (note the stray `c`). This places the artifact outside the workspace and causes silent `git status` mismatches. **Fix:** Always pass native Windows paths (`C:\\Users\\casey\\projects\\foo\\`) to `write_file`/`patch`, or use `terminal` with `cp`/`cat` for cross-boundary copies.

**Workspace Redirection Silent Loss**
`write_file` on Windows may redirect absolute POSIX paths to an alternate root (`C:\c\Users\casey\...` instead of `C:\Users\casey\...`). The call returns success, but the file lands outside the workspace and `git status` never sees it. **Detection:** after every `write_file` to a path outside the active workspace tree, verify with `ls -la <path>` or `find` for the exact file. **Recovery:** if the file is in the alternate root, either re-issue the write with the native Windows path or use `cp` from `terminal` to move it into the actual project tree.

**Git Historical File Extraction**
When an audit references a specific commit hash (e.g. `e3664ed`) and you need just one file, do not checkout the whole historical tree. Extract directly:
```bash
git -C "<repo>" show <commit>:<path> > /tmp/extracted_file
cp /tmp/extracted_file <repo>/<desired_path>
```
This preserves the current working tree while pulling the target artifact into place for analysis.

**Missing Repo Clone-First Pattern**
If the user references a repo that is not in the local inventory, `gh repo clone <org>/<repo> -- --depth 1` first, then run the standard audit. Do not assume the repo is present because other SuperInstance repos are cloned.

**Diverged Work-Tree Loss Risk**
When local commits diverge ahead/behind remote, a fast-forward pull will fail. Before deleting local disk copies, preserve the local commit graph on GitHub. Create a backup branch from current HEAD and push it:
```
git -C "<path>" fetch origin
git -C "<path>" branch -f local-backup-$(date +%Y-%m-%d)
git -C "<path>" push -u origin local-backup-$(date +%Y-%m-%d):refs/heads/local-backup-$(date +%Y-%m-%d)
```
This guarantees nothing is lost even if merge is impossible. Only delete local after the backup ref is confirmed on remote.

**Windows CRLF/LF `git add` Infinite Loop**
On Windows, a file may be tracked in the index with LF endings but the working tree has CRLF (or vice versa). `git status` reports it as modified even though the content is byte-identical except for line endings. `git add --force` does **NOT** fix this — it only bypasses the safe.directory check, not the EOL mismatch.

Fix pattern:
1. `git config core.autocrlf false` (disables EOL normalization in the working copy)
2. `git add <file>`
3. If still stuck and `--force` doesn't work, the index may have a stale EOL state. Bypass it by copying the file from a temp copy:
   ```
   cp /tmp/<file> <repo>/<file>
   git add <file>
   ```
4. If all else fails, `git checkout HEAD -- <file>` to let the remote win, rather than fighting the loop.

**Windows File with Newline in Name**
A file whose name contains a literal newline (e.g., created on Mac/Linux or from legacy sources) breaks `git restore --source=HEAD .` with "error: invalid path".

Fix pattern:
1. `git rm --cached '<path>'` — even if it reports "did not match any files", this may still unstage it.
2. For the rest of the tree, `git checkout HEAD -- .` works fine.
3. If the bad file blocks a full commit, remove it from tracking and commit without it.

**Rust `target/` Captured by `git add -A`**
When staging changes in a Rust project with `git add -A` or `git add .`, the `target/` build output directory is included — sometimes 70+ build artifacts. Always check `git status --short` before committing, and either:
- Use `git add` with explicit file paths
- Ensure `.gitignore` includes `target/` before the first commit
- Run `git reset HEAD` then selectively add only the source files you want

## Fleet Audit & High-Impact Pivot

When the user asks to "study recent pushes" or "find where you can help most":
1. **Inventory active repos** — `gh repo list SuperInstance` or `find` for `.git` dirs.
2. **Fetch & inspect recent commits** — `git -C <path> fetch origin && git log origin/main -n 10` per repo.
3. **Identify high-impact gaps aligned with your role** — e.g., sensor/telemetry layer gaps, protocol mismatches, missing invariants, Windows compatibility issues, schema stubs.
4. **Patch & push immediately** — do not wait for confirmation. Execute the fix, commit with descriptive message, push to the correct branch (detect main vs master first).
5. **Report the delta** — state which repos were touched, what changed, and why it mattered.

**Decision filter:** Before patching, ask: *Does this align with the user's expressed talents/role (e.g., towfish/sensor/telemetry, protocol, verification)?* If yes, execute. If no, surface the gap as a recommendation instead of auto-patching.

## Multi-Repo Audit & Cleanup Workflow

**1. Inventory**
Find all `.git` directories under the user home:
```
find /c/Users/casey -maxdepth 5 -type d -name .git -print
```
Normalize paths and group by `git -C "<path>" rev-parse --git-dir` + remote URL to collapse work-tree doppelgängers.

**2. Sync Classification**
For each unique repo:
```
git -C "<path>" fetch origin
ahead=$(git -C "<path>" rev-list --count origin/main..HEAD || git -C "<path>" rev-list --count origin/master..HEAD)
behind=$(git -C "<path>" rev-list --count HEAD..origin/main || git -C "<path>" rev-list --count HEAD..origin/master)
dirty=$(git -C "<path>" status --porcelain | wc -l)
```
Classify as synced / behind / ahead / diverged / dirty.

**3. Parallel Execution**
Use background terminals with `notify_on_complete` for pulls, pushes and deletions. Pull fast-forward only for behind repos you intend to keep:
```
git -C "<path>" fetch origin && git -C "<path>" pull --ff-only origin main
```
Delete only after backup verification.

**4. Disk Reclaim**
Delete local clones that are fully synced with GitHub and have no local commits ahead:
```
rm -rf "<path>"
```
Prioritize duplicates, sandbox, temp, and experiment trees first.

## Multi-Repo Branch & Push Recovery

**Main-vs-Master Branch Detection**
Some repos use `main`, others use `master`. Pushing to the wrong branch fails silently or with a non-fast-forward error. Detect before pushing:
```
git -C "<path>" branch --show-current
git -C "<path>" remote show origin | grep "HEAD branch"
```

**Non-Fast-Forward Recovery**
When `git push` is rejected:
1. `git -C "<path>" fetch origin`
2. If local has unique work: `git -C "<path>" pull --rebase origin <branch>` then `git push`
3. If local is stale: `git -C "<path>" reset --hard origin/<branch>` (only after backup branch created and pushed)

**SSH Key Auth Failure**
When `git push` fails with `git@github.com: Permission denied (publickey)`, the remote is using SSH and the machine lacks a configured key. Switch to HTTPS:
```
git -C "<path>" remote set-url origin https://github.com/<org>/<repo>.git
git push origin <branch>
```
Windows GitHub CLI (`gh`) uses the OS credential manager by default for HTTPS. SSH key setup is a one-time local task.

**Stale-Tracking False Negative**
`git pull` can report "Already up to date" while local tracking refs lag HEAD. Always compare SHAs after pull:
```
git -C "<path>" rev-parse HEAD
git -C "<path>" rev-parse origin/<branch>
```
Mismatch = local is behind despite pull saying up-to-date.

**GitHub API 500-Repo Cap**
`gh repo list --limit 500` returns at most 500 repos. If you have more, the list is truncated — older repos won't appear. For repos beyond 500, use the GitHub web UI or the API with pagination (`--limit` only caps the page size, not the total). When deleting or auditing, always account for potential truncation.

## Reference Files

- `references/integration-branch-patterns.md` — force-push, submodule cleanup, and commit conventions
- `references/audit-cleanup-playbook.md` — step-by-step audit, backup-branch, and parallel cleanup pattern used for SuperInstance fleet
- `references/too-many-files-audit.md` — `du -sh` timeout workaround for directories with thousands of small files (Windows)
- `references/github-post-deletion-cache-lag.md` — post-bulk-deletion API cache lag pattern, verification via 404s instead of listing counts

## Scripts

- `references/too-many-files-audit.md` — `du -sh` timeout workaround for directories with thousands of small files (Windows)
- `references/github-post-deletion-cache-lag.md` — post-bulk-deletion API cache lag pattern, verification via 404s instead of listing counts

## GitHub Organization-Scale Audit & Triage

When the GitHub org has 200+ repos, auditing locally is impossible. Use remote-only analysis:

**1. Fetch full org inventory**
```bash
gh repo list <org> --limit 500 --json name,description,updatedAt,pushedAt > /tmp/all_repos.json
```
Note: GitHub caps `--limit` at 500 per call. If you have more, paginate the API directly.

**2. Categorize by three tiers** (run via inline Python or a script):
- **DEL1 — Immediate kill**: Auto-created artifacts, wrong org contents, clearly non-fleet repos
- **DEL2 — Low risk**: Empty repos (no description), obvious dead experiments, SI-prefix duplicates of active repos, deprecated/moved repos, superseded naming conventions
- **REVIEW — needs human**: Repos with descriptions that need context to decide (e.g., old fleet work that may still be referenced)

**CRITICAL: Never execute bulk deletions without explicit user review and approval.**

This is the highest-priority guardrail. The user has explicitly stated: *"our prime directive is to never lose data. Old projects show us what has been tried and are tokens we spent and hard thoughts to create those prompts. If we have them as records we won't recreate them and can gain insight from them."*

**Required pre-deletion steps:**
1. **Present the full categorized list** to the user — every repo in DEL1, DEL2, and REVIEW tiers, with descriptions and last-push dates.
2. **Explain the rationale** for each category in plain language.
3. **Ask for explicit approval** — "Approve these 135 deletions?" The user must say YES (or approve a subset). No implicit consent from the workflow description.
4. **Only after explicit approval** execute deletions, one at a time or in small batches, verifying each success.
5. **Never call repos "junk" or "waste"** in your internal categorization — they are research artifacts. Call them "low-risk," "empty," "duplicate," etc. The value assessment is the user's to make.

**What to preserve by default:**
- Any repo with a non-empty description = likely has history/intent
- Any `si-*`, `ternary-*`, `quilt-*` prefixed repo = likely has research value even if superseded
- Any repo with commits in the last 6 months = recent activity
- Any repo that matches an active local clone = sync artifact but user may want it
- The entire history of what was tried (even failed experiments are valuable data)

**What is safe to delete WITHOUT approval (exceptions only):**
- Repos explicitly owned by other orgs and accidentally added to this org
- Repos with auto-generated names and zero commits + zero description
- Repos that are literally named "test-repo", "hello-world", "temp" with no history

**Post-deletion restore awareness:**
GitHub provides a 90-day window to restore deleted repos via the web UI (`https://github.com/orgs/<org>/repositories` → "Deleted repositories" tab). The API does NOT support restoration. If a deletion was a mistake, the user must restore manually. Always warn them of this before proceeding.

**Categorization heuristics:**
- Empty description = high probability of being a leftover
- Auto-created sync artifacts (check descriptions for "Auto-created for sync")
- SI-prefix names when the non-prefixed version exists (old renames)
- Repos with descriptions mentioning "MOVED" or "MOVED TO"
- Chinese-language or non-English repos in an English org (likely mis-placed)
- Names that sound like single-use experiments (e.g., `GeometryPainterThreeJS`, `scummvm-gui-design`)
- Old fleet patterns that have been superseded (e.g., `captain` when `lucineer-system` exists, `papers` when `SuperInstance-papers` exists)

## GitHub Post-Deletion Cache Lag

After bulk-deleting repos, `gh repo list` and the GitHub API **still return stale totals** for hours. Example: delete 135 repos, the listing still reports `Total: 500`. **Do NOT trust the listing count after deletions.**

**Reliable verification method:** Use individual lookups. `gh repo view org/repo` or `gh repo delete org/repo` both return `HTTP 404: Repository not found` when the repo is actually deleted. If individual lookups return 404, the deletion succeeded — the listing count is just cached.

Workflow after bulk deletion:
1. Run deletion script.
2. Verify success by doing **random sampling** of deleted names with `gh repo view org/name --json name` — all 404 = sweep worked.
3. Do NOT retry or re-run the deletion because the listing still shows the old total.
4. The cache typically resolves within 12-48 hours; no action needed.

## GitHub CLI Auth Scope Requirements

`gh` CLI requires explicit OAuth scopes for destructive operations. Default `gh auth login` grants `repo` scope (read/write for public + private). It does NOT include `delete_repo`.

**Required scopes per operation:**
- `gh repo delete` — requires `delete_repo` scope
- `gh repo create` — requires `repo` scope (included by default)
- `gh auth refresh -h github.com -s delete_repo` — grants the missing scope

**Fix pattern:** When `gh repo delete` returns `HTTP 403: Must have admin rights to Repository. This API operation needs the "delete_repo" scope. To request it, run: gh auth refresh -h github.com -s delete_repo`:
```bash
gh auth refresh -h github.com -s delete_repo
```
This is a one-time local task. After running it, all subsequent `gh repo delete` calls work without prompting.

## Personal Account vs Organization Repos

**CRITICAL:** The user's repos are on a **personal account**, NOT a GitHub Organization. Never assume SuperInstance is an org.

- Personal account repos live at `https://github.com/SuperInstance/repo-name` (SuperInstance is a username, not an org slug)
- Deleted repos are at `https://github.com/settings/deleted-repositories` (NOT the org page)
- Archived repos: go to the user's repo list, click the "Filter" dropdown, select "Archived"

**CLI commands differ for personal accounts:**
```bash
# Restore a deleted personal repo (NO org prefix in the command, but the repo path still has the username)
gh repo restore SuperInstance/fleet-weather

# Unarchive (NOT a flag — it's a separate command entirely)
gh repo unarchive SuperInstance/fleet-weather

# Archive
gh repo archive SuperInstance/fleet-weather -y

# List personal account repos (no --org flag)
gh repo list SuperInstance --limit 500
```

**User correction log:** "this is not an organization. we are a personal account!" — Always verify account type before using org-specific commands like `--org`, org settings URLs, or org bulk operation patterns.

## GitHub CLI Flag Corrections

The user's `gh` version has **strict flag parsing**. Long forms of short flags are rejected:

| Operation | WRONG | RIGHT |
|---|---|---|
| Restore confirmation | `gh repo restore repo --yes` | `gh repo restore repo` (press Y+Enter on prompt) or `gh repo restore repo -y` |
| Unarchive | `gh repo archive repo --unarchive` | `gh repo unarchive repo` |
| Archive confirmation | `gh repo archive repo --yes` | `gh repo archive repo -y` |

**Rule:** When in doubt about `gh` CLI flags, use the bare command and let the interactive prompt handle confirmation. Use `-y` (short form) for programmatic non-interactive mode.

## Reference Files

- `references/git-index-lock-bypass.md` — Windows .git/index.lock unremovable lockfile bypass via gh API
- `references/integration-branch-patterns.md` — force-push, submodule cleanup, and commit conventions
- `references/audit-cleanup-playbook.md` — step-by-step audit, backup-branch, and parallel cleanup pattern used for SuperInstance fleet
- `references/too-many-files-audit.md` — `du -sh` timeout workaround for directories with thousands of small files (Windows)
- `references/github-org-scale-audit.md` — 500-repo GitHub organization audit, 3-tier categorization, and safe deletion pattern
- `references/github-org-repo-restore.md` — 90-day restore process via web UI, post-deletion verification, and prevention guidelines
