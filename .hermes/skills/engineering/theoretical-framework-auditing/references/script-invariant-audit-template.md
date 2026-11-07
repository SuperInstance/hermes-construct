# Script Invariant Audit Template

Use when auditing scripts (image generators, API wrappers, automation) that handle credentials, paths, or external calls.

## Invariant Table

| # | Invariant | Status | Evidence |
|---|-----------|--------|----------|
| I-1 | No hardcoded API tokens / secrets | FAIL / PASS | Exact line showing hardcoded value or env var lookup |
| I-2 | No hardcoded absolute output paths | FAIL / PASS | Exact `OUT = "..."` line or configurable path |
| I-3 | Portable location resolution | FAIL / PASS | Whether paths resolve on host working directory vs fixed absolute paths |
| I-4 | Graceful failure on missing credentials | FAIL / PASS | Whether empty token triggers explicit abort or proceeds to 401 |

## Audit Output

Produce `docs/GEN_<SCRIPT>_AUDIT.md` with:
1. **Summary** — one-line risk classification (HIGH/MEDIUM/LOW)
2. **Invariant Findings** — table as above
3. **Additional Risks** — timeouts, integrity checks, deprecated dependencies
4. **Required Fixes** — numbered action items

## Script Patch Requirements

When patching a script that failed I-1 through I-4:
1. Replace hardcoded tokens with `os.environ.get("<VAR>", "")`
2. Replace hardcoded output paths with `os.environ.get("<VAR>", "<default>")`
3. Add early abort: `if not TOKEN: raise SystemExit("Missing credentials...")`
4. Add integrity verification (e.g., PNG magic bytes: `b"\x89PNG\r\n\x1a\n"`)
5. Preserve existing fallback lookup logic (`.bashrc`, config files) but **never commit loaded values**

## Repository Cross-Reference Pattern

When the referenced commit is not in any local branch:
1. `git show <commit>:<path>` to extract the file without checkout
2. If repo is not cloned: `gh repo clone <org>/<repo> -- --depth 1` first
3. Audit the extracted file, then commit the audit doc to the repo root `docs/`
