# GitHub Organization Repo Restore Process

When repos are deleted from a GitHub org (by a human or an agent), they can be restored via the **web UI only**. The GitHub API does **not** support restoration — `gh repo delete` is irreversible through the API.

## The 90-Day Window

GitHub provides a **90-day grace period** after deletion during which repos can be fully restored. After 90 days, deletions are permanent.

## Restore Steps

1. Navigate to: `https://github.com/orgs/<org-name>/repositories`
2. Click the **"Deleted repositories"** tab (near the top, next to "All repositories")
3. You'll see a list of all deleted repos with:
   - Repo name
   - Last updated date
   - Number of commits (if any)
   - Owner who deleted it
4. Click **"Restore"** next to the repo(s) you want back
5. Confirm the restore action

## What Gets Restored

- ✅ All commits and history
- ✅ All branches (including deleted branches)
- ✅ All issues, PRs, and comments
- ✅ All releases and tags
- ✅ All wiki pages
- ✅ All GitHub Actions workflows
- ❌ GitHub tokens (re-authentication may be needed)
- ❌ Third-party integrations (may need re-setup)

## Verification After Restore

After restoring, verify the repo is fully functional:

```bash
gh repo view org/repo-name --json defaultBranchRef,name
gh repo clone org/repo-name /tmp/verify-repo -- --depth 1
git -C /tmp/verify-repo log --oneline -5
```

## What I Learned This Session (2026-08-24)

- **The `gh repo delete` command cannot be undone through the API.** Once executed, repos are gone from the API perspective.
- **GitHub's `gh repo list` cache lags behind actual deletion state.** A CLI listing may still show 500 repos even after deleting 135. This is a server-side cache, not a client issue.
- **Verification must use individual lookups.** `gh repo view org/repo-name` returning 404 is the only reliable way to confirm deletion.
- **The web UI is the ONLY restore mechanism.** No API endpoint, no CLI flag. Users must be warned of this before executing bulk deletes.
- **Agents should NEVER execute bulk deletes without explicit user approval.** The value of deleted repos (even "junk" ones) is determined by the user, not the agent. Each repo represents tokens spent, hard-won architectural decisions, and historical context that is irreplaceable.

## Prevention

To prevent accidental bulk deletions in the future:
- Always present the full list to the user before executing any deletion
- Ask for explicit, written approval ("Approve these X deletions?")
- Never execute deletions in bulk — do them one at a time with verification
- Never call repos "junk", "waste", or "useless" — they are research artifacts
- Preserve the entire history of what was tried, even failed experiments
