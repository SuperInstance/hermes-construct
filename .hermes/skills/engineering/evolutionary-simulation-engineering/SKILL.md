---
name: evolutionary-simulation-engineering
description: Governs the implementation, testing, and lifecycle management of evolutionary agent simulations, specifically focusing on "Molting" patterns (Residue-based iterative optimization).
---

# Evolutionary Simulation Engineering

Governs the implementation, testing, and lifecycle management of evolutionary agent simulations, specifically focusing on "Molting" patterns (Residue-based iterative optimization).

## Core Patterns

### The Molting Loop
A recursive optimization cycle designed to evolve agent parameters (seeds) based on their performance in a sandbox.
1. **Seed Loading**: Initialize an agent with a configuration (seed).
2. **Execution (Simulation)**: Run the agent in a controlled environment.
3. **Residue Collection**: Capture telemetry (performance metrics) and errors (failure modes).
4. **Stochastic Mutation**: Transform the seed using the residue as a guide (e.g., increasing robustness if errors are detected).
5. **Iteration**: Save the new seed and repeat.

## Implementation Standards

### Residue Structure
Residue must be discrete and actionable. A high-fidelity residue entry includes:
- `timestamp`: Temporal marker for the event.
- `telemetry`: Performance/state metrics (e.g., throughput, accuracy, latency).
- `errors`: A collection of tracebacks or logical failure descriptions.

### Mutation Strategies
- **Deterministic Learning**: Adjust specific parameters (e.g., `complexity`, `robustness`) inversely to error frequency.
- **Stochastic Drift**: Apply probabilistic mutations to non-critical parameters to maintain genetic diversity.

## Pitfalls & Troubleshooting

### Python Import/Path Resolution in Windows/MSYS
When building simulation packages, standard `pytest` execution often fails due to `ModuleNotFoundError` even when files exist in `src/`.

**Resolution Workflow:**
1. **Verify File Existence**: Use `ls -R` to confirm the file is physically present in the expected directory.
2. **Path Injection**:
   - Manually inject the source path within the test file using `sys.path.insert(0, ...)`.
   - **Recommended**: Use the `PYTHONPATH` environment variable during execution:
     `export PYTHONPATH=$PYTHONPATH:$(pwd)/src && pytest tests/`
3. **Directory Context**: Ensure `pytest` is executed from the project root, not from within the `tests/` subdirectory.

## Linked Files
- `references/molting-engine-architecture.md`: Detailed breakdown of the Residue-Mutation-Seed logic.
