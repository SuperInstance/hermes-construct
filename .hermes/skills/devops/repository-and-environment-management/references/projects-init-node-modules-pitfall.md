# C:\projects Git init node_modules pitfall - 2026-08-20

## Context
User requested to make C:\projects a Git repo and push to GitHub to save space. Directory contained `node_modules/`, `package.json`, `package-lock.json`.

## What happened
Agent ran:
```
git init
git add .
git commit -m "Initial commit of C:\projects"
```
Result: entire `node_modules` tree was staged and committed. Commit output showed thousands of files being added, leading to massive repo size and slow operations.

## Remediation applied
1. Created `.gitignore` with `node_modules`
2. Removed node_modules from tracking:
```
git rm -r --cached node_modules
git add .gitignore
git commit -m "Remove node_modules from tracking"
```
3. Created GitHub repo and pushed:
```
gh repo create SuperInstance/projects --public --source=. --push
```

## Lesson
Always create `.gitignore` before first `git add .` when initializing a repo from an existing workspace. For Node workspaces, `node_modules/` must be ignored. If already committed, use `git rm -r --cached` to untrack without deleting local files.

## Reference
Related skill section: Pitfalls & Mitigations → Git init with bundled dependencies
