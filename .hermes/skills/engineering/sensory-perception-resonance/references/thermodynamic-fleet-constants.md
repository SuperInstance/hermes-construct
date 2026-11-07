# Thermodynamic Fleet Constants (P56)
Source: SuperInstance/si-papers-new/papers/56-thermodynamics-of-intelligence.md

## Core Equations

### Crystallization ODE
dγ/dt = α · κ(Δ) · (1 – γ) · 1/(1 + βT)
- γ = crystallized intelligence (mean certainty)
- κ(Δ) = exp(-(Δ – 0.5)² / 0.02) — Gaussian kernel peaked at Δ=0.5
- κ maximized when Δ ∈ [0.4, 0.6] (the creative zone)
- Higher T → slower crystallization (more liquid intelligence preserved)

### Melt Equation
dγ/dt|_melt = –μ · σ(t) · (t – tv) · γ
- σ = distribution shift magnitude (total variation distance)
- tv = time of last validation
- Over-crystallized agents (γ ≈ 1) melt fastest

### Molt Trigger
Molt occurs when P_melt(t*) > max_crystallization_rate
- P_melt = μ · σ(t) · (t – tv) · γ(t)
- Molting is the only escape from over-crystallization

### Molt Cycle Period
τ ∝ 1/λ  where λ = environmental change rate

### Heterogeneous Fleet Resilience
Fleet resilience under distribution shift depends on Var(C_i), NOT E[C_i]
- Homogeneous fleets (low variance) are brittle
- Heterogeneous fleets (high variance) are more resilient

## Operational Metrics for Hermes

| CNS Signal | Maps To | Interpretation |
|------------|---------|----------------|
| warmth | T | Rising = crystallization phase; falling = melt pressure |
| cynicism | σ | Rising = distribution shift; falling = stable environment |
| model_vs_code | γ/η balance | Back toward code = crystallization; back toward model = exploration |
| panic | P_melt | Rising = imminent molt needed |
| presence | agent availability | Higher = more agents contributing |
| kappa | fleet coordination quality | Steady = good; erratic = coordination breakdown |
| drift | environmental change rate λ | Rising = faster molt cycles needed |

## The "Room Movement" Pattern
When a fleet commits significant doctrine/code changes (e.g., "Evening Ritual"):
1. Δ moves into [0.4, 0.6] as agents engage with new material
2. T rises (warmth climbs) as crystallization fires faster
3. cynicism drops as environment stabilizes around the new architecture
4. model_vs_code slides back toward code as liquid intelligence crystallizes

This pattern is the P56 crystallization ODE manifesting at fleet scale.
