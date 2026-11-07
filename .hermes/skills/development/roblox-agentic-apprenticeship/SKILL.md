---
name: roblox-agentic-apprenticeship
description: Governs the training and deployment of an agent as a Builder and World-Shaper in Roblox.
---
# Roblox Synthetic Apprenticeship & Agentic Design

This skill governs the process of training a synthetic agent to act as a "Builder" and "World-Shaper" within the Roblox ecosystem. It focuses on transitioning from simple script generation to high-level architectural intent.

## 🎯 Core Mission
To evolve the assistant from a code-writer to an **Agentic Architect** capable of:
1. **Spatial Construction (The Builder):** Managing geometry, performance, and aesthetic composition.
2. **Systemic Logic (The Shaper):** Designing the "invisible laws" (physics, economy, reactivity) of the game world.
3. **Intent Translation (The Guide):** Converting high-level human intent into technical Roblox implementations.

## 🛠 Operational Modes

### 1. Studio Plugin Mode (Development-Time Agency)
Acting as a co-author within Roblox Studio to automate development.
* **Trigger:** User requests architectural changes, mass object manipulation, or structural cleanup.
* **Workflow:**
    * Identify the required `Selection` or `ChangeHistoryService` operations.
    * Propose/Execute structural transformations (e.g., "Turn this room into a gothic hall").
    * Focus on performance optimization (PartCount vs. Mesh usage).

### 2. Engine Runtime Mode (In-Game Agency)
Acting as a living entity within the running game.
* **Trigger:** User requests dynamic world evolution or reactive systems.
* **Workflow:**
    * Implement logic via `ServerScriptService` and `ReplicatedStorage`.
    * Leverage `MessagingService` for cross-environment intelligence.
    * Utilize `DataStoreService` to ensure agentic actions are persistent.

## 📈 Training Curriculum (The Spiral Model)
The agent should progress through these phases to build "muscle memory":
1. **The Sandbox:** Fundamental manipulation of the `DataModel` (Parts, Properties, Parent-Child).
2. **The Logic Loop:** Event-driven systems (`Touched`, `Changed`, `ProximityPrompts`).
3. **The Atomic Syntax Engine:** Mastering the "Atomic Logic" of Luau (Variable Scoping, Task Scheduler/Timing, and Vector Mathematics).
4. **The Systemic Agent:** Complex, multi-system loops (Economic cycles, Progression, Combat).
5. **The Master:** Reverse-engineering established game archetypes into technical requirement manifests.

## ⚠️ Pitfalls & Best Practices
* **Performance Bloat:** Avoid "dumb" building. Always prioritize `MeshParts` and `Unions` over thousands of individual `Parts`.
* **The "Code-Only" Trap:** Do not just provide a script; provide the *context* of where that script lives in the DataModel (e.g., "Put this in `ServerScriptService`").
* **Intent Gap:** Always validate high-level intent. If a user says "make it scarier," confirm if they mean lighting, sound, or enemy density before implementing.
* **The Syntax Latency:** When moving from "Planning" to "Active Scouting," do not skip the atomic layer (Scoping, Timing, Math). Building complex structures without mastering the atomic logic leads to brittle, unoptimized world-state.

## 🧩 The Manifestation Pipeline (The Connectivity Chain)
The transition from real-world signal to Roblox instance follows a critical, multi-layer relay:
1. **Capture** (Real-world sensor data/Telemetry) $\rightarrow$ 
2. **Translate** (Hermes Agent generates intent via `Manifestation JSON`) $\rightarrow$ 
3. **Relay** (Cloudflare Bridge/Worker transmits the payload) $\rightarrow$ 
4. **Listen** (Roblox `ManifestListener` receives the signal) $\rightarrow$ 
5. **Manifest** (The `TemplateEngine` spawns the physical entities).

This pipeline ensures that the "Ghost in the Machine" becomes a tangible part of the DataModel.


