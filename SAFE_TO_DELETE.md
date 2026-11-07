# Safe to Delete - Pre-Reformat Cleanup

> Created: 2026-01-09
> Purpose: Items that can be safely deleted before reformat (they exist on GitHub or are rebuildable)

## Projects Backed Up on GitHub

All 83 repositories under `SuperInstance` GitHub account are safe to delete locally:
- Use `gh repo list --limit 200` for complete list
- Can be re-cloned after reformat with `gh repo clone SuperInstance/<repo-name>`

## Node Modules (Rebuildable with `npm install`)

Directories that can be deleted (will be restored by running `npm install`):

```
/mnt/c/users/casey/activelogearlier/deckboss-logger/node_modules
/mnt/c/users/casey/deckboss-logger/amplify/node_modules
/mnt/c/users/casey/deckboss-logger/node_modules
/mnt/c/users/casey/Downloads/workspace-dc770e35-4180-4cb6-b8e7-fd21cfc7b3a7 (1)/node_modules
/mnt/c/users/casey/luciddreamer-os/node_modules
/mnt/c/users/casey/PersonalLog/node_modules
/mnt/c/users/casey/PlayerLoG.AI/node_modules
/mnt/c/users/casey/podcastz/node_modules
/mnt/c/users/casey/smartCDRT backup/demo/node_modules
/mnt/c/users/casey/smartCRMT/demo/node_modules
/mnt/c/users/casey/smartCRMT/node_modules
/mnt/c/users/casey/superinstance-audit/ToolGuardian/node_modules
/mnt/c/users/casey/superinstance-audit/vector-search/node_modules
/mnt/c/users/casey/voice-note-logger/deckboss-app/node_modules
/mnt/c/users/casey/voice-note-logger/node_modules
/mnt/c/users/casey/voice-note-logger/voice-note-app/node_modules
```

**Command to clean all node_modules:**
```bash
find /mnt/c/users/casey -name "node_modules" -type d -prune -exec rm -rf {} +
```

## Build Artifacts (Rebuildable)

### Next.js Build Outputs
```
/mnt/c/users/casey/Downloads/workspace-dc770e35-4180-4cb6-b8e7-fd21cfc7b3a7 (1)/.next
/mnt/c/users/casey/PersonalLog/.next
/mnt/c/users/casey/PlayerLoG.AI/.next
/mnt/c/users/casey/podcastz/.next
/mnt/c/users/casey/podcastz/core-app/.next
```

### TypeScript/JavaScript Dist Outputs
```
/mnt/c/users/casey/holo/holos_toolkit/dist
/mnt/c/users/casey/smartCRMT/demo/dist
/mnt/c/users/casey/superinstance-audit/ToolGuardian/dist
/mnt/c/users/casey/superinstance-audit/vector-search/dist
```

### Python Build Artifacts
```
/mnt/c/users/casey/perception-jepa/build
```

**Command to clean common build directories:**
```bash
find /mnt/c/users/casey -type d \( -name ".next" -o -name "dist" -o -name "build" -o -name "out" \) -prune -exec rm -rf {} +
```

## Duplicate Archives (Downloads Folder)

The Downloads folder contains many duplicate zip files:
- `agent-rpg-phase1.zip`, `agent-rpg-phase1-complete.zip`, `agent-rpg-phase1-complete (1).zip`
- `ai_society_portal.zip`, `ai_society_portal (1).zip`, `ai_society_portal (2).zip`, `ai_society_portal (3).zip`
- Multiple game ROM zips (Bonk's Adventure, Bonk's Revenge, etc.)
- Various other archived projects

**Action**: These can be deleted if projects exist on GitHub or are no longer needed

## Duplicate Backup Folder

```
/mnt/c/users/casey/smartCDRT backup/
```

**Action**: Appears to be a duplicate of `smartCRMT/`. Delete after confirming.

## Cache Directories

### npm cache
- `~/.npm/` - npm cache
- `/mnt/c/users/casey/AppData/Roaming/npm-cache/` (if exists)

### pnpm store
- `~/.pnpm-store/` - pnpm package store

### pip cache
- `~/.cache/pip/` - Python pip cache

### Docker cache
- `/mnt/c/users/casey/AppData/Local/Docker/` - Docker build cache

**Clean command:**
```bash
npm cache clean --force
pnpm store prune
pip cache purge
```

## IDE/Editor Caches

- `~/.vscode-server/` - VS Code Server installation (will reinstall automatically)
- `~/.cursor-server/` - Cursor Server (if exists)
- `~/.cache/` - General application cache

## Docker Containers (Stopped)

Safe to remove stopped containers:
- `openhands-runtime-bd93aa85d0c1433ca363ac7386fbf013`
- `openhands-runtime-b1fe94099f5549d2aa5d5a4fb8f423b3`

**Command:**
```bash
docker container prune
```

## Logs and Temporary Files

- `*.log` files throughout the system
- `/tmp/` contents
- `/var/tmp/` contents
- `~/.local/share/Trash/` - Deleted files waiting to be emptied

## Estimated Space Recovery

| Category | Estimated Size |
|----------|---------------|
| node_modules | ~5-15 GB |
| Build artifacts | ~2-5 GB |
| Cache (npm, pip, docker) | ~2-10 GB |
| Duplicate archives | ~1-3 GB |
| **Total** | **~10-33 GB** |

## Cleanup Commands Summary

```bash
# Clean all node_modules
find /mnt/c/users/casey -name "node_modules" -type d -prune -exec rm -rf {} +

# Clean build artifacts
find /mnt/c/users/casey -type d \( -name ".next" -o -name "dist" -o -name "build" \) -prune -exec rm -rf {} +

# Clean package caches
npm cache clean --force
pnpm store prune
pip cache purge

# Clean Docker
docker container prune
docker image prune -a
docker volume prune

# Clean Python __pycache__
find /mnt/c/users/casey -type d -name "__pycache__" -prune -exec rm -rf {} +
find /mnt/c/users/casey -type f -name "*.pyc" -delete
```

## Warning: DO NOT DELETE

- `/mnt/c/users/casey/.ssh/` - SSH keys
- `/mnt/c/users/casey/.aws/` - AWS credentials
- `/mnt/c/users/casey/.gitconfig` - Git configuration
- `/mnt/c/users/casey/.bashrc` - Shell configuration (after extracting API keys)
- Projects with uncommitted changes
- Any files not in version control
