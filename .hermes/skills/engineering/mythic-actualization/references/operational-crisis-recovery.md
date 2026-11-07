# Operational Crisis Recovery — Git & Disk Space
2026-08-18

## Overview

When pushing workstreams to repos, you may encounter:
1. **Disk full during git operations** (geometric repack failure)
2. **Remote force-pushed** (local history diverged from origin/main)

This reference documents proven recovery patterns.

---

## Crisis 1: Disk 100% Full During Git Commit

### Symptom

```
fatal: sha1 file '.git/objects/pack/tmp_pack_GYXzw2' write error. Out of diskspace
error: failed to perform geometric repack
```

Git's `geometric repack` runs during commit and needs temporary space. If disk is >99% full, this fails.

### Large Caches to Check

| Location | Typical Size | Clean Command |
|----------|--------------|---------------|
| `~/.cache/puppeteer` | 1-2 GB+ | `rm -rf ~/.cache/puppeteer/*` |
| `pip cache` | 5-10 GB+ | `pip cache purge` |
| `~/.npm/_cacache` | Variable | `npm cache clean --force` |
| `~/.cache/huggingface` | Variable | `rm -rf ~/.cache/huggingface/hub/*` |
| `~/AppData/Local/Temp` | Variable | `rm -rf ~/AppData/Local/Temp/*` |

### Proven Recovery (2026-08-18)

```bash
# BEFORE: 953G used, 0 available

# Step 1: Clean puppeteer cache
rm -rf ~/.cache/puppeteer/*
# Freed: ~1.4 GB

# Step 2: Clean pip cache (this is the BIG one)
pip cache purge
# Output: "Files removed: 3605 (7280.3 MB)"
# Freed: ~7.3 GB

# AFTER: 940G used, 13G available

# Now git commit works
git commit -m "..."
```

### Why This Happens

Git's geometric repack is triggered automatically during commit when there are many loose objects. It needs:
1. Space to read all existing pack files
2. Space to write new combined pack files
3. Space for temporary files during the operation

Even if the commit itself is tiny, the repack needs working space.

---

## Crisis 2: Remote Force-Pushed (Diverged History)

### Symptom

```
! [rejected]            main -> main (fetch first)
error: failed to push some refs to '...'
hint: Updates were rejected because the remote contains work that you do
hint: not have locally. This is usually caused by another repository pushing to
hint: the same ref.
```

Then after `git fetch`:
```
 + dc58cef20...9dd086952 main       -> origin/main  (forced update)
```

**The remote was force-pushed. Your local commits are on a diverged path.**

### What You'll See

```
Local main:      A -- B -- C -- YOUR_COMMIT
Remote main:     A -- B -- D -- E (different history after B)
```

### Wrong Approaches to Avoid

| Approach | Why It's Wrong |
|----------|----------------|
| `git pull --rebase` | Will rebase your commits onto the new history, potentially creating merge conflicts and losing context |
| `git push --force` | Will OVERWRITE the remote's new history with yours — very dangerous if others are working on the new history |
| `git reset --hard origin/main` | Will DELETE your local commits entirely |

### Proven Recovery (2026-08-18)

**Push your work to a NEW branch, then create a PR:**

```bash
# See the divergence
git log --oneline HEAD ^origin/main  # Your local-only commits
git log --oneline origin/main ^HEAD  # Remote's new commits

# Option 1: Rename your current branch and check out origin/main
git branch -m main old-cudaclaw-with-hermes-residual
git checkout -b main origin/main

# Option 2: Easier — just push your HEAD to a new branch
git push origin HEAD:refs/heads/hermes-residual-self-image-2026-08-18
```

**Output:**
```
 * [new branch]          HEAD -> hermes-residual-self-image-2026-08-18
```

### When to Use Which

| Scenario | Recommended Approach |
|----------|----------------------|
| You have valuable local commits | Push to new branch + create PR |
| Remote force-push was accidental/mistake | Discuss with team, maybe force-push your version |
| Your local commits are obsolete/already integrated | Reset to origin/main |

---

## Prevention Patterns

### Disk Space

1. **Before large commits/pushes**, check disk space:
   ```bash
   df -h
   ```

2. **Add to .gitignore** files that shouldn't be tracked:
   - Large cache directories
   - Build artifacts
   - Node modules, venv, etc.

3. **Periodically clean** large caches:
   ```bash
   pip cache purge
   rm -rf ~/.cache/puppeteer/*
   ```

### Git History

1. **Avoid force-pushing to shared branches** (main, develop)
2. **Use feature branches** for experimental work
3. **Before pushing**, check if remote has moved:
   ```bash
   git fetch origin
   git log --oneline HEAD..origin/main  # What's on remote that you don't have
   ```

---

## Summary Table

| Crisis | Symptom | Proven Fix |
|--------|---------|------------|
| Disk full during commit | `Out of diskspace` + geometric repack error | `pip cache purge` + `rm -rf ~/.cache/puppeteer/*` |
| Remote force-pushed | `(forced update)` in fetch output | Push to new branch: `git push origin HEAD:refs/heads/your-branch-name` |
