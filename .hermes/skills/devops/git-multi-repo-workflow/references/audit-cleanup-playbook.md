# SuperInstance Fleet Audit & Cleanup Playbook

## Context
Windows workstation with many SuperInstance clones under `/c/Users/casey`. Goal: ensure everything is backed up to GitHub, eliminate duplicates, reclaim disk, never lose local work.

## Steps Used

### 1. Inventory
```
find /c/Users/casey -maxdepth 5 -type d -name .git -print | sed 's|/.git||'
```
Group by `git rev-parse --git-dir` and remote URL.

### 2. Classify
For each repo:
```
git -C "<path>" fetch origin
ahead=$(git -C "<path>" rev-list --count origin/main..HEAD)
behind=$(git -C "<path>" rev-list --count HEAD..origin/main)
dirty=$(git -C "<path>" status --porcelain | wc -l)
```
Categories: synced, behind, ahead, diverged, dirty.

### 3. Preserve Diverged Work
Before any deletion, push local commit graph to GitHub as backup branch:
```
git -C "<path>" branch -f local-backup-2026-08-20
git -C "<path>" push -u origin local-backup-2026-08-20:refs/heads/local-backup-2026-08-20
```
Verified for: casting-call, cudaclaw, conservation-enforcer-rs, polln.

### 4. Parallel Operations
Use background terminals with notify_on_complete:
* Pull fast-forward for behind repos you keep
* Push backup branches for diverged repos
* rm -rf for safe duplicates / temp / sandbox

### 5. Cleanup Targets
Safe to delete after verification:
* Sandbox copies: sandbox/fv-annika/*, sandbox/fv-northstar/*
* Temp clones: AppData/Local/Temp/hermes-clone, temp/Lucineer
* Synced clones with zero ahead/behind/dirty: activelog-ai, boat-agent, hermes-avatar, perception-cascade, provenance-log, trinity-agent, tzpro-agent, vessel-agent, vessel-quest, hermes-roblox-construct, hermit-crab
* Tiny _si_research clones fully mirrored on GitHub

## Pitfalls Encountered
* polln subdirs share single .git at /c/Users/casey/polln/.git with remote SuperInstance/claw.git. Cannot audit individually.
* DMLog backup failed: fatal: not a valid object name 'main' — repo has no main branch locally.
* Permission denied on /c/Users/casey/AppData/Local/CoreAIPlatform.00/UKP during find.

## Outcome
All diverged work backed up to GitHub as local-backup-2026-08-20 branches. Multiple GB reclaimed by removing duplicates and synced clones.
