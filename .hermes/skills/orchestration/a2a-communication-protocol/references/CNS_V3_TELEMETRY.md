# CNS v3 Telemetry Reference

Verified integration points: `SuperInstance/autoclaw` (`crew/daemon.py`, `scripts/cns_monitor_v3_telemetry.py`).

## Blocks

| Block | Path | Type | Notes |
|---|---|---|---|
| `gamma_eta` | `payload.gamma_eta.per_agent.{id}` | object | `gamma`, `eta`, `delta` floats |
| `gamma_eta` | `payload.gamma_eta.fleet_sum` | object | `total_gamma_eta`, `utilization` |
| `thermal` | `payload.thermal` | object | `temperature`, `temperature_task`, `temperature_idle`, `is_dreaming` |
| `creative` | `payload.creative` | object | `semantic_distance`, zone booleans |
| `melt` | `payload.melt` | object | `melt_pressure`, `max_crystallization_rate`, `melt_threshold_exceeded`, `identity_hash` (G2), `deterministic` (G1) |
| `molt` | `payload.molt` | object | `molt_count`, `max_molt_chain`, `capability` |
| `uncertainty` | `payload.uncertainty` | object | `tau`, `confidence_zone` |

## Conservation Invariants

- CONS-1: `γ + η + δ = 1` within floating tolerance
- CONS-2: `γ ∈ [0,1]`, `η ∈ [0,1]`, `δ ∈ [0,0.25]`
- CONS-3: `η = (1−γ)²` in `ONE_PHASE` mode
- UA-1: `τ² = exp(−σ²)` **not** `exp(−σ²/2)`
- δ env bound: `δ*(λ_env) ∈ [0,0.25]`

## Ghost Tile Bridge Invariants (2026-08-23)

- G1-DETERMINISM: If `deterministic: true` in the melt block, then `temperature == 0.0`. Monitor emits WARNING if violated.
- G4-COMPOSITION: Equipment stacking must satisfy associative composition law. Checklist passes if fewer than 2 equipment slots or `composition_tested: true` is asserted.

## Checklist Extension Pattern

To add a new invariant rule to `scripts/verification_checklist_runner.py`:
1. Add an entry to the `RULES` dict with a unique ID (e.g. `"G1-DETERMINISM"`).
2. Use a lambda that reads from the agent snapshot dict (same shape as `agent_hist`).
3. Update the expected PASS count in `high-fidelity-verification` skill.
4. Add the rule ID to the CNS monitor's `get_alerts()` if it warrants fleet-wide alerting.

## Critical Bugs (Fixed)

### max_crystallization_rate extraction (Fixed 2026-08-23)

`max_crystallization_rate` was defaulted to `0.0` in `parse_telemetry_from_packet`, causing every packet to trigger false MOLT-1 (`P_melt > crystallization_rate`).

Fix: explicitly extract `payload["melt"]["max_crystallization_rate"]` and persist it in `agent_hist` state. Also added the field to `TelemetryQuantum` dataclass and the `agent_hist` write path.

### State contamination between test runs (Fixed 2026-08-23)

Synthetic verification packets processed against non-reset `agent_history` and `processed` lists produce cross-contaminated results.

Fix: reset both `cns_telemetry_state.json` and `cns_monitor_state.json` (clear `agent_history` and filter `processed` list) before each synthetic test run.

## Windows Dev Pitfalls

- **Stale bytecode:** `importlib.util.spec_from_file_location()` does **not** invalidate `__pycache__/*.pyc`. Delete stale `.pyc` after patching source used by importlib.
- **Path mismatch:** MSYS2 POSIX paths (`/c/Users/...`) can fail `open()` on Windows Python. Use `os.path.normpath(r'C:\\Users\\casey\\...')` when crossing terminals.
- **Listdir divergence:** `os.listdir('/c/Users/...')` (POSIX) and `os.listdir(r'C:\\Users\\...')` (native) can return different results on Windows. Use native paths for Python file I/O in this environment.

## Empirical Validation Pipeline (P56 Theorem 1)

Added `EmpiricalValidator` class to `scripts/cns_monitor_v3_telemetry.py`:
- Stores rolling windows of `(timestamp, gamma, predicted_rate)` tuples per agent.
- Computes empirical `dγ/dt` from `win[-3]` to `win[-1]`.
- Compares against `compute_max_crystallization_rate()` prediction.
- Emits `EMPIRICAL-DIVERGENCE` alert when `|empirical - predicted| > 0.05`.
- Persisted into `telemetry_state["empirical_windows"]`.

## Conservation Law Runtime Hooks (2026-08-23)

### Egress Validation (daemon.py)
Added `_validate_telemetry()` that repairs CONS-1 (renormalize), clamps CONS-2/DRM/MOLT-4, and mutates `self.telemetry = validated_tel` before packet construction.

### Ingress Repair (cns_monitor_v3_telemetry.py)
Added renormalize/clamp logic in `parse_telemetry_from_packet()` before `TelemetryQuantum` instantiation. Logs `WARNING` on repair.

## vLLM Ghost Tile Propagation (2026-08-23)

`cudaclaw_wizard.py` `VllmSetup.generate_start_script()` now:
- Builds `vllm_args` list per instance.
- Appends `--temperature` and `--seed` only when `ghost_tile.deterministic` is `True`.
- Joins args with `"\\\n".join(vllm_args)` for clean multi-line shell output.
