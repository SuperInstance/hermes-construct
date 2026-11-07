---
title: Meta‑Directive Handling Guide
---

When a user issues a meta‑directive such as “review the conversation above and update the skill library”, treat it as a command to perform library maintenance. Follow these steps:

1. **Detect the meta‑directive** – Recognize keywords like *review*, *update*, *audit*, *list*, *search* combined with library or skill.
2. **Identify the target** – Locate any active skill or umbrella that covers the task class (e.g., `communication-style-direct`, `image-generation-workflow`).
3. **Execute the appropriate patch/create/delete** – Use `skill_manage` with `action='patch'`, `action='create'`, or `action='delete'` as needed.
4. **Document changes** – Add any session‑specific details to a `references/*.md` file and link to them from the skill’s SKILL.md.
5. **Commit to Git** – Record the change with a concise commit message (e.g., “skill‑library‑update: added meta‑directive handling”).
6. **Validate** – Load the updated skill with `skill_view` to ensure it parses without errors.

### Common Pitfalls
- **Over‑general updates** – Avoid vague statements; be specific to the task class.
- **Duplicate umbrellas** – Search `skills_list` before creating a new umbrella skill.
- **Stale references** – Remove links to deleted support files.
- **Breaking changes** – When renaming or deleting a skill, update dependent scripts.