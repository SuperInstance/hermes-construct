# GitHub Destructive Operations — Critical Limitations

## What You Cannot Undo via API

**The `gh` CLI and GitHub REST API cannot restore deleted repositories.** Once a repo is deleted:
- `gh api repos/owner/repo` returns 404
- `gh repo view owner/repo` returns 404
- **There is NO API endpoint to restore**

The only restoration path is the **GitHub web UI**:
1. Go to `https://github.com/orgs/<org>/repositories`
2. Click the **"Deleted repositories"** tab
3. Click **"Restore"** next to each repo
4. Add a comment explaining why it was deleted (required by GitHub)

**Time limit:** GitHub retains deleted repos for **90 days** before permanent purge.

## What You Cannot Undo via API (Beyond Repos)

- **Branch deletion:** Force-pushing over a deleted branch does NOT restore it. The branch is gone from the ref log. You can only recover if there's a local clone or a pull request still referencing it.
- **Organization repository settings:** Rate limits on org-level operations (1000 mutations/hour for orgs). Deleting 100+ repos will hit rate limits.
- **GitHub Pages:** Deleting a repo with a Pages site destroys the published site. Restoring does NOT re-publish automatically — you must manually re-enable Pages in Settings.

## Safety Protocol (MANDATORY)

Before ANY destructive GitHub operation:

1. **List what will be deleted** — show the user the exact names
2. **Explain consequences** — "These repos contain X commits across Y branches"
3. **Ask for explicit approval** — "Confirm you want me to delete these 13 repos?"
4. **Verify local copies exist** — `cd /path && git log --oneline -1` for each repo before deleting from GitHub
5. **Execute in small batches** — delete 5-10 at a time, verify between batches, do not batch-delete 100+ repos
6. **Document what was deleted** — create a `DELETION_LOG.md` in a persistent repo for audit trail

## Common Misconceptions

| Belief | Reality |
|--------|---------|
| "gh repo delete is reversible" | No API restore — web UI only |
| "Force push restores deleted branch" | No, ref log entry is gone |
| "Trash repos survive 90 days" | Permanent after 90 days |
| "Deleting a fork deletes the original" | No, only the fork copy |
| "Archived repos can't be deleted" | Yes they can, archival just hides them from default listing |
