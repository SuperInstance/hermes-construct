---
name: repo-doc-integration-workflow
description: Workflow for creating, documenting, committing, and pushing repository changes with user‑specific preferences (direct tone, maritime units, Git safety net, parallel terminals, verification).
category: documentation
---

## Overview
This skill codifies the end‑to‑end process for adding documentation or code artifacts to a repository while respecting the user’s operational preferences.

### Core Steps
1. **Prepare content** – Write markdown/MD files using a direct, honest voice; avoid filler. When referencing maritime data, use DMS, NM, or fathoms.
2. **Add reference files** – If the task involves user preferences, include `references/user_preferences.md` (see companion reference).
3. **Stage changes** – `git add <paths>`.
4. **Commit** – Use a concise message, e.g. `git commit -m "hermes: <brief description>"`.
5. **Safety net** – Before any destructive operation (e.g., branch deletion), push a backup branch:
   ```bash
   git checkout -b backup-$(date +%F)
   git push origin backup-$(date +%F)
   ```
6. **Push** – `git push origin <branch>`.
7. **Create PR (optional)** – If a PR is desired, run:
   ```bash
   gh pr create --title "<title>" --body "<body>"
   ```
   The PR URL is returned for user reference.
8. **Notify** – Use `notify_on_complete` for long‑running git commands (e.g., large pushes) to keep the user informed.

### Verification
- After each git command, capture and display the output.
- Ensure the exit code is `0`; on failure, surface the error and suggest a retry.
- Confirm that any created files appear in the repository tree (`git ls-files`).

## User Preferences (Captured from Session)

- **Act first when directed:** If user says "go further", "synergize", "just do it", or "generate more great assets" — skip confirmation prompts. Produce artifacts immediately, push immediately.
- **Audit-and-pivot default:** When asked "find where you can help most" or "study recent pushes", treat it as a directive to: (1) enumerate fleet repos, (2) inspect recent commits, (3) identify high-impact gaps aligned with your role, (4) patch and push immediately, (5) report the delta. Do not ask which repo to start with.
- **Direct, honest voice:** No corporate filler, no "Great question!". Report findings plainly.
- **Maritime units:** Use DMS for position, NM for distance, Fathoms for depth. Only when applicable to the domain.
- **Git safety net:** Before destructive ops, push a backup branch. For diverged/ahead work, push `local-backup-YYYY-MM-DD`.
- **Parallel terminals:** Prefer `notify_on_complete=true` on background commands.
- **No fabricated output:** Report real git exit codes, show actual file contents, never substitute plausible-looking fake data.

## Archiving Creative Output
- Any generated creative artifacts (e.g., images, audio, markdown) should be copied to `~/ai-writings/` and referenced in the documentation.
- Include a `MEDIA:` tag in the final message so Telegram delivers the file natively.

## Semantic Push Pattern (NEW)

When a multi-repo synchronization is in progress:
1. Push high-value audit/analysis artifacts to origin as soon as they're written — don't wait for full completion.
2. A partial artifact on origin is worth more than a "complete" artifact on local disk because the fleet reads from GitHub.
3. Sequence for multi-repo work: Audit → push audit docs → populate schemas → push populated → verify live.

## References
- `references/user_preferences.md` – detailed user preferences used by this workflow.

## Pitfalls & Tips
- **Avoid ambiguous commit messages** – be explicit about what changed.
- **Do not delete branches without first pushing a backup**.
- **When using parallel terminals**, add `notify_on_complete=true` to the `terminal` call.
- **Never fabricate output** – always show real `git` command results.

---
