# Personal vs Organization Account Paths

Decision tree for GitHub repo management operations.

```
Is the repo under an organization or personal account?
  │
  ├── ORGANIZATION (github.com/orgs/<name>/)
  │   ├── Archive:       gh repo archive --yes org/<repo>
  │   ├── List:          gh repo list <org> --limit 500
  │   ├── Restore:       Web UI only → org/repositories → Archived tab → Unarchive
  │   └── Delete restore: Web UI only → org/repositories → Deleted tab → Restore
  │
  └── PERSONAL (github.com/<username>/)
      ├── Archive:       gh repo archive --unarchive <user>/<repo>
      │   │                      (gh repo archive without --unarchive archives)
      ├── List:          gh repo list <username> --limit 500
      ├── Restore:       gh repo restore <user>/<repo> --yes
      │   │                    or Web UI → github.com/settings/deleted-repositories
      └── Unarchive:     Open repo → Settings → Unarchive
                          or gh repo archive --unarchive <user>/<repo>
```

## Key Differences

| Action | Organization | Personal Account |
|--------|-------------|-------------------|
| Archive | `gh repo archive --yes org/r` | `gh repo archive user/r` |
| Unarchive | Web UI only | `gh repo archive --unarchive user/r` |
| Restore deleted | Web UI only | `gh repo restore user/r --yes` or Web UI |
| Deleted tab | org/repositories → Deleted tab | settings/deleted-repositories |
| Archived filter | org/repositories → Archived tab | Filter dropdown on user repos page |

## Session 2026-08-24 Lesson

User's repos are under `SuperInstance` but this is a **personal profile**, not an organization. The org management paths (org settings tabs, org-level `gh repo list`) were incorrect. Correct paths are under `github.com/settings/deleted-repositories` for restores and the user profile filter for archiving.
