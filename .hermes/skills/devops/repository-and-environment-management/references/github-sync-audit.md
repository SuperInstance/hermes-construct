# GitHub Sync Audit — Multi-Clone Fleet

## Context
User request: "I want you to get everything on GitHub". Fleet lives on Windows with MSYS/bash. 53 local clones under C:\Users\casey, many duplicates, temp clones, and a monorepo work-tree masquerading as independent repos.

## Key Discoveries
- `polln/` is a single git repository whose `.git` is at `/c/Users/casey/polln/.git`. All subpaths like `/c/Users/casey/polln/claw`, `/c/Users/casey/polln/spreadsheet-moment`, `/c/Users/casey/polln/knowledge-graph`, etc., share the same git dir and show-top-level. They must not be audited/pushed individually.
- Duplicates: `/c/Users/casey` and `/c/Users/casey/AppData/Local/Temp/hermes-clone` both point to `SuperInstance/hermes-construct.git`
- Sandbox copies: `/c/Users/casey/sandbox/fv-annika/boat-agent`, `/c/Users/casey/sandbox/fv-northstar/boat-agent`, etc.
- Folder-inside-repo confusion: Paths named `open-mind`/`open-terminal` actually point to `SuperInstance/plato-portal.git`

## Detection Script Pattern
```bash
while IFS=| read -r path; do
  git_dir=$(git -C "$path" rev-parse --git-dir 2>/dev/null)
  top=$(git -C "$path" rev-parse --show-toplevel 2>/dev/null)
  remote=$(git -C "$path" remote get-url origin 2>/dev/null)
  echo "$path|$remote|$git_dir|$top"
done < repo_list.txt | sort -k3
```
Group by `git_dir` to find shared work-trees.

## Sync Classification
Using `git fetch` then comparing HEAD vs origin/<branch>:
- synced
- behind N
- ahead N
- diverged

Example findings:
- `/c/Users/casey/claw` → SuperInstance/claw.git master → synced
- `/c/Users/casey/pincher` → pincher.git main → behind 2
- `/c/Users/casey/hermes-roblox-construct` → synced
- `polln/*` subdirs → shared git-dir → skip individual audit

## Action Rules
- Do not issue individual `git push` for paths sharing a git-dir.
- Do not delete canonical clones; mark temp/duplicates for user confirmation.
- When behind, pull only if user confirms active use.
- When ahead, push to a new named branch first, never force-push without explicit instruction.
- Always use `git -C "<path>"` on Windows/MSYS to avoid cd context drift.

## Pitfalls Avoided
- Misinterpreting subdirectory paths as independent repos
- Wasteful fetches on shared work-trees
- Accidental force-push on diverged history
