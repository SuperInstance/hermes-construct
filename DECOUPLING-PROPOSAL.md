# Decoupling Proposal — hermes-construct harness vs. vendored hermes-agent

**Date:** 2026-07-10
**Branch:** `decoupling-research-2026-07-10`
**Author:** Research session (grounded in live repo + upstream inspection)

> *"hermes-construct is a beautiful idea, but we need the harness decoupled from
> the hermes so that as hermes community updates their system ours updates
> alongside, because we build a snail shell for the artificial life to live in
> and operate out of."*

Read literally: the Rust shell kernel (`src/`) is the **snail shell**. The
vendored Python `hermes-agent` tree is the **artificial life**. Right now they
share one git history, so 5,028 commits of upstream drift threaten the shell's
own code with every sync attempt. This proposal finds that the decoupling is
**far more tractable than the commit-count panic implies**, because the two
halves are *already* runtime-independent — the pain is purely git-history-level.

---

## Markers used throughout

| Marker | Meaning |
|--------|---------|
| ✅ | Verified true right now (inspected the code/git/PyPI directly) |
| ⚠️ | True, but with real caveats you must weigh |
| 🔮 | The eventual target — not yet true, needs work |

---

## TL;DR — the recommendation in five lines

1. **The Rust shell and the Python tree have ZERO runtime coupling today.** ✅
2. **`hermes-agent` is published on PyPI** (latest `0.18.2`; vendored copy is stale at `0.15.1`). ✅
3. **SuperInstance has ZERO original modifications to Python logic** — the 12 Python-file changes are cherry-picks of upstream's own fixes, all subsumed by any tag ≥ `0.16.0`. ✅
4. **Recommendation: delete the vendored Python tree from this repo; pin `hermes-agent==<tag>` as a pip dependency.** This is the cleanest separation and all its preconditions are already met.
5. **Immediate safe step:** the 2 "critical security fixes" from `UPSTREAM-MERGE-AUDIT.md` are *already applied* (cherry-picked). The real immediate win is adding a `requirements-hermes.txt` pin to `0.18.2` — non-destructive, delivers 5,000+ missed upstream improvements instantly.

---

## 1. Ground truth: how coupled are the shell and the agent, really?

### 1.1 The Rust shell kernel is fully self-contained ✅

`src/main.rs` is a standalone binary. It:
- Loads `.env` via `dotenvy` (API keys: `TELEGRAM_BOT_TOKEN`, `DEEPINFRA_API_KEY`, `ZAI_API_KEY`).
- Inits its own SQLite (`universe.db` via `rusqlite`, bundled).
- Starts Telegram long-polling via `teloxide` **or** falls back to a stdio port.
- Registers LLM providers (`ensign::DeepInfraProvider`, `ensign::ZaiProvider`) as Rust trait objects.
- Runs its own tick loop (`kernel.rs`) with conservation budgets, rooms, tiles, spectral analysis, module autoloading.

A grep of **all** `src/*.rs` for `python|hermes_agent|hermes-agent|cli\.py|run_agent|subprocess|Command::new|spawn|\.py` found **zero** references to Python, the Python tree, or process-spawning into it. The only `spawn` hits are `tokio::spawn` (async tasks) and a `shell_spawn` budget field name. A grep for `config\.yaml|\.hermes|/agent/|/gateway/|/tools/|/skills/|/hermes_cli/|HERMES_HOME` found **no files** — the Rust kernel does not touch `~/.hermes`, does not read `config.yaml`, and does not reference any Python package directory.

**Conclusion:** the snail shell does not currently host the artificial life at runtime. They are two parallel, independent programs that happen to live in one git repo. ✅

### 1.2 The deploy scripts deploy ONLY the Rust binary ✅

`scripts/deploy-oracle.sh` and `scripts/install-oracle.sh` (the real ARM/Oracle Cloud deploy path):
- `cargo build --release [--target aarch64-unknown-linux-gnu]`
- Copy `bin/hermes-construct` + `rooms/*.json` + `ensigns/*.json` + systemd unit
- Create `/etc/hermes-construct/hermes.env` (Telegram + provider keys only)
- State lives at `/var/lib/hermes-construct/universe.db`

A grep of `deploy/` and the three deploy scripts for `python|pip install|hermes-agent|cli\.py` returned **no matches**. The ARM production deployment contains **zero Python**. The entire vendored Python tree is dead weight on the deploy artifact.

### 1.3 The `acp_adapter` is NOT a Rust→Python bridge ✅

`acp_adapter/` is part of the **Python hermes-agent tree** (it ships as the `hermes-acp` console script in `pyproject.toml`). It is an ACP *server* that **exposes** hermes-agent to external clients (VS Code, Zed, JetBrains). `acp_adapter/entry.py` directly imports `run_agent`, `hermes_cli.main`, `hermes_cli.env_loader`, `tools.mcp_tool`, and inserts the project root onto `sys.path` so `from run_agent import AIAgent` resolves.

This is **tight coupling to the Python internals**, not a clean process boundary between Rust and Python. There is no code anywhere in the repo that makes the Rust shell spawn `hermes-acp` or speak ACP to it. The ACP adapter is a *consumer-facing* surface of the Python agent, not the integration seam between the two halves.

> ⚠️ **Caveat for the future:** if the shell is ever to *host* the agent (the literal "snail shell for artificial life to live in"), ACP-over-stdio is the right protocol boundary to build that on — but that bridge does not exist yet and is out of scope for this decoupling proposal. It is revisited in §6.

### 1.4 `hermes-agent` is a real, pip-installable PyPI package ✅

```
$ pip index versions hermes-agent
hermes-agent (0.18.2)
Available versions: 0.18.2, 0.18.1, 0.18.0, 0.17.0, 0.16.0, 0.15.2, 0.15.1, ...
```

The upstream `pyproject.toml` declares `[project] name = "hermes-agent"`, ships console scripts (`hermes`, `hermes-agent`, `hermes-acp`), and declares its full package set via `setuptools.packages.find`. Git tags map cleanly to PyPI versions:

| Upstream tag | PyPI version |
|---|---|
| (vendored here, stale) | `0.15.1` |
| `v2026.7.7.2` (latest tag) | `0.18.2` |

Tag cadence is roughly weekly (`v2026.3.12` → `v2026.7.7.2`). A stable, pinnable target exists — this does not have to mean tracking bleeding-edge `main`.

### 1.5 The upstream drift, re-measured ✅

```
$ gh api repos/SuperInstance/hermes-construct/compare/SuperInstance:main...NousResearch:main
{"ahead_by":5028,"behind_by":43,"status":"diverged","total_commits":5028}
```

SuperInstance is **5,028 commits behind** upstream and **43 commits ahead** (its own work). The stale `UPSTREAM-MERGE-AUDIT.md` (2026-06-01) said "38 behind, 11 ahead" — the gap grew ~130× in five weeks. Upstream changed **3,936 files** in that window.

---

## 2. The real conflict surface (this is the key finding)

SuperInstance's 43-ahead commits were inspected commit-by-commit. They fall into exactly two buckets:

### Bucket A — SuperInstance-original work (new files, zero conflict risk) ✅

~31 commits, all adding **new files** that upstream does not have:

| Area | Files |
|------|-------|
| Rust shell kernel | `src/*.rs` (14 files), `Cargo.toml`, `Cargo.lock`, `.cargo/` |
| Shell configs | `rooms/*.json`, `ensigns/*.json`, `templates/` |
| Examples | `examples/*.rs` (5 example binaries) |
| ARM deploy | `deploy/`, `scripts/deploy-oracle.sh`, `scripts/install-oracle.sh`, `scripts/deploy-jetson.sh`, `scripts/test-arm-build.sh`, `scripts/hermes-construct.service` |
| Docs | `BOOTSTRAP_SOLUTION.md`, `BUDGET_SOLUTION.md`, `DIARY.md`, `DIARY-RILEY.md`, `PLATO_BUILD_PLAN.md`, `PROTOTYPE_v0.1.md`, `ROADMAP.md`, `ROUTING_SOLUTION.md`, `SCHEMAS.md`, `TEST-DIARY.md`, `UPSTREAM-MERGE-AUDIT.md`, `AGENT.md`, `docs/ARCHITECTURE.md`, `docs/QUICKSTART.md` |
| CI | `.github/workflows/ci.yml` |
| Assets | `assets/hermes-construct.jpg`, `memory/` |

**None of these touch upstream Python files.** Upstream will never conflict with them because they don't exist upstream.

### Bucket B — Cherry-picked upstream fixes (modifying Python files, already subsumed) ✅

12 commits that modified Python files (`tools/approval.py`, `tools/file_tools.py`, `gateway/run.py`, `run_agent.py`, `utils.py`, `docker/`, `agent/`, `tools/skills_guard.py` + their tests). **Every one is a verbatim backport of an upstream commit** — confirmed by their upstream PR numbers (`#34601`, `#35717`, `#36231`, `#36705`) and by the fact that their patch subjects match `UPSTREAM-MERGE-AUDIT.md`'s listed hashes exactly.

These include the **2 "critical security fixes"** the audit told us to merge immediately:
- `9d65e95e9` — block agent writes to `~/.hermes/config.yaml` (silent approval bypass)
- `c4f47eb6d` — pair terminal-side gate for config.yaml writes

**They are already applied.** ✅ And they are subsumed by any upstream tag ≥ `0.16.0` — upstream `main` now contains their evolved descendants (`7bfdc0bca fix(security): close env/config write-deny bypass`, `123c8f3a2 fix(config): close unreadable-overwrite bug class`). A pin bump to `0.18.2` picks up all 12 *plus* the 5,000+ commits of additional upstream work.

### The actual overlap: 3 files ⚠️

The only files that **both** SuperInstance and upstream modify (the real merge-conflict surface):

| File | SuperInstance's change | Upstream's change | Conflict severity |
|------|------------------------|--------------------|-------------------|
| `.env.example` | Added ensign/deploy keys | Added new provider keys | Trivial — union the keys |
| `.gitignore` | Added `dist/` | Various | Trivial |
| `README.md` | Full rewrite (fork-visitor friendly) | Ongoing updates | Moderate — but SI's README is intentionally its own |

**That is the entire conflict surface. Three root-level config/doc files.** The "5,028 commits behind" number is almost entirely the *vendored Python tree being stale* — and SuperInstance doesn't actually modify that tree.

---

## 3. Decoupling mechanisms evaluated (with real tradeoffs for THIS repo)

### 3.1 PyPI dependency pin — **RECOMMENDED** ✅

Replace the vendored Python source tree with `pip install hermes-agent==<version>`.

| Dimension | Assessment |
|-----------|------------|
| Feasibility | ✅ `hermes-agent` is on PyPI, version `0.18.2` maps to tag `v2026.7.7.2`. |
| Coupling | ✅ Zero runtime coupling (verified §1.1). Removing the tree breaks nothing the Rust binary uses. |
| Patches needed | ✅ Zero original Python patches exist. All 12 changes are subsumed by `0.18.2`. |
| ARM deploy | ✅ Deploy scripts already use zero Python (verified §1.2). |
| Sync UX | ✅ "Catch up" = bump one version pin in one file. No merge, no conflict resolution. |
| Contributor friction | ✅ None — Rust contributors never see Python source. Python users `pip install`. |
| Offline ARM builds | ⚠️ `pip install` needs network at install time. Mitigated: deploy-oracle.sh already requires internet for `cargo build`; a `pip install` step is no worse. For fully-offline, pre-build a wheel and ship it in the deploy bundle. |
| Hacking on Python internals | ⚠️ Loses the ability to edit Python source in-place. Mitigated: `pip install -e <path-to-clone>` against a separate clone of upstream, or see §3.2 for the fork case. |

**This is the cleanest option and every precondition is already met.**

### 3.2 Git submodule pointing at `NousResearch/hermes-agent` (or a SI fork) — viable fallback ⚠️

Pin a submodule to an upstream tag.

| Dimension | Assessment |
|-----------|------------|
| Feasibility | ✅ Works. |
| Sync UX | ⚠️ `git submodule update --remote` + commit the new SHA. Not hard, but easy to forget. |
| Contributor friction | ⚠️ Real: `git submodule update --init` on clone, detached-HEAD confusion, `.gitmodules` churn. For a repo whose primary contributors work on the Rust shell, this is friction for code they don't use. |
| When to prefer this over PyPI | Only if SuperInstance needs to **maintain patches** to Python source that upstream won't accept. Today they have none. |
| Recommendation | Use **only** if/when Python patching becomes necessary, and point it at a `SuperInstance/hermes-agent` fork, not upstream directly. |

### 3.3 Git subtree — not recommended ⚠️

Same goal as submodule, messier history.

| Dimension | Assessment |
|-----------|------------|
| Sync UX | ⚠️ `git subtree pull` produces merge commits that interleave upstream history into SuperInstance's — exactly the noise we're trying to escape. |
| Visibility | ⚠️ Harder to tell "ours" vs "vendored" at a glance (no separate checkout). |
| Verdict | Strictly worse than a submodule here, and strictly worse than PyPI given `hermes-agent` is installable. Reject. |

### 3.4 Vendor-and-diff tooling — pragmatic but unnecessary here ⚠️

Keep vendoring, but maintain a script that tracks the fork point and generates a patch set against a specific upstream tag, so "catching up" means re-applying SuperInstance's *actual* patches rather than a raw merge of 5,000+ commits.

| Dimension | Assessment |
|-----------|------------|
| Effort | ⚠️ Requires building and maintaining the diff tooling (`git format-patch` against the fork point, re-apply loop). |
| Benefit here | Near-zero — SuperInstance's actual patch set against Python is **empty** (Bucket B is all upstream's own commits). There is nothing to re-apply. |
| Verdict | Would be the right answer if SuperInstance had dozens of bespoke Python patches. It doesn't. PyPI pin achieves the same cleanliness with less machinery. Reject for now; revisit only if bespoke Python patches accumulate. |

---

## 4. Concrete recommendation

### Primary: PyPI pin, single repo ✅

Keep **one repo** (`SuperInstance/hermes-construct`). Its content becomes:

- **The snail shell (stays):** `src/`, `Cargo.toml`, `Cargo.lock`, `.cargo/`, `rooms/`, `ensigns/`, `examples/`, `templates/`, `deploy/`, `scripts/` (SuperInstance's deploy/install/test scripts), `tests/` (Rust tests), the SuperInstance docs (`*.md`), `assets/`, `memory/`, `AGENT.md`, `.env.example`, `.gitignore`, `README.md`, `LICENSE`.
- **The artificial life (removed from repo, installed via pip):** the entire vendored Python tree — `agent/`, `gateway/`, `cron/`, `providers/`, `skills/`, `optional-skills/`, `tools/`, `tui_gateway/`, `acp_adapter/`, `acp_registry/`, `hermes_cli/`, `plugins/`, `ui-tui/`, `web/`, `website/`, `docker/`, `locales/`, `infographic/`, `nix/`, `packaging/`, `datagen-config-examples/`, `optional-mcps/`, and all root-level Python modules (`run_agent.py`, `cli.py`, `model_tools.py`, `toolsets.py`, `batch_runner.py`, `trajectory_compressor.py`, `toolset_distributions.py`, `hermes_bootstrap.py`, `hermes_constants.py`, `hermes_state.py`, `hermes_time.py`, `hermes_logging.py`, `utils.py`, `mcp_serve.py`, `mini_swe_runner.py`), plus `setup.py`, `pyproject.toml`, `MANIFEST.in`, `uv.lock`, `package.json`, `package-lock.json`, `flake.nix`, `flake.lock`, `constraints-termux.txt`, `docker-compose*.yml`, `Dockerfile`, `.dockerignore`, `.hadolint.yaml`, `.mailmap`, `.envrc`, `.gitattributes`, and all `RELEASE_v*.md`.
- **The pin (new):** a `requirements-hermes.txt` (or a minimal `pyproject.toml` for the Python-dev side) containing `hermes-agent==0.18.2` — corresponding to upstream tag `v2026.7.7.2`.

**Why one repo, not two:** the Rust shell is SuperInstance's IP and the only thing that deploys. The Python agent is upstream's IP, obtained externally. There is no second SuperInstance-owned Python codebase to house in a second repo — yet.

### Fallback (only if Python patching becomes necessary): two repos + submodule 🔮

If SuperInstance ever needs bespoke Python patches that upstream won't merge:
1. Fork `SuperInstance/hermes-agent` from `NousResearch/hermes-agent`.
2. Maintain patches there, against tagged releases.
3. In `hermes-construct`, add a git submodule pointing at `SuperInstance/hermes-agent@<tag>`.
4. Deploy via `pip install -e ./vendor/hermes-agent` (editable, so patches apply).

**This is not needed today** — there are zero bespoke Python patches. Do not build this until there's at least one patch to carry.

---

## 5. Migration path (phased, lowest-risk first)

### Phase 0 — already done (verify and breathe) ✅

The 2 critical security fixes from `UPSTREAM-MERGE-AUDIT.md` are **already applied** via cherry-pick (`9d65e95e9`, `c4f47eb6d`). There is no urgent security gap to close. The audit's "Phase 1: merge NOW" was already executed by a prior session.

**Action:** none. Just stop worrying about those two commits.

### Phase 1 — establish the pin, non-destructive ✅ (safe to do immediately)

1. Add `requirements-hermes.txt` with `hermes-agent==0.18.2`.
2. Verify `cargo build --release` succeeds (it will — the Rust shell has no Python dependency).
3. Verify `scripts/deploy-oracle.sh --skip-build` logic doesn't reference Python (it doesn't).
4. Update `.github/workflows/ci.yml` to also run `cargo build --release` and `cargo test` (the current CI is a stub: `pytest || true` on Python 3.10–3.12, which doesn't even test the Rust shell).

**Risk:** none. Nothing is removed. The pin file is additive. This step alone makes "catch up to upstream" a one-line version bump for anyone who wants the Python agent.

### Phase 2 — remove the vendored Python tree ⚠️ (large diff, low logical risk)

1. `git rm -r` the entire Python tree (see §4 inventory).
2. Keep the 3 shared files as SuperInstance's own versions:
   - `.env.example` — keep SI's (ensign/deploy keys); manually union any upstream keys you want.
   - `.gitignore` — keep SI's (`dist/`); upstream additions are irrelevant once Python is gone.
   - `README.md` — keep SI's rewrite; update the "Install" section to say `pip install -r requirements-hermes.txt` for the Python agent, `cargo build --release` for the shell.
3. Commit. The diff is large (thousands of files deleted) but the *logic* is safe: nothing in `src/`, `deploy/`, or `scripts/` imports or references the removed tree.
4. Verify the Rust shell still builds and the deploy script still works end-to-end on an ARM box.

**Risk:** the only real risk is removing a file that someone's workflow depends on but that I didn't catch. Mitigation: do this in a PR, run the full deploy on a test Oracle instance, keep the commit revertible. The `cargo build` + deploy-script verification is the safety net.

### Phase 3 — update docs and CI 🔮

1. `README.md`: clarify the two-mode story — `cargo run` (shell, ARM-friendly) vs `pip install hermes-agent` (full Python agent). Document that they're independent today.
2. `ROADMAP.md`: if the vision is for the shell to eventually *host* the agent, document the ACP-bridge plan (see §6) as a future milestone, not a current feature.
3. CI: replace the stub `pytest || true` with `cargo build --release && cargo test`. Add a separate job that does `pip install hermes-agent==$(cat requirements-hermes.txt | tail -1)` + `hermes --version` to confirm the pin resolves.

### Phase 4 — the sync cadence going forward ✅

Once decoupled, "tracking upstream" becomes:
1. Watch `NousResearch/hermes-agent` tags.
2. When a new tag ships (roughly weekly), bump the version in `requirements-hermes.txt`.
3. Run `pip install -r requirements-hermes.txt && hermes --version` to confirm.
4. (Optional) Run the Python agent's test suite against the new pin in CI.

No merges. No conflict resolution. No 5,000-commit catch-up attempts. Ever.

---

## 6. The "snail shell hosts the artificial life" vision (future, out of scope) 🔮

Today the Rust shell and the Python agent are **parallel, not integrated** — the shell has its own LLM providers (`ensign.rs`) and never invokes the Python agent. The user's metaphor ("a snail shell for the artificial life to live in and operate out of") implies the shell should eventually *orchestrate* the Python agent.

If/when that becomes the goal, the right boundary is already present in the codebase:

- `hermes-acp` (the `acp_adapter` console script) exposes the Python agent over **ACP (Agent Communication Protocol)** on stdio JSON-RPC.
- The Rust shell could `spawn` `hermes-acp` as a subprocess and speak ACP over its stdin/stdout.
- At deploy time, `deploy-oracle.sh` would add one step: `pip install -r requirements-hermes.txt` (pulls `hermes-agent==<pin>`).
- This keeps the two halves decoupled at the **process/protocol** level — the shell never imports Python; it talks to a pinned, pip-installed agent over a versioned protocol.

**This is explicitly a future milestone.** Building it is not required for the decoupling to deliver value — the decoupling (PyPI pin + tree removal) is valuable *because the two are independent today*, regardless of whether they're ever integrated tomorrow.

---

## 7. What NOT to do

- **Do NOT attempt a `git merge upstream/main` of the 5,028 commits.** It is enormously risky, touches 3,936 files, and the vast majority of those changes are to the Python tree that this proposal removes entirely. You'd be resolving conflicts in code you're about to delete.
- **Do NOT use git subtree.** It interleaves upstream history into SuperInstance's log — the exact noise this proposal eliminates.
- **Do NOT build vendor-and-diff tooling.** There are zero bespoke Python patches to diff. It's machinery for a problem that doesn't exist here.
- **Do NOT split into two repos yet.** There's no second SuperInstance-owned codebase to house. One repo + one pip pin is simpler and sufficient.
- **Do NOT touch `main`.** This proposal is on `decoupling-research-2026-07-10`. All changes land via PR after review.

---

## 8. Summary table

| Question | Answer | Marker |
|----------|--------|--------|
| Do the Rust shell and Python tree share runtime coupling? | No — verified zero references | ✅ |
| Is `hermes-agent` pip-installable? | Yes — PyPI `0.18.2`, maps to tag `v2026.7.7.2` | ✅ |
| Does SuperInstance modify Python logic? | No — 12 changes are all upstream cherry-picks, subsumed by `0.18.2` | ✅ |
| Are the 2 critical security fixes applied? | Yes — already cherry-picked | ✅ |
| What's the real conflict surface? | 3 files: `.env.example`, `.gitignore`, `README.md` | ✅ |
| Does ARM deploy need Python? | No — deploy scripts are Rust-only | ✅ |
| Recommended mechanism? | PyPI pin (`hermes-agent==0.18.2`), remove vendored tree | ✅ |
| Immediate safe step? | Add `requirements-hermes.txt` (Phase 1, non-destructive) | ✅ |
| Should the shell host the agent? | Not today; ACP-bridge is a future milestone | 🔮 |

---

*This proposal is grounded in direct inspection of the repo at commit `81d601c1c` (branch `decoupling-research-2026-07-10`) and upstream `NousResearch/hermes-agent@main` as fetched on 2026-07-10. Re-verify the upstream commit count before acting — it grows hourly.*
