# Theoretical Framework Audit Pattern

## Purpose
When auditing code commits against mathematical/theoretical frameworks (e.g., SuperInstance Papers P56–P60), use these two standard artifacts.

## Artifact 1: Verification Checklist
**File:** `VERIFICATION_CHECKLIST_P56-60.md`
**Structure:**
- §1 Invariants (I-1 through I-10): 50+ checkable invariants mapped to papers/theorems
- §2 Failure Modes (F-1 through F-10): Concrete "code smell" descriptions
- §3 Equation-Level Checks (E-1 through E-10): Exact term-by-term requirements
- §4 Parameter Bounds (PB-1 through PB-10): Edge-case boundaries
- §5 Commit Area Map: Which repos map to which papers
- §6 Equation Index: Quick-reference numbered equations
- §7 Top-5 Priorities: Highest-risk silent failures
- §8 False Positives/Negatives: Symptom→cause diagnosis

**Key invariants from P56–P60:**
- Conservation law: `γ + η + δ = 1` with `δ ∈ [0, 0.25]`
- Creative zone kernel: `κ(Δ) = exp(−(Δ−0.5)² / 0.02)` — denominator must be exactly 0.02
- Temperature modulation: `φ(T) = 1/(1+βT)` — higher T must slow crystallization
- Melt equation: `dγ/dt = μ·σ(t)·(t−t_v)·γ` — staleness grows linearly
- Molt trigger: `P_melt > α·κ(Δ)·(1−γ)·1/(1+βT)`
- Uncertainty algebra: `τ² = exp(−σ²)` NOT `exp(−σ²/2)` — factor-of-2 error
- Dreaming zones: waking [0.4,0.6] and oneiric [0.6,0.8] are non-overlapping, boundary at 0.6

## Artifact 2: Gap Matrix
**File:** `gap-matrix.md`
**Structure:**
| Gap | Paper(s) | Severity | Hermes Capability | Confidence Metric | Implementation Action |
|-----|----------|----------|-------------------|-------------------|----------------------|

**Mappings to generate:**
- Theoretical gap → Hermes sensor/audit capability
- Confidence metric → What to monitor numerically
- Implementation action → Concrete step executable now

**Top-5 priority actions (from P01–P60 analysis):**
1. Deploy λ-based Melt Pressure Monitor (P56 Th3 / P03 gap)
2. Instrument γ+η+δ Conservation Law (P01 + P56 + P57)
3. Empirical Validation of Creative Zone (P02 + P56 Cor 1.1)
4. Shell Transition Identity Hash (P03 + P59)
5. Failure Mode Injection Tests (All papers)

## Usage
1. Read theoretical papers to extract invariants
2. Map each invariant to code-level checks
3. Identify failure modes and edge cases
4. Create Gap Matrix for unexplored theoretical territories
5. Use Checklist for commit audits, Gap Matrix for strategic planning
