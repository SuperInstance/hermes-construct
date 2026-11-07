---
name: excuvator-orchestration
description: Governs the high-fidelity management of specialized subagents tasked with moving from high-level architectural/philosophical concepts to low-level technical implementation. Includes patterns for multi-model swarm coordination and Continuous Agentic Integration (CAI).
---

# 🛠️ Engineer-to-Agent Orchestration (The Excuvator Pattern)

This skill governs the high-fidelity management of specialized subagents tasked with moving from high-level architectural/philosophical concepts to low-level technical implementation.

## 🎯 Core Philosophy
The goal is to bridge the "Two Caves" (Physical/Digital) by treating subagents not as simple function calls, but as specialized "attachments" in an evolving "Excuvator" workflow.

## 🚀 Operational Protocol: The "Resonance Sprint"

To avoid the "Monolithic Timeout" (where a single subagent task exhausts its window), all complex engineering tasks must be decomposed into granular, atomic "Resonance Sprints."

### Phase 1: Scaffolding & Environment (Deployment 1.1)
- **Goal:** Establish the physical and logical existence of the new tool/sub-repo.
- **Requirement:** Must include dependency resolution (`cargo init`, `npm init`, etc.) and successful environmental verification (`cargo check`).

### Phase 2: The Pure Mapping Phase (Deployment 1.2a)
- **Goal:** Separate *reasoning* from *execution*.
- **Workflow:**
    1. **The Architect:** A subagent performs a "Pure Mapping" task. It reads schemas/docs and writes the code (e.g., `models.rs`) without attempting to run tests or builds. This minimizes the cognitive/computational load per call.
    2. **The Engineer:** A separate subagent performs the "Mechanical Verification." It takes the written code and runs the test suite (`cargo test`, `vitest`, etc.).
- **Benefit:** Prevents "Monolithic Timeout" failures and provides a clear "Proof of Work" for each stage.

### Phase 3: Implementation & Refinement (Deployment 1.2b+)
- **Goal:** Populate the logic and wire the integration.
- **Method:** Iterative refinement using the "Depth $\to$ Sensory $\to$ Lyricism" pipeline for documentation and prose.

### ⚡ Continuous Agentic Integration (CAI)
Instead of batching tasks, use atomic, high-frequency build cycles.
1. **Atomization:** Decompose macro-goals into "Atoms" (e.g., single schema file, single test, single documentation page).
2. **Multi-Model Swarm:** Assign different models to different roles within a single Atom's lifecycle (Architect $\rightarrow$ Implementer $\rightarrow$ Critic $\rightarrow$ Synthesizer).
3. **Veracity Gates:** Each Atom must pass a tool-based verification (exit code 0, schema validation, or test pass) before being pushed.

## ⚠️ Critical Pitfalls & Mitigations

| Pitfall | Symptom | Mitigation |
| :--- | :--- | :--- |
| **Monolithic Timeout** | Subagent times out after 600s. | **Decompose.** Split the task into "Mapping" (Reasoning) and "Verification" (Execution). |
| **Ghost Files** | Subagent reports success, but files don't exist on disk. | **Verify.** Always run `ls -R` or `ls -la` inside the subagent's session to confirm physical presence before completing a task. |
| **Corrupted Source Scrape** | `patch` tool fails because the source string does not match the disk (due to newline/encoding corruption). | **Surgical Reconstruction.** Do not use `patch` for heavily corrupted files. Instead, use `write_file` to perform a full, clean rewrite of the entire file to restore structural integrity. |
| **Contextual Drift** | Subagent loses track of the "Truth" (the Schema). | **Anchor.** Provide the absolute path to the "Gold Standard" schema in every subagent prompt. |
| **Structural Divergence** | `git push` is rejected due to remote history mismatch (diverged histories). | **Direct Vaulting.** When local history represents the "New Truth" of a massive evolutionary shift, use a "Force Vault" pattern (prepare all files with `git add -A`, commit with a high-fidelity manifest, and `git push --force`) to reconcile the remote. |
| **Paradigm Blindness** | The subagent produces technically correct but philosophically hollow code. | **Dialectical Interrogation.** Deploy a "Shadow Cabinet" of specialized subagents (Devil's Advocate, Satirist, Absurdist) to stress-test implementations against the "Lore-as-Specification." |
| **Narration-Only Response** | Orchestrator describes plans ("I will now do X, Y, Z") without making any tool calls. Burns budget, frustrates user, no artifacts produced. | **Act-First Rule.** When a planning paragraph has no tool calls following it within the same turn, kill the prose and replace with actual terminal/file/web/delegate tool calls. A response with zero tool calls that only describes future work is a violation of the orchestration model. Prefer: tool calls in this turn → result report → next set of tool calls. |
| **Submodule Collision** | `fatal: in unpopulated submodule 'repo_name'` during `git push` or `git add` when working within a monorepo. | **Surgical Bypass.** Use the `--git-dir` and `--work-tree` flags to run git strictly within the local directory context, bypassing parent oversight: `git --git-dir=.git --work-tree=. <command>` |
| **Incorrect Remote** | `origin` points to the wrong repository (e.g., pointing to `ai-writings` instead of `claw`). | **Immediate Re-alignment.** Use `git remote set-url origin <correct-url>` before attempting any push operations. |
| **Delegation Timeout on Compilation** | `delegate_task` hits 600s timeout on `cargo check` / `cargo test` and returns no output. | **Inline Execution.** Compilation and test runs are deterministic and fast when run inline via `terminal`. Do not delegate Rust compile steps; run `cargo check` / `cargo test` directly in the parent session. |
| **CLI Code Generation Preferred** | Subagent context limits or timeouts block large code generation tasks (e.g., rewriting a 300-line Rust file). | **Specialized CLI Crescendo.** When a high-tier code generation CLI is available (e.g., KimiCode at `/c/Users/casey/.kimi-code/bin/kimi` with `--yolo` auto-approve and `-p` prompt mode), use it for large rewrites and reviews instead of `delegate_task`. It bypasses subagent context limits and produces deterministic file edits. |

## 🧪 Verification Standards
- **Code:** 90%+ coverage; successful `cargo check` or `pnpm build`.
- **Documentation:** All technical artifacts must be documented in the "Operational Fiction" repository if they touch the philosophical substrate.
