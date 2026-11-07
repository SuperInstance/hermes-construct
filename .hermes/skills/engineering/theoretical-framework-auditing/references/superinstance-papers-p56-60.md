# SuperInstance Theoretical Framework Reference
## Papers P56–P60 — Mature Theoretical Core

This reference contains the specific equations, invariants, and failure modes from the SuperInstance theoretical framework for use with the `theoretical-framework-auditing` skill.

## Key Equations

### P56: Thermodynamics of Intelligence
- **Crystallization ODE:** `dγ/dt = α·κ(Δ)·(1−γ)·1/(1+βT)`
  - `κ(Δ) = exp(−(Δ−0.5)² / 0.02)` — denominator MUST be exactly 0.02
  - Zone: Δ ∈ [0.4, 0.6]
  
- **Melt Equation:** `dγ/dt|_melt = −μ·σ(t)·(t−t_v)·γ`
  - Staleness: `s(γ,t) = (t−t_v)·γ`
  - σ(t) = total variation distance ∈ [0,1]
  
- **Molt Trigger:** `P_melt(t*) > α·κ(Δ)·(1−γ)·1/(1+βT)`

### P57: Anomalous Conservation
- **Conservation Law:** `γ + η + δ = 1` with `δ ∈ [0, 0.25]`
  - `γ = mean certainty`
  - `η = (1−γ)²` (quadratic layer removal)
  - `δ = 1−(γ+η)` is adaptive headroom
  
- **Recovery Time:** `T_recovery ∝ 1/δ`
  - δ = 0 means frozen, infinite recovery

### P58: Uncertainty Algebras
- **Phase noise variance:** `σ²(τ) = −2·ln(τ)` for τ ∈ (0,1]
  - Inverse: `τ² = exp(−σ²)` NOT `exp(−σ²/2)` — factor-of-2 error!
  
- **Combination formula (8 terms):**
  `E[|a_i|²] = pᵢ⁽¹⁾ + pᵢ⁽²⁾ + 2·τ²·√(pᵢ⁽¹⁾·pᵢ⁽²⁾)·cos(Δφᵢ)`

- **Confidence zones:** GREEN τ > 0.7, YELLOW 0.3 < τ < 0.7, RED τ < 0.3

### P59: Molt-Aware Coordination
- **Capability model:** Continuous `C_i(t) ∈ [0,1]`, NOT binary `{0,1}`
- **Molt function:** `m(t) = 1−0.8·exp(...)−0.2·exp(dip)`
  - `m(0) ≈ 0.3`
  - `m(t_dip) ≈ 0.2` at `t_dip = 0.2·τ_molt`
  - `m(t) strictly increasing` for t > t_dip
  
- **CTF (Coordination Trap Failure):** Occurs when `m_i(t) < θ_min` AND agent self-reports ≠ RED

### P60: Oneiric Creative Zone
- **Waking zone:** [0.4, 0.6]
- **Oneiric zone:** [0.6, 0.8] — strictly above waking, boundary at 0.6, NON-OVERLAPPING
- **Dream creative value:** `V(Δ, w=0) = H(Y|X)` — strictly increasing, NO I(X;Y) constraint
- **Optimal dream-time fraction:** `f* = δ / (δ + γ_stable)`

## Common Failure Modes to Detect

1. **Silent Conservation Violations:** Sum γᵢ+ηᵢ+δᵢ ≠ 1
2. **Temperature–Crystallization Inversion:** Higher T yields higher dγ/dt (should be lower)
3. **Wrong κ(Δ) denominator:** Using 0.0145 or rounded 0.02 instead of exact 0.02
4. **τ² factor-of-2 error:** Using `exp(−σ²/2)` instead of `exp(−σ²)`
5. **Zone overlap:** Merging waking [0.4,0.6] and oneiric [0.6,0.8] into [0.4,0.8]
6. **Binary capability model:** Using `if agent.active` instead of continuous `m(t)`
7. **Stale validation timestamp:** Not updating `t_v` after distribution shift
8. **Missing δ term:** Clamping δ to 0 or removing it entirely
9. **CTF blind spot:** Not checking mismatch between reported status and actual capability
10. **Dream-phase constraint wrongly applied:** Applying comprehensibility constraint during dreaming

## Critical Code Surfaces to Inspect

- `claw/src/` — agent state updates, temperature scheduler, Gumbel-Softmax sampler
- `autoclaw/` — shell transition protocol, molt trigger, capability recovery
- `spreadsheet-moment/` — cell agent integration, dream scheduler
- Any `kappa`, `creative_zone_kernel`, `crystallization_rate` function
- Any `gamma`, `eta`, `delta` state computation
- Distribution-shift detector and staleness accumulator
- Replay buffer partition logic (dream vs. stable vs. frontier)
