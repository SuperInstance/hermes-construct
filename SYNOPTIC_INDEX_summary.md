# Synoptic Index Reaction Summary - 2026-08-13

Fetched https://raw.githubusercontent.com/SuperInstance/claw/clean-publish/SYNOPTIC_INDEX.md

## Fragmentation Issues Identified

- **Gitlink/Submodule Confusion**: `claw` is tracked as 160000 gitlink in polln with no .gitmodules, causing fatal unpopulated submodule errors.
- **AI-writings scattering**: `ai-writings` repo exists but docs duplicated under docs/, papers/, hermes-* folders. Content audit needed to consolidate creative writing.
- **Bloated projects**: `spreadsheet-moment` contains node_modules and hardware-marketplace. polln root has 200+ top-level dirs including extraction artifacts, languages/i18n/translations duplicates.
- **No modular glue**: Repos communicate via docs only, no automated agentic glue. Need CNS-style bus or manifest registry.

## Next Steps Recommended

1. Stabilize git: remove gitlink or populate submodule correctly
2. Move ai-writing artifacts to ai-writings repo
3. Extract hardware-marketplace, claw-mobile, etc. to own repos
4. Build manifest index for inter-repo dependencies
5. Extract hardware-marketplace to own repo
6. Consolidate docs/ into repo-docs/
7. Move ai-writing artifacts from project folders to ai-writings repo
8. Create manifest.yaml for inter-repo dependencies with automated sync

Core repos audited: claw, spreadsheet-moment, constrainttheory, dodecet-encoder, claw-extensions, constrainttheory-ml-demo, ai-writings, SuperInstance-papers
