---
name: digital-artifact-playtest-audit
description: Governs the high-fidelity, multi-iteration auditing of digital artifacts (games, web apps, simulations) to ensure alignment with architectural intent, aesthetic coherence, and user experience standards.
---

# SKILL.md: Digital Artifact Play-Testing & UX Audit

Governs the high-fidelity, multi-iteration auditing of digital artifacts (games, web apps, simulations) to ensure alignment with architectural intent, aesthetic coherence, and user experience standards.

## Purpose
To move beyond superficial "it works" checks and perform deep-dive sensory and functional verification of interactive environments.

## Triggers
- User requests an "audit" or "review" of a playable environment (e.g., a game, a web demo, or a simulation).
- A new digital asset (like a Quilt-based game) is deployed and requires functional/aesthetic verification.
- The user asks for "play-test reviews" across multiple iterations.

## Workflow

### Phase 1: Environment Discovery & Shelling
1. **URL/Access Verification:** Identify the entry point (URL or local path).
2. **Visual/Tactile Mapping:** Perform an initial "look and feel" scan. Is the loading state clear? Is the branding consistent?
3. **Control Mapping:** Identify the primary input schemes (WASD, Mouse, Touch, etc.) and UI landmarks.

### Phase 2: The Multi-Pass Play-Test
Perform at least three distinct iterations to extract deep value:

#### Iteration 1: Functional & UX Baseline
- **Goal:** Is it playable and intuitive?
- **Checklist:**
    - **Visibility:** Are HUD elements (HP, Battery, Sensors) legible in the "active" state?
    - **Controls:** Do the on-screen prompts match the actual input responsivity?
    - **Navigation:** Are menus, dialogue boxes, and level gates functional?
    - **Clarity:** Is the primary goal of the current state (e.g., "Mine 5 iron") unmistakable?

#### Iteration 2: Technical & Integration Deep-Dive
- **Goal:** Does it respect the ecosystem's architecture?
- **Checklist:**
    - **Aesthetic Cohesion:** Does the "look and feel" match the subsystem (e.g., "Industrial/Junk" for Scrapcraft)?
    - **Input Latency/Feedback:** Does the UI react to inputs (clicks, movement) with appropriate visual/audio feedback?
    - **Integration:** Does the asset feel like part of the "Quilt" or "Superinstance" ecosystem, or does it feel "bolted on"?

#### Iteration 3: Synthesis & Final Verdict
- **Goal:** Provide a high-fidelity report.
- **Deliverable:** A multi-layered report categorized by:
    - **Status:** (e.g., READY, MONITOR, REWORK)
    - **Visual/Aesthetic Identity**
    - **UX/Legibility Findings**
    - **Functional/Technical Bugs**
    - **Integration/Ecosystem Fit**

## Pitfalls
- **The "Dialogue Blindness" Trap:** Large, centered dialogue boxes can obscure the gameplay loop. When auditing, ensure you clear all introductory/tutorial overlays to verify the *active* HUD.
- **Static Analysis Bias:** A screenshot of a dialogue box is NOT a screenshot of a gameplay loop. Always progress the state to the "active" phase before performing final HUD/Responsiveness audits.
- **Assuming Functionality:** A button labeled "Next" might be visually present but interactionally blocked by an invisible `onboarding-wizard` div. Always verify clickability.

## References
- `references/playtest-report-template.md` - Structure for delivering high-fidelity audit results.
