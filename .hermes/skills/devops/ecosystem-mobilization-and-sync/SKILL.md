---
name: ecosystem-mobilization-and-sync
description: Governs the high-fidelity distribution, synchronization, and verification of documentation, schemas, and architectural context across a distributed, multi-repository ecosystem.
---

# Ecosystem Mobilization & Synchronization

Governs the movement, synchronization, and verification of documentation, schemas, and architectural context across a distributed, multi-repository ecosystem.

## 🎯 Purpose
To prevent \"intelligence decay\" and \"context fragmentation\" when a single architectural change must be propagated across multiple independent repositories.

## 🛠 Core Workflow: The Mobilization Sequence

### 1. Preparation & Mapping
- **Verify Source:** Ensure master documentation, schemas, and guides are in their final, versioned state in the master repository.
- **Target Mapping:** Map the source (Master Docs) to the destination (Target Repo `docs/` or `onboarding/`).

### 2. The \"Atomic Sync\" Pattern
To avoid the `180s` timeout limit and Git divergence in large repositories:
- **DO NOT** use bulk loop scripts (e.g., `for r in repos; do ... done`) via `terminal` if the repos contain thousands of files (like `node_modules/`).
- **DO** iterate manually/sequentially via discrete `terminal` calls.
- **Sequence for each repo:**
  1. `cd <repo>`
  2. `git fetch origin`
  3. `git pull origin <branch> --rebase` (Handle divergence early).
  4. `mkdir -p docs`
  5. `cp -r <source_docs_path>/* docs/` (Use `rsync` if available to exclude heavy/junk directories).
  6. `git add docs/`
  7. `git commit -m \"docs: sync master architecture and onboarding\"`
  8. `git push origin HEAD`

### 3. The \"Massive Docs Commit\" Strategy
In high-volume synchronization (e.g., adding dozens of onboarding/security files at once):
- **Symptom:** Large file counts or high insertion numbers (e.g., 50k+ lines) causing long-running Git processes.
- **Behavior:** Git may emit numerous `LF will be replaced by CRLF` warnings on Windows hosts.
- **Action:** Proceed with the commit. These are cosmetic warnings handled by Git and do not indicate corruption or failure.
- **Post-Commit:** Always verify the push success by checking the exit code of the `git push` command immediately following the commit.

### 4. Private Repo Clone + Built-Site Import Pattern
When the source archive is a private GitHub repo containing a pre-built static site (e.g., `fleet-static-host`):
1. **Clone shallow:** `gh repo clone <org>/<repo> -- --depth 1` to avoid pulling full history.
2. **Identify content roots:** Inspect `public/` for built HTML, `papers/`, `writings/`, `art/` subdirs.
3. **Preserve directory structure:** Mirror the built-site layout into the target archive (e.g., `si-papers-new/papers/site/`).
4. **Copy selectively:** Import only content directories (HTML, JSON assets, markdown source). Exclude heavy binary directories (`ternary/data/`, `node_modules/`, build tooling).
5. **Commit as single atomic archive push:** Stage the entire copied tree, commit with a descriptive message, push. Do not interleave with code changes.
6. **Verify file counts:** `find <target> -type f | wc -l` before and after to confirm completeness.

## ⚠️ Pitfalls & Mitigations

### LF/CRLF Warnings Are Cosmetic (Windows)
- **Symptom:** `warning: LF will be replaced by CRLF the next time Git touches it` during `git add` on Windows.
- **Fix:** These are **not errors**. Proceed with commit. Git handles line-ending normalization automatically. Do not abort or retry.

### Diverged Branches (Non-fast-forward)
- **Symptom:** `git push` returns `[rejected] HEAD -> main (non-fast-forward)`.
- **Fix:** 
  - **Safe:** `git pull --rebase` to integrate remote changes.
  - **Aggressive (Requires User Consent):** `git reset --hard origin/<branch>` to wipe local state.

### Bulk Operation Timeouts
- **Symptom:** Command exit code `124` (Timeout).
- **Cause:** Recursive `cp` or `git push` on massive file counts exceeds 180s.
- **Fix:** Break the loop. Execute one repository at a time. Explicitly exclude `node_modules/` or `.venv/` from the copy phase.

### Pathing Proxy & Symlink Mismatches
- **Symptom:** `write_file` or `patch` operations land in a "ghost directory" (e.g., `C:\\\\c\\\\Users\\\\...`).
- **Fix:** **Always use absolute paths** for filesystem operations during high-stakes orchestration. 

### Remote Collision
- **Symptom:** Attempting to add a remote `origin` that already exists.
- **Fix:** Use `git remote remove origin` before re-establishing a connection if needed.

### Schema Stub Propagation
- **Risk:** Pushing "synced" docs to a repo whose gold-standard schemas are empty stubs means the fleet validates against nothing.
- **Mitigation:** Before declaring any repo "synced," inspect `schemas_gold_standard/` (or equivalent) for stub byte counts. If stubs detected, deliver a Schema Population Plan as part of the sync artifacts.

### Intentional Mid-Stream Push
- **Pattern:** Push high-value audit artifacts (Checklist, Gap Matrix) to GitHub *before* completing full schema population or code changes.
- **Why:** The fleet reads from GitHub. A partial artifact on origin is more valuable than a "complete" artifact only on local disk.
- **Sequence:** Audit → push audit docs → populate schemas → push populated schemas → verify Checklist against live.

## References
- `references/repo-mobilization-order.md` — canonical push order for SuperInstance repos
- **Related skill:** `schema-population-workflow` — use this skill when mobilization requires populating empty schemas in target repos. Load it before starting any sync that touches `schemas_gold_standard/` or similar canonical schema directories.

## Succession Logic
Every synchronization task MUST culminate in the creation of a "Handover" artifact. A repository is not "synced" until its internal documentation reflects the current state of the global architecture.
