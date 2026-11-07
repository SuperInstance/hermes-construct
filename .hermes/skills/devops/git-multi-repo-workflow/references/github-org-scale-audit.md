# GitHub Organization-Scale Repository Audit & Triage

Applies when the GitHub org has 200+ repos and local auditing is infeasible.

## Workflow

### 1. Fetch full org inventory
```bash
gh repo list <org> --limit 500 --json name,description,updatedAt,pushedAt > /tmp/all_repos.json
```

Note: GitHub caps `--limit` at 500 per call. For orgs beyond 500, paginate via the REST API directly:
```bash
curl -s "https://api.github.com/orgs/<org>/repos?per_page=100&page=1" -H "Authorization: token $GITHUB_TOKEN"
```

### 2. Categorize via inline Python
```bash
gh repo list <org> --limit 500 --json name,description,updatedAt,pushedAt | python3 -c "
import json, sys
data = json.load(sys.stdin)
# ... categorization logic ...
"
```

### 3. Three-Tier Categories

**DEL1 — Immediate Kill**
- Auto-created sync artifacts (description contains "Auto-created for sync")
- Wrong org contents (e.g., Chinese-language repos in an English org, unrelated projects)
- Dead 3rd-party integrations that are no longer relevant

**DEL2 — Low Risk**
- Empty repos (no description AND no recent activity)
- SI-prefix duplicates (e.g., `si-knowledge-graph` when `knowledge-graph` exists)
- Deprecated/moved repos (description mentions "MOVED", "MOVED TO")
- Superseded naming conventions (`captain` when `lucineer-system` exists, `papers` when `SuperInstance-papers` exists)
- Single-use experiments (`GeometryPainterThreeJS`, `scummvm-gui-design`, `roblox-builder-kit` without description)

**REVIEW — Needs Human**
- Repos with descriptions that need context to decide
- Old fleet work that may still be referenced
- Active-sounding repos that haven't been updated in 30+ days

### 4. Deletion
```bash
gh repo delete <org>/<repo> --yes
```

If this returns `HTTP 403: Must have admin rights to Repository.`, grant the scope first:
```bash
gh auth refresh -h github.com -s delete_repo
```

### 5. Report
Return the counts: DEL1 deleted, DEL2 deleted, REVIEW count requiring human input.
