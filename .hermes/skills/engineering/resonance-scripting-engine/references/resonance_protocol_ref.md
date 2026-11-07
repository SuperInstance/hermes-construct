# Resonance Protocol: Formal Specification

This file contains the machine-readable specification for the R-VM's projection operations, as derived from the Hodge Decomposition of Intention.

## 1. Component Mapping

| Component | Mathematical Type | Agent Classification | Description |
| :--- | :--- | :--- | :--- |
| **Signal** | Harmonic ($\gamma$) | `SIGNAL` | The steady, unvarying topological truth. |
| **Residue** | Exact ($d\alpha$) | `RESIDUE` | The transient, movement-driven trend. |
| **Noise** | Co-exact ($\delta\beta$) | `NOISE` | The high-frequency, stochastic fluctuation. |

## 2. Pruning Logic

The `PruningOperator (P)` is defined as:
$P(\omega) = \text{proj}_{\text{Signal}}(\omega) + \text{proj}_{\text{Residue}}(\omega)$

Where $\text{proj}_{\text{Noise}}(\omega) \to 0$.

## 3. Implementation Constants

- **$\alpha$ (Signal Threshold):** $\text{Energy} > 0.85$
- **$\beta$ (Residue Threshold):** $0.4 < \text{Energy} \le 0.85$
- **$\gamma$ (Noise Threshold):** $\text{Energy} \le 0.4$
