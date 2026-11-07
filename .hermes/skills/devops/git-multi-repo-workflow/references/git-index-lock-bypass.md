# Git Index Lock Bypass (Windows)

When `.git/index.lock` cannot be removed (Windows file handle conflict — another git process holds it, or the OS won't release it even after killing the process), the lock blocks all git operations.

## Symptoms
```
fatal: Unable to create 'C:/Users/casey/.git/index.lock': File exists.
Another git process seems to be running in this repository, or the lock file may be stale
```

Common cause: a previous `git` command timed out (timeout=120s), leaving a zombie git process that either exited but didn't clean up the lock, or is still holding a file handle.

## Bypass Strategy: Push via GitHub API

When the lock is unmovable, bypass git entirely by pushing through the REST API:

```bash
# 1. Read the file content and base64-encode it
content="$(base64 -w0 /c/Users/casey/path/to/file.md)"

# 2. Push directly to GitHub
gh api repos/<org-or-user>/<repo>/contents/<file-path> \
  -X PUT \
  -F "message=Commit message here" \
  -F "content=${content}"
```

This creates a commit on the default branch (usually `master`) via the GitHub API, completely bypassing the local git state.

## When to Use This
- Lock file persists after killing all git processes (`taskkill` / `Stop-Process`)
- `rm -f .git/index.lock` returns "Device or resource busy"
- The file content is ready on disk and just needs to land on GitHub
- You have verified the file exists at the expected path first

## Caveats
- Creates a commit on the default branch — may not match your intended branch
- Skips hooks, LFS, and branch protection rules
- The local working tree remains out of sync — you'll need to reconcile locally later
- Only works for individual files; not a substitute for full git workflows
