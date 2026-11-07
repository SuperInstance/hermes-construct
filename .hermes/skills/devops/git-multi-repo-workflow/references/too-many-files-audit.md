# Too-Many-Small-Files Audit Timeout

On Windows (MSYS2/bash), `du -sh <dir>` can hang/timeout on directories with thousands of small files (e.g., AI-Writings with 1229 .md files) because each file requires a stat() syscall.

## Symptom
`du -sh` or `find ... -printf '%s\n'` on the same directory times out after 10-30s.

## Workarounds

**Use `wc -c` on glob patterns instead of `du` on directories:**
```bash
# Fast — uses readdir stat batch
wc -c /c/Users/casey/projects/AI-Writings/*.md
# total line gives aggregate size

# For subdirs only
wc -c /c/Users/casey/projects/AI-Writings/experiments/*
```

**Use `stat -c '%s'` per-file in a loop:**
```bash
# Faster than du for large flat directories
find /path -maxdepth 1 -type f -exec stat -c '%s %n' {} \; | awk '{s+=$1}END{print s/1024/1024 " MB"}'
```

**When `du` does work, limit depth:**
```bash
# du -sh /c/Users/casey/... works fine for top-level dirs
# The issue is deep recursive traversal of directories with many files
```

## Data from Session (2026-08-24)
- `du -sh` on AI-Writings (1229 files) → timeout
- `wc -c *.md` on AI-Writings → 9,692,215 bytes (9.7 MB)
- This confirmed the repo is tiny on disk despite the large file count
