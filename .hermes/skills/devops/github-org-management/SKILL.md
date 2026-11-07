---
name: github-org-management
description: Safe management of GitHub organizations — archiving, cleanup, pagination traps, and bulk operations with preservation-first semantics.
---

# GitHub Repository Management

Safe bulk operations on GitHub **organizations** and **personal accounts** — archiving, cleanup, pagination traps, and bulk operations with preservation-first semantics.

**Before acting, always confirm whether the repos are under an organization or a personal account.** This changes every navigation path and restore URL.

## Core Principle: Archive Over Delete

**When the user says "clean up" or "remove repos": archive them instead.**

- Archived repos are fully restorable with history intact (one click on the web UI).
- Deleted repos have a 90-day undelete window but often require manual web UI navigation.
- Never assume "junk" — the user's history is research. All project directories are valuable tokens spent and hard-won architectural decisions.

**Archive command:** `gh repo archive --yes SuperInstance/<repo>`

**Archive-all-inclusive:** The `--yes` flag is mandatory in non-interactive contexts (scripts, terminal sessions). Without it, `gh repo archive` prompts for confirmation and hangs/fails silently.

## Workflow

### 1. Discover What Exists

```bash
# List all repos (may have pagination)
gh repo list SuperInstance --limit 500 --json name,description,isPrivate

# Parse to understand state
gh repo list SuperInstance --limit 500 --json name,description,isPrivate,isArchived
```

**WARNING:** `gh repo list` may show stale counts after deletions. GitHub's CLI caches the count (e.g., still showing 500 after deletes). Always verify individual repos:

```bash
gh repo view SuperInstance/<repo> 2>&1
# Returns 0 (success) if alive, 1 with "Could not resolve" if deleted
```

### 2. Categorize Before Acting

Before any bulk operation, categorize repos into:
- **ARCHIVE** — Most repos (saves history, reversible)
- **DELETE** — Only if explicitly confirmed (empty placeholders, test scaffolds)
- **KEEP** — Active fleet repos

### 3. Archive Operation

```bash
# Single repo
gh repo archive --yes SuperInstance/<repo>

# Batch archive (shell loop)
for repo in "repo1" "repo2" "repo3"; do
  gh repo archive --yes "SuperInstance/$repo"
done
```

Check return code to determine outcome:
- **0 (success)** — Archived successfully
- **Already archived** — "already archived" in output (skip)
- **Not found** — Was already deleted (404) — needs manual web UI restore
- **Other error** — Log and investigate

### 4. Restore From Archive

Archived repos are restored via web UI:
1. Go to `https://github.com/orgs/<org>/repositories`
2. Filter by "Archived" tab
3. Click "Unarchive" next to the repo

### 5. Restore From Deletion

Deleted repos are restored via web UI:
1. Go to `https://github.com/orgs/<org>/repositories`
2. Click "Deleted repositories" tab
3. Click "Restore" next to the repo
4. GitHub gives 90 days for restoration

**CRITICAL:** The `gh repo archive` command CANNOT restore deleted repos. Once deleted, only the GitHub web UI can restore.

## Personal Account Repos

If repos are on a **personal profile** (not an org), paths differ:

**Restoring deleted repos (personal):**
1. Go to `https://github.com/settings/deleted-repositories`
2. Click **Restore** next to the repo

**Unarchiving (personal):**
1. Go to `https://github.com/<username>?tab=repositories`
2. Use the **Filter** dropdown → select **Archived**
3. Open the repo → Settings → scroll to bottom → **Unarchive**

**CLI (personal account):**
```bash
# Restore a deleted repo (personal)
gh repo restore <username>/<repo> --yes

# Unarchive
gh repo archive <username>/<repo> --unarchive
```

**Listing personal repos:**
```bash
gh repo list <username> --limit 500 --json name,isPrivate,isArchived
```

**Key difference:** Personal accounts do NOT have a "Deleted repositories" tab on the main repos page. Deleted repos are only accessible via `settings/deleted-repositories`. The `gh repo restore` command works for personal accounts but NOT for orgs (org undelete requires web UI).

## Pitfalls

### Pitfall: `--yes` Flag Required
`gh repo archive` requires `--yes` in non-interactive scripts. Without it, the command hangs waiting for terminal confirmation and silently fails.

### Pitfall: Stale `gh repo list` Count
After bulk operations, `gh repo list` may continue showing the old total count. This is a GitHub API caching issue. Always verify individual repos with `gh repo view`.

### Pitfall: Already Deleted Repos
If a repo was previously deleted, `gh repo archive` will return 404. The command cannot restore a deleted repo. Check for "already archived" (success) vs "Could not resolve" (already gone).

### Pitfall: API Scoped Permissions
`gh api orgs/<org>/deleted-repos` returns 404 unless the token has `admin:org` scope. Standard `repo` scope is insufficient.

### Pitfall: Re-creating vs Restoring
If a deleted repo is "restored" via `gh api repos/<org>/<repo>` with `PUT`, it creates a **new empty repo** — all history is lost. Always use the web UI for restoration.

### Pitfall: Personal vs Organization Account
The user's repos may be on a **personal account**, not an organization. If you see paths like `github.com/<username>/<repo>` or `settings/deleted-repositories`, this is a personal account — the org paths (`orgs/<org>/repositories`) will not work. Always confirm the account type before giving navigation instructions. The `gh repo restore` command only works for personal accounts, not orgs.

## Verification

After any bulk operation, verify:

```bash
# Count current repos
gh repo list SuperInstance --limit 500 --json name | python3 -c "import sys,json; print(f'Total: {len(json.load(sys.stdin))}')"

# Verify specific repos
for repo in "active-repo-1" "active-repo-2"; do
  gh repo view "SuperInstance/$repo" &> /dev/null && echo "✅ $repo" || echo "❌ $repo MISSING"
done
```

## Related

- See `references/api-quirks.md` for detailed API scope and pagination notes
- See `references/account-type-paths.md` for personal vs organization decision tree and correct restore URLs
- See `templates/batch-archive.sh` for ready-to-use archive scripts
- See `scripts/verify-fleet.sh` for active repo verification
