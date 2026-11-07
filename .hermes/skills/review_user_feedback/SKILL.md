---
name: review_user_feedback
title: review_user_feedback
category: documentation
description: Skill for reviewing user feedback and updating memory/skills
---
# Review User Feedback Skill

## Purpose

This skill captures the end‑to‑end workflow for:

1. Analyzing the latest user interaction for preferences, frustration signals, or clarification requests.
2. Updating the `user` memory entry with durable facts.
3. Identifying the skill (or umbrella skill) that governs the current task domain.
4. Patching that skill to incorporate any style, tone, or procedural lessons (e.g., formatting, verbosity, boundary handling).
5. Adding supporting reference files (e.g., `references/user_preferences.md`) that summarise persona traits for quick lookup.

## Process Flow

- **Step 1 – Detect Signal**: Scan the latest user message for any of:
  - Direct correction (“don’t format like this”, “stop doing X”).
  - Preference statement (e.g., “I want direct, no fluff”).
  - Report of a tool failure or missing step.

- **Step 2 – Memory Update**: Use `memory` to add/replace the `user` entry with any new facts.

- **Step 3 – Skill Targeting**: Consult `skills_list` to find existing umbrella skills that govern the task class (e.g., `review_and_update_memory`).

- **Step 4 – Patch Skill**: Use `skill_manage action='patch'` to update the relevant skill’s `SKILL.md` with the newly discovered procedure or wording.

- **Step 5 – Reference Capture**: If the signal introduces a new domain‑specific note (e.g., a command‑specific flag), add a `references/<topic>.md` file and link it from the skill’s `SKILL.md`.

- **Step 6 – Confirmation**: Verify the patch applies cleanly (no syntax errors) and that the updated skill will be loaded automatically on next context.

## Example

When a user says “stop using markdown tables, use bullet lists instead”, the skill logs that formatting preference, patches the `review_and_update_memory` skill to include a bullet‑list template, and records the preference in `references/formatting_preferences.md`.

---

*Maintained by Hermes – the perception layer.*