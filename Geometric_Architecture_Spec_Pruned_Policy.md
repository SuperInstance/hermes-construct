# Geometric Architecture Spec: Pruned Policy Manifold

## 1. Executive Summary
This specification defines the formal mathematical framework for the **Pruned Policy**, conceptualizing a neural policy not as a static set of weights, but as a dynamic geometric manifold. The core objective is to utilize **Hodge Decomposition** to partition the policy's differential forms into constituent components: **Signal**, **Noise**, and **Harmonic Residue**, enabling precise pruning of non-contributory topological features.

## 2. Mathematical Foundation: The Policy Manifold
Let $\mathcal{M}$ be a smooth, compact $n$-dimensional Riemannian manifold representing the latent policy space. The policy $\pi$ is defined as a $k$-form $\omega \in \Omega^k(\mathcal{M})$, where $k$ represents the order of the features being modeled (e.g., $k=1$ for vector fields/directional gradients).

### 2.1 Metric Structure
The manifold is equipped with a Riemannian metric $g$, which induces:
- An inner product on $\Omega^k(\mathcal{M})$.
- The Hodge star operator $\star: \Omega^k(\mathcal{M}) \to \Omega^{n-k}(\mathcal{M})$.
- The codifferential $\delta: \Omega^k(\mathcal{M}) \to \Omega^{k-1}(\mathcal{M})$, defined as $\delta = (-1)^{n(k-1)+1} \star d \star$.

## 3. Hodge Decomposition of the Policy
The fundamental principle of the Pruned Policy is the unique orthogonal decomposition of any $k$-form $\omega$ (representing policy behavior/gradients) into three distinct subspaces:

$$\omega = d\alpha + \delta\beta + \gamma$$

Where:
1.  **$d\alpha$ (Exact Component / Signal):** The longitudinal component, where $\alpha \in \Omega^{k-1}(\mathcal{M})$. This represents the "potential-driven" signal—the direct, gradient-climbing path that drives purposeful behavior.
2.  **$\delta\beta$ (Co-exact Component / Noise):** The transverse component, where $\beta \in \Omega^{k+1}(\mathcal{M})$. This represents "curl-like" or rotational fluctuations. In the context of pruning, this component captures stochastic noise and high-frequency oscillations that do not contribute to the net directional progress.
3.  **$\gamma$ (Harmonic Component / Residue):** The component where $\Delta \gamma = 0$ (where $\Delta = d\delta + \delta d$ is the Hodge-Laplacian). This represents the topological "residue"—features that are closed and co-closed, existing as global properties of the manifold's topology (e.g., holes or non-trivial cycles).

## 4. The Pruning Mechanism
The "Pruning" process is defined as the projection of the policy $\omega$ onto the Signal subspace, effectively suppressing the Noise and minimizing the impact of the Harmonic Residue.

### 4.1 Pruning Operator $\mathcal{P}$
$$\mathcal{P}(\omega) \approx d\alpha$$

**Criteria for Pruning:**
- **Noise Suppression:** If $\|\delta\beta\| < \epsilon$, the transverse component is considered purely stochastic and is truncated.
- **Residue Regularization:** The harmonic component $\gamma$ is constrained to prevent the policy from becoming trapped in topological "dead zones" or non-trivial cycles that lead to limit cycles rather than convergence.

## 5. Architecture Specification

| Component | Mathematical Role | Functional Purpose |
| :--- | :--- | :--- |
| **Signal Subspace** | $\text{Im}(d)$ | Drives convergence and goal-directed behavior. |
| **Noise Subspace** | $\text{Im}(\delta)$ | Represents local fluctuations and exploration entropy. |
| **Harmonic Subspace** | $\text{Ker}(\Delta)$ | Encapsulates the global topology of the task space. |

## 6. Implementation Roadmap
1.  **Discretization:** Map the continuous manifold $\mathcal{M}$ to a discrete simplicial complex (e.g., via a Graph Neural Network architecture).
2.  **Laplacian Computation:** Construct the combinatorial Hodge-Laplacian $L_k$ for the $k$-th order simplices.
3.  **Spectral Decomposition:** Use spectral methods to isolate the kernels of $L_{k-1}$ and $L_{k+1}$ to perform the decomposition in discrete space.
4.  **Pruning Execution:** Apply a thresholding function to the co-exact component to prune high-frequency noise.
