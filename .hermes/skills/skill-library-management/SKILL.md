---
name: skill-library-management
description: Guide for updating the Hermes skill library in response to meta‑directives.
summary: Provides a repeatable workflow for detecting signals, selecting target skills, patching, adding support files, committing, and validating. Includes pitfalls and versioning guidance.
---

## Purpose
- Centralize process for skill updates triggered by user meta‑directives (e.g., "review and update the skill library").

## Workflow
1. **Signal Detection** – Identify style, workflow, or technique correction.
2. **Target Selection** – Find loaded skill or umbrella covering the task class.
3. **Update** – Patch existing skill or create new umbrella; add support files (`references/`, `templates/`, `scripts/`).
4. **Commit** – Use Git to record changes; optionally push to fleet wiki.
5. **Validate** – Load skill with `skill_view` to ensure it parses.

## Pitfalls
- Adding vague statements; keep updates specific to class of work.
- Creating duplicate umbrellas; search `skills_list` first.
- Stale references; remove links to deleted support files.
- Breaking changes; update dependent scripts.

## Example
When a user asks to "review the conversation above and update the skill library", treat it as a meta‑directive to execute a library‑maintenance task. Follow the workflow above, locate the relevant skill (e.g., `communication-style-direct`), patch it with new subsections, add any `references/` files, commit the changes, and verify with `skill_view`.