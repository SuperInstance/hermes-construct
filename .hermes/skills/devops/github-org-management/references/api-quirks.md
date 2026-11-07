# GitHub Org API Quirks

## Scopes

| Scope | Access |
|-------|--------|
| `repo` (default) | Read/write repos, limited org admin endpoints |
| `admin:org` | Full org management including `deleted-repos` listing |
| `delete_repo` | Required for `gh repo delete` |
| `read:org` | List org members, teams |

**Key finding:** `gh api orgs/<org>/deleted-repos` returns 404 with standard `repo` scope. Requires `admin:org`.

## Pagination

`gh repo list` fetches all repos up to `--limit`. GitHub returns max 100 per page by default. With `--limit 500`, the CLI auto-paginates.

However, the **count shown** may be stale after deletions because GitHub caches the total org count on the list endpoint. Individual `gh repo view` calls hit a different (real-time) endpoint.

## Rate Limits

- 5000 requests/hour for authenticated requests
- `gh repo view` per repo is 1 request
- Bulk verification of 200 repos = 200 API calls (consumes 4% of hourly budget)

## Restore Methods

| State | RESTORE METHOD |
|-------|----------------|
| Archived | Web UI → Repositories → Archived tab → Unarchive |
| Deleted | Web UI → Repositories → Deleted repositories tab → Restore (90-day window) |
| Both | Cannot restore via API — web UI only |

## Deleted vs Archived Behavior

- **Archive:** Repo still exists, code is read-only, wiki/pages enabled. Can be unarchived anytime.
- **Delete:** Repo is soft-deleted for 90 days. After that, it's permanently destroyed.
- **Webhook cleanup:** GitHub fires `repository.deleted` webhook events.

## Caching Notes

GitHub's `gh repo list` total count is not reliable post-delete. Always use `gh repo view` for verification, or parse the actual JSON response length rather than trusting the count number.
