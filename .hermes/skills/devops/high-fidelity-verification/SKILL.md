---
name: high-fidelity-verification
description: Governs the verification of digital artifacts via local, remote, and external testing to prevent hallucinated persistence.
---

# Skill: High-Fidelity Artifact Verification (HFAV)

This skill governs the process of ensuring that any digital artifact (file, code, or repository) created by an agent is not only "written" to the local disk but is actually persistent, synchronized, and verifiable from an external perspective.

## Purpose

To prevent "hallucinated persistence"—situations where an agent believes a file has been written successfully when it has actually failed due to pathing errors, filesystem permissions, or environment mismatches.

## Verification Protocol

### Phase 1: Local Manifestation (The Internal Truth)

Before reporting success, the agent MUST:
1.  **Recursive Scan:** Use `ls -R` or `search_files` to confirm the file exists at the exact absolute path intended.
2.  **Content Parity:** Use `read_file` to sample the first and last 10 lines of the file to ensure the content matches the intended output and is not truncated or empty.
3.  **Absolute Path Enforcement:** Verify that the terminal-executed command and the filesystem-tool call are using the exact same absolute pathing convention (e.g., `/c/Users/...` vs `C:\\Users\\...`).

### Phase 2: Remote Synchronization (The Network Truth)

If the task involves a Git repository:
1.  **State Check:** Run `git status` to ensure the working tree is clean and changes are staged.
2.  **Explicit Push:** Execute `git add . && git commit -m "[Message]" && git push origin [branch]`
3.  **Log Verification:** Check the exit code of the push.
4.  **Manifesto Pattern:** For critical philosophical or foundational texts (e.g., Manifestos), the push should be treated with higher ceremony—ensuring Git history is clean and the commit message is communicative.

### Phase 3: External Validation (The Outside-In Truth)

The final and most critical step. The agent MUST:
1.  **Bypass Local Cache:** Use `curl -I` (for headers) or `curl [URL]` (for content) to fetch the file directly from the remote host (e.g., GitHub) using its public URL.
2.  **HTTP Verification:** Confirm an `HTTP/1.1 200 OK` response.
3.  **Content Confirmation:** If possible, verify that the `Content-Length` or a hash matches the local version.

## Verification Protocol

### Phase 1: Local Manifestation (The Internal Truth)

Before reporting success, the agent MUST:
1.  **Recursive Scan:** Use `ls -R` or `search_files` to confirm the file exists at the exact absolute path intended.
2.  **Content Parity:** Use `read_file` to sample the first and last 10 lines of the file to ensure the content matches the intended output and is not truncated or empty.

### Phase 2: Remote Synchronization (The Network Truth)

If the task involves a Git repository:
1.  **State Check:** Run `git status` to ensure the working tree is clean and changes are staged.
2.  **Explicit Push:** Execute `git add . && git commit -m "[Message]" && git push origin [branch]`
3.  **Log Verification:** Check the exit code of the push.

### Phase 3: External Validation (The Outside-In Truth)

The final and most critical step. The agent MUST:
1.  **Bypass Local Cache:** Use `curl -I` (for headers) or `curl [URL]` (for content) to fetch the file directly from the remote host (e.g., GitHub) using its public URL.
2.  **HTTP Verification:** Confirm an `HTTP/1.1 200 OK` response.
3.  **Content Confirmation:** If possible, verify that the `Content-Length` or a hash matches the local version.

## Pitfalls

- **Path Mismatches:** Always use absolute paths. Do not rely on relative paths in `terminal()` calls. On Windows, MSYS2 POSIX paths (`/c/Users/...`) can resolve differently than native Python paths (`C:\\\\\\\\Users\\\\\\\\...`). A known failure mode is the `C:\\\\c\\\\Users\\\\...` prefix: when `/c/Users/casey/...` is incorrectly rooted, the artifact lands outside the workspace. **Fix:** Pass native Windows paths to `write_file`/`patch`, or use `terminal` with `cp` for cross-boundary copies. Always `read_file` after `write_file` to confirm the path resolved correctly.
- **Stale Bytecode Cache (Windows):** After patching a `.py` file that is loaded via `importlib.util.spec_from_file_location()`, CPython may continue serving stale bytecode from `__pycache__/*.pyc`. **Manually delete `__pycache__` before rerunning verification** after hot-fixing a monitor or runner script.
- **F-String Quote Nesting:** In Windows/MSYS Python, an f-string delimited by double quotes cannot contain escaped double-quote subscripts without extra escaping: `f"echo '{inst['id']}'"` is safe; `f"echo '{inst[\\\"id\\\"]}'"` can trigger `SyntaxError: unexpected character after line continuation character` on some Python builds. Prefer single-quote subscripts inside double-quote f-strings, or use `.format()`/string concatenation when nesting gets deep.
- **Test State Contamination:** When running synthetic verification packets through a stateful monitor, **reset `agent_history` and `processed` lists in the telemetry/monitor state files** between test runs. Otherwise prior test data pollutes results and produces false FAIL/PASS outcomes. Write a one-shot reset script or shell command before each synthetic test.
- **Silent Failures:** Never assume a `write_file` succeeded just because it didn't return an error. Always follow with a `read_file`.
- **write_file Is Destructive Overwrite:** `write_file` replaces the *entire* file. Passing placeholder, test, or "no-repeat" text destroys real files (scripts, schemas, production code). Only call it when actual intended content is ready. For drafts or self-notes, use chat or `todo`, not `write_file`.
- **Subagent Blindness:** Subagents often fail to perform Phase 3. This skill should be used by the Orchestrator to command a "Verification Subagent."
- **Hallucinated Persistence:** Agents may incorrectly report success if they rely on their own internal status logs rather than verifying the physical state of the filesystem and remote endpoints. Always perform a post-write `ls` and an external `curl` to break the feedback loop of "assumed success."
- **Lint Diff Noise:** `write_file` may report "Pre-existing lint errors" with ANSI-colored diff output that looks alarming. This is cosmetic; re-run the build tool and trust the binary exit code.
- **Patch Retry Hazard:** If `patch` fails on a file (especially Rust files with derive macros), the file is in an undefined state. Do NOT retry `patch`. Use `write_file` for a complete clean rewrite.
- **Compile After Every Write:** For compilation-checked languages (Rust, TypeScript), run the type checker immediately after each `write_file`. Waiting until multiple writes accumulate turns one error into 20+ cascade errors that are harder to diagnose in bulk.
- **Subagent Blindness:** Subagents often fail to perform Phase 3. This skill should be used by the Orchestrator to command a "Verification Subagent."
- **Hallucinated Persistence:** Agents may incorrectly report success if they rely on their own internal status logs rather than verifying the physical state of the filesystem and remote endpoints. Always perform a post-write `ls` and an external `curl` to break the feedback loop of "assumed success."

## Verification Protocol

When verifying schema or state changes, prefer deterministic checklists over eyeballing:
- Use `scripts/verification_checklist_runner.py <state.json>` for CONS/MOLT/DRM/CK/OPT/UA/MLT/G1/G4 invariants.
- Expect **13/13 PASS** before reporting integration success (11 base + G1-DETERMINISM + G4-COMPOSITION).
- If new rules are added to the runner, update this expectation immediately.
- **State-reset before synthetic tests:** When feeding synthetic packets through a stateful monitor, reset `agent_history` and `processed` lists between runs. Otherwise prior test data produces false FAIL/PASS outcomes. Use a one-shot reset block before each synthetic test:
  ```python
  state["agent_history"] = {}
  state["processed"] = [x for x in state.get("processed", []) if x not in ("smoke", "g1", "final", "test")]
  ```
- **New rules follow a dict-entry pattern:** Add rules to `RULES` in `verification_checklist_runner.py` with `description` and `check` lambda. Alerts in `cns_monitor_v3_telemetry.py` must mirror the same rule name so PASS/FAIL and alert output stay correlated.
- **Ghost Tile rules added in this session:**
  - `G1-DETERMINISM`: `deterministic=true` implies `T=0.0`.
  - `G4-COMPOSITION`: equipment stacking passes (≤2 slots) or asserts `composition_tested`.

## CNS v3 Verification Patterns

### Empirical Validator (P56 Theorem 1)

When verifying that live telemetry matches theoretical crystallization models:
1. Build an `EmpiricalValidator` that stores `(timestamp, gamma, predicted_rate)` tuples per agent in a rolling window (default 10 samples).
2. Compute empirical `dγ/dt` from `win[-3]` to `win[-1]`.
3. Compare against `compute_max_crystallization_rate(gamma, semantic_distance, temperature)`.
4. Emit `EMPIRICAL-DIVERGENCE` alert if `|empirical - predicted| > 0.05`.
5. Persist windows into `telemetry_state["empirical_windows"]` across monitor restarts.

### Ingress Repair

Before constructing `TelemetryQuantum` from inbound USCP-v3 packets:
1. Renormalize γ+η+δ to sum 1.0 if total is within [0,2].
2. Clamp γ∈[0,1], η∈[0,1], δ∈[0,0.25].
3. Log `WARNING` when repair occurs so the source can be traced.

### Egress Validation

Before emitting CNS pulses from a daemon:
1. Run `_validate_telemetry(self.telemetry)` to repair out-of-bounds values.
2. Mutate `self.telemetry = validated_tel` so the source of truth is clean.
3. Use `validated_tel[...]` (not `self.telemetry[...]`) in the packet construction block.

### Ghost Tile Inference Wiring

When propagating deterministic config into vLLM launch scripts:
1. Build `vllm_args` list per instance.
2. Append `--temperature 0.0` and `--seed <id>` only if `inst["ghost_tile"]["deterministic"] is True`.
3. Join with `"\\n".join(vllm_args)` for clean multi-line shell output.
4. Expose `ghost_tile` in the OpenClaw manifest so CNS monitor can read it.

### Windows Test Fixes

- Use `tempfile.mkdtemp()` + `shutil.rmtree()` instead of `tempfile.mktemp()` for test directories.
- Use `os.replace()` instead of `os.rename()` for atomic file overwrites.
- Skip HOME-expansion unit tests on Windows where `~` maps to `USERPROFILE`.

### Rust/Criterion Benchmark Verification (2026-08-23)

See `references/rust_criterion_verification.md` for the complete protocol for verifying Rust crates with `criterion` benchmarks, including build verification, unit test verification, benchmark execution, validation against latency budgets, and CI/CD integration. Derived from CNS↔DSH Bridge spike validation (8/8 tests PASS, 4/4 latency targets met at 12-90x under budget).