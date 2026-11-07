---
name: swarm-intent-modeling-reference
description: Reference documentation for Intent-based swarming, spatial partitioning, and collective agent behavior.
---

# REFERENCE: Swarm Intent Modeling

## 🎯 Intent-Based Swarming
Instead of individual per-agent decision making for high-level goals, we use **Intent-based broadcasting**.

### Propagation Flow
1.  **Broadcast**: `SwarmCoordinator.broadcastIntent(intent)` is called.
2.  **Async Delivery**: Intent is placed on the Message Bus using `task.defer` to prevent frame-time spikes.
3.  **Global Vector Calculation**: The Coordinator calculates a singular "Global Intent Vector" based on the current active intents.
4.  **Agent Integration**: Agents combine this high-level vector with their local Boid (Flocking) vectors.

## 🗺️ Spatial Partitioning (Grid-Based)
To maintain $O(N)$ complexity for neighbor detection in large swarms, we implement a static or dynamic spatial grid.

### Implementation Workflow
- **Grid Generation**: Divide the 3D workspace into uniform cubes (cells) of size $S$.
- **Registration**: As agents move, they register their `AgentID` into the corresponding `GridKey` (e.g., `"x,y,z"`).
- **Querying**: To find neighbors, an agent only queries its current cell and the 26 adjacent cells.

### Complexity Comparison
| Method | Complexity | Scalability |
| :--- | :--- | :--- |
| Brute Force | $O(N^2)$ | Very Low |
| Spatial Grid | $O(N \cdot k)$ | High |

*where $k$ is the average number of agents per local neighborhood.*
