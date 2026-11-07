# GitHub Post-Deletion Cache Lag

## The Problem

After bulk-deleting repos with `gh repo delete`, the GitHub API and `gh repo list` **still return stale totals** for hours (sometimes 12-48 hours). The listing count does not reflect actual deletions immediately.

**Symptom:** You just deleted 135 repos. `gh repo list SuperInstance` still shows `Total: 500`. Individual `gh repo view SuperInstance/xxx` returns `HTTP 404: Repository not found`.

## Root Cause

GitHub's pagination/listing endpoint caches the total count server-side. The count is not recomputed after write operations — it updates on a periodic cache refresh cycle. This is a known GitHub API behavior.

## Verification Pattern

Never use the listing total to verify deletion success. Use individual lookups instead:

```bash
# REPLACEMENT FOR: gh repo list SuperInstance (trusts stale cache)
# ACTUAL VERIFICATION:
gh repo view SuperInstance/<repo-name> --json name 2>&1
# Expected: "GraphQL: Could not resolve to a Repository" + HTTP 404

# Or attempt delete (also returns 404 on already-deleted repos):
gh repo delete SuperInstance/<repo-name> --yes 2>&1
# Expected: "Repository '<name>' not found" + HTTP 404
```

**Sampling strategy for bulk operations:** Pick 10-20 random repo names from the deletion list and verify each returns 404. If all sample repos are 404, the sweep succeeded even if the listing count is stale.

## Cache Resolution Timeline

| Time After Deletion | Listing Count | Individual Lookups |
|---------------------|---------------|-------------------|
| 0-4 hours           | Stale (old)   | Accurate (404)    |
| 4-12 hours          | May be stale  | Accurate (404)    |
| 12-48 hours         | Typically fresh | Accurate (404) |
| >48 hours           | Fresh         | Accurate (404)    |

**No action needed** — the cache resolves itself. Do NOT re-run deletion scripts.

## Session Example (2026-08-24)

Sweep: 135 repos deleted via bash script.
- Script output: individual `gh repo delete` commands returned various results
- Post-sweep verification: `gh repo view SuperInstance/fishinglog-ai` → 404 (correct — repo was deleted)
- `gh repo list` still showed ~500 repos (stale cache, expected behavior)
- Conclusion: delete sweep succeeded, cache will resolve naturally

## Key Takeaway

**Trust 404s, not listing counts.** The listing total is a cache metric, not a truth source. Individual lookups are the only reliable post-deletion verification.
