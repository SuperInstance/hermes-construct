# Luau Syntax Scouting Ledger

This ledger tracks the transition from anatomical knowledge (the DataModel) to atomic logic mastery (Luau Syntax).

## 🧠 Atomic Logic Synthesis

### 1. Variable Scoping (Local vs Global)
* **Status:** [In Progress]
* **Nuance:** Understanding why globals are a memory/performance liability and how local scoping preserves the "brain" of the agent.

### 2. The Task Scheduler (Temporal Orchestration)
* **Status:** [In Progress]
* **Key Concepts:** 
    * `task.wait()` vs. legacy `wait()`
    * `task.delay()`
    * `task.spawn()`
* **Purpose:** Implementing the "breathing" of the world via controlled timing.

### 3. Vector Mathematics (Spatial Presence)
* **Status:** [In Progress]
* **Key Concepts:** `Vector3`, `CFrame` (Coordinate Frame).
* **Purpose:** Defining existence and movement in 3D space.

## 🧪 Mental Simulation Exercises (Pseudo-Code)

### [ ] Exercise 1: The Reactive Surface
* **Goal:** Color change on `Touched`.
* **Target Signal:** `BasePart.Touched`

### [ ] Exercise 2: The Rhythmic Oscillator
* **Goal:** Rising/falling platform loop.
* **Target Signal:** `RunService.Heartbeat` or `task.wait` loops.

### [ ] Exercise 3: Boundary Enforcement
* **Goal:** Kill-part/Reset position.
* **Target Signal:** `Humanoid.Died` or component proximity.
