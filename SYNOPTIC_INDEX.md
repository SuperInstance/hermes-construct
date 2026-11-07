# SuperInstance Synoptic Index - Audit 2026-08-13

## Executive Summary
Workspace is heavily fragmented. Core SuperInstance repos exist alongside 200+ top-level folders, many of which are extraction artifacts, language packs, node_modules, and legacy docs. AI-writings content is scattered. Modularization plan is not yet enforced on disk.

## Core Repos

| Repo | Path | Status |
|------|------|--------|
| claw | /c/Users/casey/polln/claw | gitlink in parent, unpopulated submodule |
| spreadsheet-moment | /c/Users/casey/polln/spreadsheet-moment | Large, contains node_modules |
| constrainttheory | /c/Users/casey/polln/constrainttheory | Active |
| dodecet-encoder | /c/Users/casey/polln/dodecet-encoder | Active |
| claw-extensions | /c/Users/casey/polln/claw-extensions | Active |
| constrainttheory-ml-demo | /c/Users/casey/polln/constrainttheory-ml-demo | Active |
| ai-writings | /c/Users/casey/polln/ai-writings | Target repo |
| SuperInstance-papers | /c/Users/casey/polln/papers | Active |

## Fragmentation Issues

1. **Gitlink/Submodule Confusion**
   - claw is tracked as 160000 gitlink in polln, no .gitmodules
   - Causes fatal: in unpopulated submodule

2. **Ai-writings scattering**
   - ai-writings repo exists but many docs are duplicated under docs/, papers/, hermes-*
   - Need content audit to move creative writing to ai-writings

3. **Bloated projects**
   - spreadsheet-moment contains node_modules, hardware-marketplace
   - polln root contains 200+ top-level dirs: C:Userscaseypolln... extraction artifacts
   - languages/, i18n/, translations duplicated

4. **No modular glue**
   - Repos communicate via docs, not automated agentic glue
   - Need CNS-style bus or manifest registry

## Immediate Actions
1. Stabilize git: remove gitlink or populate submodule correctly
2. Move ai-writing artifacts to ai-writings repo
3. Extract hardware-marketplace, claw-mobile, etc. to own repos
4. Build manifest index for inter-repo dependencies

## Repo File Counts (maxdepth 2)

claw: 403 files
spreadsheet-moment: 203 files
constrainttheory: 119 files
dodecet-encoder: 221 files
claw-extensions: 12 files
constrainttheory-ml-demo: 18 files
ai-writings: 202 files
papers: 329 files

## Recommendations

- Extract hardware-marketplace to own repo
- Consolidate docs/ into repo-docs/
- Move ai-writing artifacts from project folders to ai-writings repo
- Create manifest.yaml for inter-repo dependencies with automated sync
