# SuperInstance Gitlink Workaround

## Problem
The `polln/` monorepo tracks child repositories (`claw/`, `claw-extensions/`, etc.) as 160000 gitlinks in `git ls-files --stage`, without a corresponding `.gitmodules` entry.

Symptoms:
- `git rev-parse --show-toplevel` from `/c/Users/casey/polln/claw` returns `C:/Users/casey/polln`
- `git -C /c/Users/casey/polln/claw add ...` → `fatal: in unpopulated submodule 'claw'`
- `write_file` with POSIX `/c/Users/...` resolves to `C:\c\Users...` producing ghost writes

## Verified Workaround
1. Clone upstream repo to temp worktree:
   ```bash
   rm -rf /tmp/claw_work
   git clone https://github.com/SuperInstance/claw.git /tmp/claw_work
   ```
2. Perform all writes/commits/pushes in the clean clone.
3. Use `clean-publish` branch for onboarding atoms.
4. If mirroring back to `polln/`, copy files via `cp` from worktree to `polln/claw/` after push.

## Path Safety
- Prefer `terminal` with `cat > /c/Users/casey/...` over `write_file` for POSIX paths on Windows hosts.
- Always `ls -l` immediately after write.
- Verify workspace root is `C:\Users\casey`, not `C:\c\Users`.

Recorded 2026-08-13 during claw-schema onboarding push.
