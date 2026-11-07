# Git filter-repo purge example

Used to remove node_modules from history before pushing to GitHub.

```bash
cd /c/projects
git filter-repo --path node_modules --invert-paths --force
# origin remote is removed by filter-repo, re-add it
git remote add origin https://github.com/SuperInstance/projects.git
git push -f origin master
```

Notes:
- filter-repo removes origin remote intentionally
- Always verify remote HEAD after force push
- Do this before creating the safety backup branch
